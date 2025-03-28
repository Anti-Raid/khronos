-- Create docs using luau-docgen
local fs = require"@lune/fs"
local process = require"@lune/process"

if process.args[1] == nil then 
    print("Usage: createdocs.luau <save path folder>")
    return
end

if not fs.isDir(process.args[1]) then 
    print("Invalid path: "..process.args[1].." (not a directory or does not exist)")
    return
end

local istatemod = require("./luau-docgen/documentor_core/istate")
local documentor = require("./luau-docgen/documentor_core/documentor")
local getrt = require("./luau-docgen/documentor_core/rtenhance/getrt")

local globals = getrt()

if globals.runtime == "UnknownRt" then 
    print("Warning: Unknown runtime used, filesystem operations may not be available") 
end

-- Module resolution plugin
local modresolveplugin: istatemod.OptionalPlugin = {
    plugin_header = istatemod.PLUGIN_HEADER,
    resolveModuleToUrl = function(fieldtype: string, module: string, type: string): {
        moduleUrl: string,
        typeUrl: string,
    } 
        local modfilename = string.lower(module):gsub("/", "_")..".md"

        return {
            moduleUrl = `./{modfilename}`,
            typeUrl = `./{modfilename}#{type}`,
        }  
    end
}

local plugin = istatemod.ChainPlugin.new({modresolveplugin, istatemod.DefaultPlugin})

for i, docfile in fs.readDir("types/@antiraid") do 
    print("Creating docs for "..docfile)

    -- module name is @antiraid + filename (without .luau)
    local filename = docfile:sub(1, -6)
    local modname = "@antiraid/"..filename

    globals.fs_ops.fs_writefile = function(path: string, content: string) 
        if path ~= "@virtualpath" then error("Invalid path: "..path) end
        fs.writeFile(process.args[1].."/"..filename..".md", content)
    end

    (globals :: any).documentor_args = {"types/@antiraid/"..docfile, "--output", "@virtualpath", "--modname", modname}

    documentor(globals, function(istate: istatemod.IState) 
        istate.plugin = plugin
    end)
end