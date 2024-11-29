function HorizontalRule(_) -- luacheck: ignore 131
    return pandoc.Para({
        pandoc.Str("&#x002a;"), pandoc.Space(), pandoc.Str("&#x002a;"), pandoc.Space(),
        pandoc.Str("&#x002a;"),
    })
end

function Str(elem) -- luacheck: ignore 131
    return pandoc.Str(string.gsub(string.gsub(elem.text, "<", "&lt;"), ">", "&gt;"))
end
