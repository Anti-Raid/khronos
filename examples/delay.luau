local pyield = require"@antiraid/promise".yield
task.delay(5, function() print("Hi") end)
print("Here")
pcall(pyield, cli.ext.input("A: "))
