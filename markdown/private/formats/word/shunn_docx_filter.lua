local strip_nondeterminism = nil

function get_strip_nondeterminism(meta)
    if meta["strip-nondeterminism"] then
        strip_nondeterminism = meta["strip-nondeterminism"]
        meta["strip-nondeterminism"] = nil
    end
    return meta
end

function run(_)
    local pandoc_data_dir = os.getenv("PANDOC_DATA_DIR")

    local success, reason, code = os.execute(strip_nondeterminism .. " -t zip " ..
                                                 pandoc.path
                                                     .join({pandoc_data_dir, "reference.docx"}))
    if not success then
        io.stderr:write("Stripping nondeterminism failed: " .. reason .. " " .. code .. "\n")
        os.exit(1)
    end
end

return {{Meta = get_strip_nondeterminism}, {Pandoc = run}}
