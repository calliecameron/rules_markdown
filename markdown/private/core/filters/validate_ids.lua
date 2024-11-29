function validate(elem)
    if elem.identifier:sub(1, 2) == "__" then
        io.stderr:write(
            "Element IDs must not start with '__'; this is reserved for internal use: got '" ..
                elem.identifier .. "' on a '" .. elem.tag .. "'\n")
        os.exit(1)
    end
end

return {
    {
        CodeBlock = validate,
        Div = validate,
        Figure = validate,
        Header = validate,
        Table = validate,
        Code = validate,
        Image = validate,
        Link = validate,
        Span = validate,
    },
}
