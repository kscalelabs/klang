"""Defines the top-level package for PyKlang."""

from .bindings import get_version

# Use the version from the Rust bindings.
__version__ = get_version()
