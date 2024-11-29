-- Whether something was the first paragraph in an included file might change
-- after inclusion, so we first have to remove all existing annotations, and
-- then re-annotate.
local first_paragraph = true

function remove_annotations(elem)
    for _, class in ipairs(elem.classes) do
        if class == "firstparagraph" or class == "otherparagraph" or class == "blankline" then
            return elem.content
        end
    end
end

function is_nbsp(elem)
    if #elem.content == 1 and elem.content[1].tag == "Str" and elem.content[1].text == "\u{A0}" then
        return true
    end
    return false
end

function set_first_paragraph(_)
    first_paragraph = true
end

function set_first_paragraph_and_skip(_)
    first_paragraph = true
    return nil, false
end

function annotate(elem)
    if is_nbsp(elem) then
        first_paragraph = true
        return pandoc.Div({elem}, pandoc.Attr("", {"blankline"})), false
    end

    local annotation = "otherparagraph"
    if first_paragraph then
        first_paragraph = false
        annotation = "firstparagraph"
    end
    return pandoc.Div({elem}, pandoc.Attr("", {annotation})), false
end

return {
    {Div = remove_annotations}, {
        traverse = "topdown",
        Pandoc = set_first_paragraph,
        BlockQuote = set_first_paragraph_and_skip,
        BulletList = set_first_paragraph_and_skip,
        CodeBlock = set_first_paragraph,
        DefinitionList = set_first_paragraph,
        Figure = set_first_paragraph_and_skip,
        Header = set_first_paragraph,
        HorizontalRule = set_first_paragraph,
        LineBlock = set_first_paragraph,
        Note = set_first_paragraph_and_skip,
        OrderedList = set_first_paragraph_and_skip,
        Table = set_first_paragraph_and_skip,
        Para = annotate,
    },
}
