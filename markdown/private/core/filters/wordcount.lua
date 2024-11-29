-- modified from https://pandoc.org/lua-filters.html#examples
local words = 0

local wordcount = {
    Str = function(elem)
        -- only count words that contain at least one non-punctuation character
        if elem.text:match("[^%p\u{A0}“”‘’–—…]") then
            words = words + 1
        end
    end,

    CodeBlock = function(elem)
        local _, n = elem.text:gsub("%S+", "")
        words = words + n
    end,

    Code = function(elem)
        local _, n = elem.text:gsub("%S+", "")
        words = words + n
    end,
}

function get_wordcount(doc, _)
    -- skip metadata, just count body
    pandoc.walk_block(pandoc.Div(doc.blocks), wordcount)
end

function set_wordcount(meta)
    meta.wordcount = tostring(words)
    return meta
end

return {{Pandoc = get_wordcount}, {Meta = set_wordcount}}
