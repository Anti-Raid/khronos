local memusage=require"@antiraid/interop".memusage
local promise = require"@antiraid/promise"
local kv = require"@antiraid/kv"
local ctx = ...
local count = 0
for i = 1,100 do 
	promise.yield(kv.new(ctx):set("test"..i, i))
	task.spawn(function() print(memusage()); count+=1 end)
end

print("Waiting for threads to finish")

while true do 
	if count == 100 then break end
	coroutine.yield() 
end
