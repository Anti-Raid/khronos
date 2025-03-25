use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::isolate::KhronosIsolate;
use super::runtime::{KhronosRuntime, OnBrokenFunc};
use crate::utils::assets::AssetManager as AssetManagerTrait;

/// A simple abstraction around khronos runtime/isolates to allow named isolate access
///
/// Like isolates, these are also cheap to clone
#[derive(Clone)]
pub struct KhronosRuntimeManager<T: AssetManagerTrait + Clone + 'static> {
    /// The runtime itself
    rt: KhronosRuntime,

    /// A map of name to sub-isolate
    sub_isolates: Rc<RefCell<std::collections::HashMap<String, KhronosIsolate<T>>>>,

    /// The main isolate (if any)
    main_isolate: Rc<RefCell<Option<KhronosIsolate<T>>>>,

    /// A function to be called if the runtime is marked as broken
    on_broken: Rc<RefCell<Option<OnBrokenFunc>>>,
}

impl<T: AssetManagerTrait + Clone + 'static> KhronosRuntimeManager<T> {
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
        rt.set_on_broken(Box::new(move |_lua| {
            m_ref.main_isolate.borrow_mut().take();
            m_ref.clear_sub_isolates();

            let Some(on_broken) = m_ref.on_broken.borrow_mut().take() else {
                return;
            };

            on_broken(_lua);
        }));

        m
    }

    /// Sets the main isolate
    pub fn set_main_isolate(&self, isolate: KhronosIsolate<T>) {
        self.main_isolate.borrow_mut().replace(isolate);
    }

    /// Returns the runtime
    pub fn runtime(&self) -> &KhronosRuntime {
        &self.rt
    }

    /// Returns the main isolate (if any)
    pub fn main_isolate(&self) -> Option<KhronosIsolate<T>> {
        self.main_isolate.borrow().clone()
    }

    /// Returns a sub-isolate by name
    pub fn get_sub_isolate(&self, name: &str) -> Option<KhronosIsolate<T>> {
        self.sub_isolates.borrow().get(name).cloned()
    }

    /// Adds a sub-isolate by name
    pub fn add_sub_isolate(&self, name: String, isolate: KhronosIsolate<T>) {
        self.sub_isolates.borrow_mut().insert(name, isolate);
    }

    /// Removes a sub-isolate by name
    pub fn remove_sub_isolate(&self, name: &str) -> Option<KhronosIsolate<T>> {
        self.sub_isolates.borrow_mut().remove(name)
    }

    /// Clears all sub-isolates
    pub fn clear_sub_isolates(&self) {
        self.sub_isolates.borrow_mut().clear();
    }

    /// Returns the hashmap of sub-isolates
    pub fn sub_isolates(&self) -> std::collections::HashMap<String, KhronosIsolate<T>> {
        self.sub_isolates.borrow().clone()
    }

    /// Clears the bytecode cache of all isolates
    pub fn clear_bytecode_cache(&self) {
        for (_, isolate) in self.sub_isolates.borrow().iter() {
            isolate.bytecode_cache().clear_bytecode_cache();
        }

        if let Some(ref main_isolate) = *self.main_isolate.borrow() {
            main_isolate.bytecode_cache().clear_bytecode_cache();
        }
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
