"""
ReactPyx - Un framework moderno que combina React con Python y Rust
"""

__version__ = "0.5.1"

# Importar y re-exportar los hooks
from .hooks import (
    use_state,
    use_effect,
    use_effect_with_deps,
    use_context,
    use_reducer,
    use_lazy_state,
)

# Importar las clases principales del framework
try:
    from ._core import VNode, Patch, EventHandler, LazyComponent, SuspenseComponent
except ImportError:
    import _core

    VNode = _core.VNode
    Patch = _core.Patch
    EventHandler = _core.EventHandler
    LazyComponent = _core.LazyComponent
    SuspenseComponent = _core.SuspenseComponent

from .server import ConnectionManager
from .runtime import set_root

__all__ = [
    # Hooks
    "use_state",
    "use_effect",
    "use_effect_with_deps",
    "use_context",
    "use_reducer",
    "use_lazy_state",
    # Clases
    "VNode",
    "Patch",
    "EventHandler",
    "LazyComponent",
    "SuspenseComponent",
    # Server
    "ConnectionManager",
    "set_root",
]
