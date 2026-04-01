use crate::traits::runtimeprovider::RuntimeProvider;

pub trait KhronosContext: 'static + Clone + Sized {
    type RuntimeProvider: RuntimeProvider;
    
    /// Returns a runtime provider
    fn runtime_provider(&self) -> Option<Self::RuntimeProvider>;

    /// Returns the contexts memory limit, if any
    fn memory_limit(&self) -> Option<usize> {
        None
    }
}
