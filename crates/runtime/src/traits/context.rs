use crate::traits::runtimeprovider::RuntimeProvider;
use dapi::controller::DiscordProvider;

pub trait KhronosContext: 'static + Clone + Sized {
    type DiscordProvider: DiscordProvider;
    type RuntimeProvider: RuntimeProvider;

    /// Returns a Discord provider
    ///
    /// This is used to interact with Discord API
    fn discord_provider(&self) -> Option<Self::DiscordProvider>;
    
    /// Returns a runtime provider
    fn runtime_provider(&self) -> Option<Self::RuntimeProvider>;

    /// Returns the contexts memory limit, if any
    fn memory_limit(&self) -> Option<usize> {
        None
    }
}
