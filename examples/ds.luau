local evt,ctx=...
p=require"@antiraid/promise"
a=require"@antiraid/datastores"
b=a.new(ctx)
return p.yield(b.DummyStoreImpl.returnValue({1,2,3}))
