function Div(elem) -- luacheck: ignore 131
    for _, class in ipairs(elem.classes) do
        if class == "nospellcheck" then
            return elem.content
        end
    end
    return nil
end

function Span(elem) -- luacheck: ignore 131
    for _, class in ipairs(elem.classes) do
        if class == "nospellcheck" then
            return elem.content
        end
    end
    return nil
end
