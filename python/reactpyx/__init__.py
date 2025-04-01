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
    use_lazy_state
)

# Importar las clases principales del framework
from reactpyx._core import (  # Cambiado de "from core_reactpyx import"
    VNode,
    Patch,
    EventHandler,
    LazyComponent,
    SuspenseComponent
)

__all__ = [
    # Hooks
    'use_state',
    'use_effect',
    'use_effect_with_deps',
    'use_context',
    'use_reducer',
    'use_lazy_state',
    
    # Clases
    'VNode',
    'Patch',
    'EventHandler',
    'LazyComponent',
    'SuspenseComponent',
]
