# ReactPyx API Reference

<div align="center">
  <img src="assets/api-reference.png" alt="API Reference" width="300">
  <p><strong>Compatible with Python 3.8 through 3.13</strong></p>
</div>

This documentation details all APIs available in ReactPyx.

## Table of Contents

- [Virtual DOM](#virtual-dom)
- [Hooks](#hooks)
- [Special Components](#special-components)
- [CLI](#cli)
- [JSX Precompiler](#jsx-precompiler)
- [Event System](#event-system)

---

## Virtual DOM

### VNode

The base class for virtual nodes.

```python
VNode(
    tag: str,
    props: dict,
    children: list,
    is_critical: bool = False,
    cache_duration_secs: int = 0,
    key: str = None
)
```

#### Methods

| Method     | Description              |
| ---------- | ------------------------ |
| `render()` | Renders the node as HTML |

### Patch

Types of modifications for virtual nodes.

| Type           | Description                    |
| -------------- | ------------------------------ |
| `AddProp`      | Adds a property to a node      |
| `RemoveProp`   | Removes a property from a node |
| `UpdateProp`   | Updates an existing property   |
| `AddChild`     | Adds a child node              |
| `RemoveChild`  | Removes a child node           |
| `ReplaceChild` | Replaces a child node          |

---

## Hooks

### use_state

```python
value, set_state = use_state(component_id: str, key: str, initial_value)
```

Creates local state for the component.

### use_effect

```python
use_effect(effect_function)
```

Executes a side effect on each render (without dependencies).

### use_effect_with_deps

```python
use_effect_with_deps(effect_id: str, effect_function, dependencies: list)
```

Executes side effects only when the specified dependencies change.

### use_context

```python
value = use_context(component_id: str, key: str)
```

Accesses a shared context value.

### use_reducer

```python
state, dispatch = use_reducer(component_id: str, key: str, reducer, initial_state)
```

Manages complex states with a reducer pattern.

### use_lazy_state

```python
value = use_lazy_state(component_id: str, key: str, initial_value=None)
```

Initializes a state only when needed.

---

## Special Components

### SuspenseComponent

Handles loading states and errors.

```python
suspense = SuspenseComponent()
suspense.load_data()

if suspense.is_loading():
    # Show loading indicator
elif suspense.has_error():
    # Show error message
else:
    # Show content
```

### LazyComponent

Loads components asynchronously.

```python
lazy = LazyComponent()
await lazy.load_resource_async(delay=2)

if await lazy.is_loading():
    # Show loading
else:
    result = await lazy.get_result()
    # Use the result
```

---

## CLI

### Available Commands

| Command                    | Description                       |
| -------------------------- | --------------------------------- |
| `create-project <name>`    | Creates a new project             |
| `init [--env]`             | Initializes project dependencies  |
| `run`                      | Runs the development server       |
| `build [--env] [--output]` | Builds the project for production |
| `install <library>`        | Installs a library/plugin         |

---

## JSX Precompiler

```python
precompiler = JSXPrecompiler()
python_code = precompiler.precompile_jsx("path/to/file.jsx")
```

---

## Event System

```python
handler = EventHandler()

# Add listener
handler.add_event_listener("click", callback_function)

# Trigger event
handler.trigger_event("click", [arg1, arg2], py)

# Remove listeners
handler.remove_event_listeners("click")
```

---

For more examples and use cases, see the [advanced concepts](advanced-concepts.md) section.
