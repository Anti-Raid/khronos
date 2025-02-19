local pyield = require"@antiraid/promise".yield
task.delay(5, function() print("Hi") end)
cli.ext.print("Here")
pcall(pyield, cli.ext.input("A: "))
