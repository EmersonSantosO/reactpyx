from contextvars import ContextVar
from typing import Optional

# Context variable to store the current session ID
# This allows hooks to know which user session they belong to
_session_context: ContextVar[Optional[str]] = ContextVar(
    "session_context", default=None
)


def get_current_session_id() -> Optional[str]:
    return _session_context.get()


def set_current_session_id(session_id: str):
    return _session_context.set(session_id)


def reset_current_session_id(token):
    _session_context.reset(token)
