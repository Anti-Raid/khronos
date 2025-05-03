/// A EventProvider provides the basic way for scripts to receive events
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait EventProvider: 'static + Clone {
    /// Recieves the next event the script's context should recieve
    async fn recv_next_event(&self) -> Result<crate::primitives::event::Event, crate::Error>;
}