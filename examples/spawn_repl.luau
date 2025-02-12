print("Spawning repl after setting some globals!")
foo = "you found me"

-- Ask CLI to spin up a repl
cli.ext.request_next_entrypoint({ Repl = { task_wait_mode = "WaitAfterExecution" } })

-- Because request_next_entrypoint only gets honored after the current script finishes,
-- the below code also executes as well
bar = "you found me too"
