#![allow(clippy::disallowed_methods)] // Allow RefCell borrow here

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::isolate::KhronosIsolate;
use super::runtime::{KhronosRuntime, OnBrokenFunc};

pub struct IsolateData<ExtraData: 'static> {
    pub isolate: KhronosIsolate,
    pub data: ExtraData
}

/// A simple abstraction around khronos runtime/isolates to allow named isolate access
///
/// Like isolates, these are also cheap to clone
pub struct KhronosRuntimeManager<ExtraData: 'static> {
    /// The runtime itself
    rt: KhronosRuntime,

    /// A map of name to sub-isolate
    sub_isolates: Rc<RefCell<std::collections::HashMap<String, Rc<IsolateData<ExtraData>>>>>,

    /// The main isolate (if any)
    main_isolate: Rc<RefCell<Option<IsolateData<ExtraData>>>>,

    /// A function to be called if the runtime is marked as broken
    on_broken: Rc<RefCell<Option<OnBrokenFunc>>>,
}

impl<ExtraData: 'static> Clone for KhronosRuntimeManager<ExtraData> {
    fn clone(&self) -> Self {
        Self {
            rt: self.rt.clone(),
            sub_isolates: self.sub_isolates.clone(),
            main_isolate: self.main_isolate.clone(),
            on_broken: self.on_broken.clone()
        }
    }
}

impl<ExtraData: 'static> KhronosRuntimeManager<ExtraData> {
    /// Creates a new runtime manager
    pub fn new(rt: KhronosRuntime) -> Self {
        if rt.has_on_broken() {
            panic!("Cannot create a runtime manager with a runtime that already has a on_broken callback set");
        }

        let m = Self {
            rt: rt.clone(),
            sub_isolates: Rc::new(RefCell::new(HashMap::new())),
            main_isolate: Rc::new(RefCell::new(None)),
            on_broken: Rc::new(RefCell::new(None)),
        };

        let m_ref = m.clone();

        // Ensure to clear out the isolates when the runtime is broken
        rt.set_on_broken(Box::new(move || {
            m_ref.main_isolate.borrow_mut().take();
            m_ref.clear_sub_isolates();

            let Some(on_broken) = m_ref.on_broken.borrow_mut().take() else {
                return;
            };

            on_broken();
        }));

        m
    }

    /// Sets the main isolate
    pub fn set_main_isolate(&self, isolate: IsolateData<ExtraData>) {
        self.main_isolate.borrow_mut().replace(isolate);
    }

    /// Returns the runtime
    pub fn runtime(&self) -> &KhronosRuntime {
        &self.rt
    }

    /// Returns the main isolate (if any)
    pub fn main_isolate(&self) -> std::cell::Ref<Option<IsolateData<ExtraData>>> {
        self.main_isolate.borrow()
    }

    /// Returns a sub-isolate by name
    pub fn get_sub_isolate(&self, name: &str) -> Option<Rc<IsolateData<ExtraData>>> {
        self.sub_isolates.borrow().get(name).cloned()
    }

    /// Adds a sub-isolate by name
    pub fn add_sub_isolate(&self, name: String, isolate: IsolateData<ExtraData>) {
        self.sub_isolates.borrow_mut().insert(name, isolate.into());
    }

    /// Removes a sub-isolate by name
    pub fn remove_sub_isolate(&self, name: &str) -> Option<Rc<IsolateData<ExtraData>>> {
        self.sub_isolates.borrow_mut().remove(name)
    }

    /// Clears all sub-isolates
    pub fn clear_sub_isolates(&self) {
        self.sub_isolates.borrow_mut().clear();
    }

    /// Returns the hashmap of sub-isolates
    pub fn sub_isolates(&self) -> std::collections::HashMap<String, Rc<IsolateData<ExtraData>>> {
        self.sub_isolates.borrow().clone()
    }

    /// Returns if a on_broken callback is set
    pub fn has_on_broken(&self) -> bool {
        self.on_broken.borrow().is_some()
    }

    /// Registers a callback to be called when the runtime is marked as broken
    pub fn set_on_broken(&self, callback: OnBrokenFunc) {
        self.on_broken.borrow_mut().replace(callback);
    }
}
