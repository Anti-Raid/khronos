pub struct ThreadLimiter {
    thread_limit: usize,
    threads: std::cell::Cell<usize>,
}

impl ThreadLimiter {
    pub fn new(thread_limit: usize) -> Self {
        Self {
            thread_limit,
            threads: std::cell::Cell::new(0),
        }
    }

    /// Returns the current number of threads recorded
    /// by the thread limiter.
    pub fn current_threads(&self) -> usize {
        self.threads.get()
    }

    /// Returns the thread limit set for this thread limiter.
    pub fn thread_limit(&self) -> usize {
        self.thread_limit
    }

    fn on_thread_add_impl(&self) -> mlua::Result<()> {
        let threads = self.threads.get();
        if threads >= self.thread_limit {
            return Err(mlua::Error::external("Thread limit reached"));
        }

        self.threads.set(threads + 1);

        Ok(())
    }
}

impl mlua_scheduler::taskmgr::SchedulerFeedback for ThreadLimiter {
    fn on_thread_add(
        &self,
        _label: &str,
        _creator: &mlua::Thread,
        _thread: &mlua::Thread,
    ) -> mlua::Result<()> {
        self.on_thread_add_impl()
    }
}

impl mlua_scheduler_ext::feedbacks::ThreadAddMiddleware for ThreadLimiter {
    fn on_thread_add(
        &self,
        _label: &str,
        _creator: &mlua::Thread,
        _thread: &mlua::Thread,
    ) -> mlua::Result<()> {
        self.on_thread_add_impl()
    }
}
