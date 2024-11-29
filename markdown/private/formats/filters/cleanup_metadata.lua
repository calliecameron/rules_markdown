function Meta(meta) -- luacheck: ignore 131
    if meta["notes"] ~= nil then
        meta["notes"] = nil
    end
    if meta["finished"] ~= nil then
        meta["finished"] = nil
    end
    if meta["publications"] ~= nil then
        meta["publications"] = nil
    end
    if meta["wordcount"] ~= nil then
        meta["wordcount"] = nil
    end
    if meta["poetry-lines"] ~= nil then
        meta["poetry-lines"] = nil
    end
    if meta["repo"] ~= nil then
        meta["repo"] = nil
    end
    if meta["parsed-dates"] ~= nil then
        meta["parsed-dates"] = nil
    end

    if FORMAT ~= "epub" then
        if meta["identifier"] ~= nil then
            meta["identifier"] = nil
        end
    end

    return meta
end
