function Meta(meta) -- luacheck: ignore 131
    if meta["title"] == nil then
        meta["title"] = "[Untitled]"
    end
    return meta
end
