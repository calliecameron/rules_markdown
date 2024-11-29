function Div(elem) -- luacheck: ignore 131
    for _, class in ipairs(elem.classes) do
        if class == "firstparagraph" or class == "otherparagraph" or class == "blankline" then
            return elem.content
        end
    end
end
