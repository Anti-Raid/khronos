use mlua_scheduler::taskmgr::SchedulerFeedback;

pub struct DummyFeedback;

impl SchedulerFeedback for DummyFeedback {
    fn on_response(
        &self,
        _label: &str,
        _th: &mlua::Thread,
        _result: Result<mlua::MultiValue, mlua::Error>,
    ) {
    }
}

