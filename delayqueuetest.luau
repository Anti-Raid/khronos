local channel = require "@antiraid/channel" :: any
local time = require "@antiraid/datetime" :: any
local delayqueue = channel.DelayChannel()
task.delay(2, function()
    print("Adding to delay queue")
    delayqueue:add("hello world", time.timedelta_seconds(2))
end)
local v = delayqueue:next()
print(v)