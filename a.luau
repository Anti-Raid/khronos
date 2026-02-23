local t = {}
local mt = {}

function mt.__eq(a, b)
    return false
end

setmetatable(t, mt)
table.find({ t, "value does not matter" }, t)
