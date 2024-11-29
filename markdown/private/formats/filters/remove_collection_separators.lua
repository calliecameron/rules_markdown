function Div(elem) -- luacheck: ignore 131
    for _, class in ipairs(elem.classes) do
        if class == "collectionseparator" then
            return {}
        end
    end
end
