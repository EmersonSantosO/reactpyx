# Registry for event handlers
# Maps unique IDs to callback functions

import uuid
from typing import Callable, Dict, Any

_handler_registry: Dict[str, Callable] = {}


def register_handler(callback: Callable) -> str:
    """
    Registers a callback function and returns a unique ID.
    If the callback is already registered (same object), returns existing ID?
    For now, simple UUID.
    """
    # TODO: Check if callback is already registered to avoid leaks/duplication?
    # For now, we generate a new ID every render, which is not ideal for memory
    # but simplest for prototype.
    # A better approach would be to use id(callback) if it's stable,
    # or have the callback object carry its ID.

    handler_id = str(uuid.uuid4())
    _handler_registry[handler_id] = callback
    return handler_id


def get_handler(handler_id: str) -> Callable:
    return _handler_registry.get(handler_id)


def clear_registry():
    _handler_registry.clear()
