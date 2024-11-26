"""Utility functions."""

def _escape_key_value_part(s):
    return s.replace("\\", "\\\\").replace("=", "\\=")

def key_value_arg(key, value):
    return _escape_key_value_part(key) + "=" + _escape_key_value_part(value)
