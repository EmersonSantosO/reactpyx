# Advanced Concepts in ReactPyx

<div align="center">
  <img src="assets/advanced-concepts.png" alt="Advanced Concepts" width="350">
</div>

## Contents

- [Memoization](#memoization)
- [Plugin Architecture](#plugin-architecture)
- [SSR (Server-Side Rendering)](#ssr-server-side-rendering)
- [Caching Strategies](#caching-strategies)
- [Design Patterns](#design-patterns)

---

## Memoization

ReactPyx allows memoizing components to avoid unnecessary renders:

```python
@memoize
def ExpensiveComponent(props):
    # Expensive calculations...
    return <div>Result: {result}</div>
```

## Plugin Architecture

You can extend ReactPyx with custom plugins:

```python
# Define plugin
def my_plugin_function(arg1, arg2):
    # Plugin implementation
    return result

# Register plugin
register_plugin("my-plugin", my_plugin_function)

# Use plugin
run_plugin("my-plugin")
```

## SSR (Server-Side Rendering)

ReactPyx supports server-side rendering:

```python
# On the server
html_string = render_to_string(App, props)

# FastAPI response
@app.get("/")
def read_root():
    return HTMLResponse(render_to_string(App, {"initial": True}))
```

## Caching Strategies

### Cached Components

```python
def MyComponent():
    return <div is_critical={True} cache_duration_secs={60}>
        <h1>Content cached for 60 seconds</h1>
    </div>
```

### Selective Invalidation

```python
# Mark components for cache invalidation
EventHandler().trigger_event("invalidate_cache", ["MyComponent"], py)
```

## Design Patterns

### Container/Presentation Pattern

```python
# Container component (logic)
def UserContainer(props):
    data, set_data = use_state("user", {})

    # Effect that runs when ID changes
    use_effect_with_deps(
        "load-user",
        lambda deps: load_user_data(props.id, set_data),
        [props.id]
    )

    # Logging effect that runs on every render
    use_effect(lambda: print(f"Rendering user: {props.id}"))

    return <UserPresentation data={data} />

# Presentation component (UI)
def UserPresentation(props):
    return <div className="user">
        <h2>{props.data.name}</h2>
        <p>{props.data.email}</p>
    </div>
```

### Higher-Order Components (HOC)

```python
def withAuthentication(Component):
    def AuthenticatedComponent(props):
        user = use_context("auth", "user")

        if not user:
            return <Redirect to="/login" />

        return <Component {...props} user={user} />

    return AuthenticatedComponent

# Usage
AdminPanel = withAuthentication(AdminPanel)
```

### Render Props

```python
def ListWithRenderProp(props):
    items = props.get("items", [])
    render_item = props.get("renderItem")

    return <ul>
        {[render_item(item) for item in items]}
    </ul>

# Usage
def MyComponent():
    items = ["one", "two", "three"]

    return <ListWithRenderProp
        items={items}
        renderItem={lambda item: <li key={item}>{item.upper()}</li>}
    />
```

For more information about optimization strategies, see the [optimization guide](optimizacion.md).
