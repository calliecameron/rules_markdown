function validate_raw(str)
    if str:find("'") or str:find("\"") then
        io.stderr:write("Found quotes that weren't converted to smart quotes. Replace them with " ..
                            "backslash-escaped literal curly quotes (“ ” ‘ ’).\n")
        os.exit(1)
    end
end

function validate(elem)
    validate_raw(elem.text)
end

function Pandoc(doc) -- luacheck: ignore 131
    doc.blocks:walk({Str = validate})
    local title = doc.meta["title"]
    if title then
        local title_type = pandoc.utils.type(title)
        if title_type == "Inlines" or title_type == "Blocks" then
            title:walk({Str = validate})
        elseif title_type == "string" then
            validate_raw(title)
        end
    end
end
