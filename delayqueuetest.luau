local channel = require "@antiraid/channel" :: any
local time = require "@antiraid/datetime" :: any
local delayqueue = channel.DelayChannel()
local now = nil
task.delay(2, function()
    now = os.clock()
    print("Adding to delay queue")
    delayqueue:add("hello world 2", time.timedelta_seconds(5))
    delayqueue:add("hello world 1", time.timedelta_seconds(2))
    local ok, err = pcall(delayqueue.clear, delayqueue)
    if not ok then 
        print("Failed to clear delay queue:", ok, err)
    end
    if #delayqueue ~= 0 then
        print("Expected 0 item in delay queue after clear, got "..#delayqueue)
    end
    print("Cleared delay queue, adding again")
    delayqueue:add("hello world 2", time.timedelta_seconds(5))
    delayqueue:add("hello world 1", time.timedelta_seconds(2))
    print("Delay queue has ", #delayqueue, " items")
end)
print("Waiting for value...")
local v = delayqueue:next()
assert(now ~= nil, "task.delay did not run?")
print(v, " after ", os.clock() - now, " seconds")
print("Waiting for next value...")
local v2 = delayqueue:next()
print(v2, " after ", os.clock() - now, " seconds")
assert(#delayqueue == 0, "Expected 0 item in delay queue, got "..#delayqueue)