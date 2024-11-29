function Meta(meta) -- luacheck: ignore 131
    if meta["version"] then
        meta["subject"] = "Version: " .. pandoc.utils.stringify(meta["version"])
    end
    return meta
end
