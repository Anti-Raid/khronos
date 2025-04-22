local evt,ctx=...
local p=require"@antiraid/promise"
local a=require"@antiraid/datastores"
local dt=require"@antiraid/datetime"
local b=a.new(ctx)
print(p.yield(b.CopyDataStore.copy({1,2,3})))
print(tostring(p.yield(b.CopyDataStore.copy(dt.new("IST")))))
print(p.yield(b.CopyDataStore.copy(dt.UTC:now())))
print(p.yield(b.CopyDataStore.copy("multi", "value")))
