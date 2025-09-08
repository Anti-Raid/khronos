--task.delay(60, function() print("HI") end)
print("World")
while true do 
	task.wait(0.1)
end
