local metadata_out_file = nil

function get_metadata_out_file(meta)
    if meta["metadata-out-file"] then
        metadata_out_file = meta["metadata-out-file"]
        meta["metadata-out-file"] = nil
    end
    return meta
end

function write_metadata(doc)
    if metadata_out_file then
        local template = pandoc.template.compile("$meta-json$")
        local options = pandoc.WriterOptions({template = template})
        local output = pandoc.write(doc, "markdown", options)
        local outputFile = io.open(metadata_out_file, "w")
        if outputFile == nil then
            io.stderr:write("Failed to open metadata output file: " .. metadata_out_file .. "\n")
            os.exit(1)
        end
        outputFile:write(output)
        outputFile:close()
    end
end

return {{Meta = get_metadata_out_file}, {Pandoc = write_metadata}}
