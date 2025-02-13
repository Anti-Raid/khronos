--!nocheck
local cli = _G.cli
local pyield = require"@antiraid/promise".yield
local port = tonumber(pyield(cli.ext.input("Enter port number to listen on: ")))
if not port then
    error("Invalid port number")
end

local addr = pyield(cli.ext.input("Enter address to listen on: "))

if not addr or addr == "" then 
    addr = "localhost"
end

local router = cli.http_server.new_router(addr, port)

local i = 0;
router:get("/", function(req)
    i = i + 1
    return "Hello, world! " .. tostring(i)
end)

router:get("/stop", function(req)
    router:stop()
    return "Server stopped"
end)

task.spawn(function()
    pyield(router:serve())
end)

print("Server started on " .. addr .. ":" .. port)

while true do 
    task.wait(1)
    if not router:is_running() then
        task.wait(1)
        break
    end
    print("hi")
end
