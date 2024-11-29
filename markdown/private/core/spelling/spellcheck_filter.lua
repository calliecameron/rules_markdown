function has_nospellcheck(elem)
    for _, class in ipairs(elem.classes) do
        if class == "nospellcheck" then
            return true
        end
    end
    return false
end

function forbid_nospellcheck(elem)
    if has_nospellcheck(elem) then
        io.stderr:write(
            "The 'nospellcheck' class is only allowed on divs and span; found on type '" .. elem.tag ..
                "'\n")
        os.exit(1)
    end
end

function CodeBlock(elem) -- luacheck: ignore 131
    forbid_nospellcheck(elem)
    return {}
end

function Div(elem) -- luacheck: ignore 131
    if has_nospellcheck(elem) then
        return {}
    end
    return elem.content
end

function Figure(elem) -- luacheck: ignore 131
    forbid_nospellcheck(elem)
    return pandoc.Figure(elem.content, elem.caption, {})
end

function Header(elem) -- luacheck: ignore 131
    forbid_nospellcheck(elem)
    return pandoc.Header(elem.level, elem.content, {})
end

function Para(elem) -- luacheck: ignore 131
    if #elem.content == 3 and elem.content[1].tag == "Str" and elem.content[1].text == "!include" and
        elem.content[2].tag == "Space" and elem.content[3].tag == "Str" then
        return {}
    end
    return nil
end

function Table(elem) -- luacheck: ignore 131
    forbid_nospellcheck(elem)
end

function Code(elem) -- luacheck: ignore 131
    forbid_nospellcheck(elem)
    return {}
end

function Image(elem) -- luacheck: ignore 131
    forbid_nospellcheck(elem)
    return pandoc.Image(elem.caption, "", elem.title, {})
end

function Link(elem) -- luacheck: ignore 131
    forbid_nospellcheck(elem)
    if pandoc.utils.stringify(elem.content) == elem.target then
        return {}
    end
    return pandoc.Link(elem.content, "", elem.title, {})
end

function SmallCaps(elem) -- luacheck: ignore 131
    return elem.content
end

function Span(elem) -- luacheck: ignore 131
    if has_nospellcheck(elem) then
        return {}
    end
    return elem.content
end
