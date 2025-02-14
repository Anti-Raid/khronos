--!nocheck
local cli = _G.cli
local httpserver = cli.http_server
local pyield = require"@antiraid/promise".yield
local port = tonumber(pyield(cli.ext.input("Enter port number to listen on: ")))
if not port then
    error("Invalid port number")
end

local addr = pyield(cli.ext.input("Enter address to listen on: "))

if not addr or addr == "" then 
    addr = "localhost"
end

local router = httpserver.new_router(addr, port)

local i = 0;
router:get("/", function(req)
    i = i + 1
    return "Hello, world! " .. tostring(i)
end)

router:post("/json", function(req)
    local json = pyield(req.body:json())
    if not json.name then
        return httpserver.jsonresponse(400, {
            error = "Missing `name` field"
        })
    end
    if not json.age then
        return httpserver.jsonresponse(400, {
            error = "Missing `age` field"
        })
    end
    return "Hello, " .. json.name .. "! You are " .. json.age .. " years old."
end)

router:get("/stop", function(req)
    router:stop()
    return "Server stopped"
end)

task.spawn(function()
    local ok, err = pcall(pyield, router:serve())

    if not ok then
        print("Failed to start server: " .. tostring(err))
    end
end)

print("Server started on " .. tostring(router.bind_addr))

while true do 
    task.wait(1)
    if not router:is_running() then
        task.wait(1)
        break
    end
end
