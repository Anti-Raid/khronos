// May happen or may not, this file is a dummy till then
const MULTIDISPATCHER: &str = r#"
local eventStream, scriptexec, pluginreg = ...
local pluginregreqcache = {}
local reqcache = {}
local scriptEnvs = {}
while true do
   local scripts, event = eventStream:next()
   for script in scripts do
       env = nil
       if scriptEnvs[script.name] then 
           env = scriptEnvs[script.name]
       else
           env = {
              require = function(script)
                if not path then
                    error("missing argument #1 to 'require' (string expected)")
                end

                -- Fast path
                if pluginreg[path] then
                    if pluginregreqcache[path] then
                        return pluginregreqcache[path]
                    end
                    local plugin = pluginreg[path]:load(...)
                    pluginregreqcache[path] = plugin
                    return plugin
                end

                local debugname = debug.info(2, "s")
                luacall(coroutine.running(), string.sub(debugname, 10, -3), path, ...)
                return coroutine.yield()
              end, 
           print = <custom print code here>, stdout = {}}
           scriptEnvs[script.name] = env
       end 

       local ok, res = pcall(scriptexec, script, script.context, event)
       if ok then script:ok(res) else script:err(res) end
   end
end
"#