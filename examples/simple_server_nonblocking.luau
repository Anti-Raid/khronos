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
    local ok, _ = pcall(pyield, router:serve())

    if not ok then
        cli.ext.print("Server stopped")
    end
end)

cli.ext.print("Server started on " .. tostring(router.bind_addr))

while true do 
    task.wait(1)
    if not router:is_running() then
        task.wait(1)
        break
    end
    cli.ext.print("Stopped server")
end
