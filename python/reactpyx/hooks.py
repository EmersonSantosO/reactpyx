"""
Módulo de hooks para ReactPyx.
Este módulo exporta los hooks disponibles en la versión de Rust
"""

from core_reactpyx import (
    use_state, 
    use_effect, 
    use_effect_with_deps, 
    use_context, 
    use_reducer, 
    use_lazy_state
)

__all__ = [
    'use_state',
    'use_effect',
    'use_effect_with_deps',
    'use_context',
    'use_reducer',
    'use_lazy_state'
]

"""
Guía de Uso de Hooks de ReactPyx

1. use_state:
   - Uso: value, set_value = use_state(component_id, key, initial_value)
   - Ejemplo: count, setCount = use_state("counter", "count", 0)

2. use_effect:
   - Uso: use_effect(effect_function)
   - Ejemplo: use_effect(lambda: print("Componente renderizado"))
   - Se ejecuta en cada renderizado

3. use_effect_with_deps:
   - Uso: use_effect_with_deps(effect_id, effect_function, dependencies)
   - Ejemplo: use_effect_with_deps("counter-effect", lambda deps: print(f"Count: {count}"), [count])
   - Solo se ejecuta cuando cambian las dependencias

4. use_context:
   - Uso: value = use_context(component_id, key)
   - Ejemplo: theme = use_context("ThemeProvider", "theme")

5. use_reducer:
   - Uso: state, dispatch = use_reducer(component_id, key, reducer, initial_state)
   - Ejemplo: 
     state, dispatch = use_reducer("todo-app", "todos", lambda state, action: [...], [])
     dispatch({"type": "ADD_TODO", "payload": "Nueva tarea"})

6. use_lazy_state:
   - Uso: value = use_lazy_state(component_id, key, initial_value=None)
   - Ejemplo: config = use_lazy_state("app", "config", load_default_config())
"""
