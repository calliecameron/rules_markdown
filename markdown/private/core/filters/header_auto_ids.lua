local header_ids = {}

function Header(elem) -- luacheck: ignore 131
    if not header_ids[elem.level] then
        header_ids[elem.level] = 1
    end
    if #elem.identifier == 0 or elem.identifier:sub(1, 2) == "__" then
        elem.identifier = "__h" .. elem.level .. "_" .. header_ids[elem.level]
    end
    header_ids[elem.level] = header_ids[elem.level] + 1
    return elem
end
