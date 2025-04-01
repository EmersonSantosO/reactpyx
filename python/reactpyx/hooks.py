"""
Hook module for ReactPyx.
This module exports hooks available from the Rust version
"""

from reactpyx._core import (  
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
ReactPyx Hooks Usage Guide

1. use_state:
   - Usage: value, set_value = use_state(component_id, key, initial_value)
   - Example: count, setCount = use_state("counter", "count", 0)

2. use_effect:
   - Usage: use_effect(effect_function)
   - Example: use_effect(lambda: print("Component rendered"))
   - Runs on every render

3. use_effect_with_deps:
   - Usage: use_effect_with_deps(effect_id, effect_function, dependencies)
   - Example: use_effect_with_deps("counter-effect", lambda deps: print(f"Count: {count}"), [count])
   - Only runs when dependencies change

4. use_context:
   - Usage: value = use_context(component_id, key)
   - Example: theme = use_context("ThemeProvider", "theme")

5. use_reducer:
   - Usage: state, dispatch = use_reducer(component_id, key, reducer, initial_state)
   - Example: 
     state, dispatch = use_reducer("todo-app", "todos", lambda state, action: [...], [])
     dispatch({"type": "ADD_TODO", "payload": "New task"})

6. use_lazy_state:
   - Usage: value = use_lazy_state(component_id, key, initial_value=None)
   - Example: config = use_lazy_state("app", "config", load_default_config())
"""
