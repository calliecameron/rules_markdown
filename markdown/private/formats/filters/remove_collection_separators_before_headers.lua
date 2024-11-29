local next_id = 1
local ids_to_remove = {}
local latest_separator = nil

function annotate(elem)
    for _, class in ipairs(elem.classes) do
        if class == "collectionseparator" then
            elem.identifier = "__collectionseparator_" .. next_id
            next_id = next_id + 1
        end
    end
    return elem
end

function mark_for_removal(elem)
    if elem.tag == "Div" then
        for _, class in ipairs(elem.classes) do
            if class == "collectionseparator" then
                latest_separator = elem.identifier
                return nil, false
            end
        end
        return
    end

    if elem.tag == "Header" and latest_separator then
        ids_to_remove[latest_separator] = true
    end
    latest_separator = nil
end

function remove(elem)
    for _, class in ipairs(elem.classes) do
        if class == "collectionseparator" then
            if ids_to_remove[elem.identifier] then
                return {}
            end
            return elem.content
        end
    end
end

return {{Div = annotate}, {traverse = "topdown", Block = mark_for_removal}, {Div = remove}}
