local increment_included_headers = false

function increment(elem)
    elem.level = elem.level + 1
    return elem
end

function include(elem)
    if #elem.content == 3 and elem.content[1].tag == "Str" and elem.content[1].text == "!include" and
        elem.content[2].tag == "Space" and elem.content[3].tag == "Str" then
        local path = elem.content[3].text
        local includedFile = io.open(path, "r")
        if includedFile == nil then
            io.stderr:write("Failed to open included file: " .. path .. "\n")
            os.exit(1)
        end
        local content = includedFile:read("*all")
        includedFile:close()
        local doc = pandoc.read(content, "json")
        if increment_included_headers then
            return pandoc.walk_block(pandoc.Div(doc.blocks), {Header = increment}).content
        else
            return doc.blocks
        end
    end
end

function get_increment(meta)
    if meta["increment-included-headers"] then
        increment_included_headers = true
        meta["increment-included-headers"] = nil
    end
    return meta
end

return {{Meta = get_increment}, {Para = include}}
