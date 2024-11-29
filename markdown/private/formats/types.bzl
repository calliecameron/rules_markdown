"""Types for output formats."""

visibility("//markdown/private/formats/...")

def filter(attr, file, arg):
    return struct(
        attr = attr,
        file = file,
        arg = arg,
    )
