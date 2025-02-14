--!nocheck
local cli = _G.cli
local httpserver = cli.http_server
local pyield = require"@antiraid/promise".yield
local null = require"@antiraid/interop".null
local kv = require"@antiraid/kv"
local _evt, ctx = ...

local port = tonumber(pyield(cli.ext.input("Enter port number to listen on: ")))
if not port then
    error("Invalid port number")
end

local addr = pyield(cli.ext.input("Enter address to listen on: "))

if not addr or addr == "" then 
    addr = "localhost"
end

local kvexecutor = kv.new(ctx)
local router = httpserver.new_router(addr, port)

local i = 0;
router:get("/", function(req)
    i = i + 1
    return "Hello, world! " .. tostring(i)
end)

router:get("/echo/{statement}", function(req)
    local params = req:path()
    return "Echo: " .. tostring(params.statement)
end)

router:post("/get", function(req)
    local ok, json = pcall(pyield, req.body:json())

    if not ok then
        return httpserver.jsonresponse(400, {
            error = "Invalid JSON"
        })
    end

    if not json.key then
        return httpserver.jsonresponse(400, {
            error = "Missing `key` field"
        })
    end
    local value = pyield(kvexecutor:get(json.key))
    if value == null or value == nil then value = null end
    return httpserver.jsonresponse(200, {
        value = value or nil
    })
end)

router:post("/set", function(req)
    local ok, json = pcall(pyield, req.body:json())

    if not ok then
        return httpserver.jsonresponse(400, {
            error = "Invalid JSON"
        })
    end

    if not json.key then
        return httpserver.jsonresponse(400, {
            error = "Missing `key` field"
        }, httpserver.headers({
            ["X-Missing-Field"] = "key"
        }))
    end

    if not json.value then
        return httpserver.jsonresponse(400, {
            error = "Missing `value` field"
        }, httpserver.headers({
            ["X-Missing-Field"] = "value"
        }))
    end

    local value = pyield(kvexecutor:set(json.key, json.value))
    return httpserver.jsonresponse(200, {
        value = value
    })
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