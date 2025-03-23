pub struct ThreadLimiter {
    pub thread_limit: usize,
    pub threads: std::cell::RefCell<usize>,
}

impl mlua_scheduler_ext::feedbacks::ThreadAddMiddleware for ThreadLimiter {
    fn on_thread_add(
        &self,
        _label: &str,
        _creator: &mlua::Thread,
        _thread: &mlua::Thread,
    ) -> mlua::Result<()> {
        let mut threads = self.threads.borrow_mut();
        if *threads >= self.thread_limit {
            return Err(mlua::Error::external("Thread limit reached"));
        }

        *threads += 1;

        Ok(())
    }
}
