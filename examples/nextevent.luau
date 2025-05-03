local pyield = require"@antiraid/promise".yield
local evt,ctx = ...
local kv = require"@antiraid/kv"
local memusage = require"@antiraid/interop".memusage
local ds = require"@antiraid/datastores"

task.spawn(function() 
	while true do
		task.wait(2)
		print("Sending next event")
		local ok, err = xpcall(pushevent, debug.traceback, {
  			inner = {
				base_name = "AntiRaid",
  				data = {},
  				name = "OnStartup",
			}
		})
		
		if not ok then
			print("Error sending next event: ", err)
		end
	end
end)

local function evtHandler(evt) 
	task.wait(0.1)
	print(evt)
	task.wait(2)
	pcall(pyield, kv.new(ctx):set("foo", evt))
	print(pcall(pyield, ds.new(ctx).CopyDataStore.copy({a=1})))
	print("Done processing event with current memusage of " .. memusage())
end

while true do 
	print("Waiting for next event")
	local evt = pyield(ctx:nextevent())
	task.spawn(evtHandler, evt)
end
