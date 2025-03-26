-- Create docs using luau-docgen
local fs = require"@lune/fs"
local process = require"@lune/process"

if process.args[1] == nil then 
    print("Usage: createdocs.luau <save path>")
    return
end

local documentor = require("./luau-docgen/documentor_core/documentor")
local getrt = require("./luau-docgen/documentor_core/rtenhance/getrt")

local globals = getrt()

if globals.runtime == "UnknownRt" then 
    print("Warning: Unknown runtime used, filesystem operations may not be available") 
end

type Fragment = {
    module: string,
    content: string
}

local fragments: {Fragment} = {}
for i, docfile in fs.readDir("types/@antiraid") do 
    print("Creating docs for "..docfile)

    -- module name is @antiraid + filename (without .luau)
    local modname = "@antiraid"..docfile:sub(1, -6)

    globals.fs_ops.fs_writefile = function(path: string, content: string) 
        if path ~= "@virtualpath" then error("Invalid path: "..path) end
        fragments[#fragments + 1] = {module = modname, content = content}
    end

    (globals :: any).documentor_args = {"types/@antiraid/"..docfile, "--output", "@virtualpath", "--modname", "@antiraid/"..docfile}

    documentor(globals)
end

-- Sort fragments by module name
table.sort(fragments, function(a: {Fragment}, b: {Fragment}) 
    return a.module < b.module
end)

-- Convert to a {string}
for i, fragment in ipairs(fragments) do 
    fragments[i] = fragment.content
end

-- Write fragments to output file
fs.writeFile(process.args[1], table.concat(fragments, "\n\n---\n\n"))
