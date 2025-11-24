# Optimization in ReactPyx

<div align="center">
  <img src="assets/optimization.png" alt="Optimization" width="350">
</div>

## Contents

- [Performance Optimization](#performance-optimization)
- [Minification](#minification)
- [Code splitting](#code-splitting)
- [Lazy loading](#lazy-loading)
- [Profiling](#profiling)

---

## Performance Optimization

### Component Memoization

```python
@memoize(["prop1", "prop2"])
def ExpensiveComponent(props):
    # Renders only when prop1 or prop2 change
    return <div>...</div>
```

### Proper Use of Hooks

```python
def Component():
    # ❌ BAD: Expensive logic in every render
    processed_data = process_data(props.data)

    # ✅ GOOD: Only executes when data changes
    use_effect_with_deps(
        "process",
        lambda deps: set_processed.set(process_data(props.data)),
        [props.data]
    )

    # ✅ GOOD: Using the use_effect hook for logging without dependencies
    use_effect(lambda: console_log("Rendering completed"))
```

### Optimize Re-renders

```python
def OptimizedComponent(props):
    # Using local state to memoize expensive data
    memo, set_memo = use_state("memo", None)

    # Calculate result only when props.data changes
    use_effect_with_deps(
        "calculate-memo",
        lambda deps: set_memo.set(expensive_calculation(props.data)) if props.data != None else None,
        [props.data]
    )

    # Log each render
    use_effect(lambda: print("Component rendered"))

    return <div>{memo}</div>
```

### Avoid Unnecessary Calculations

```python
def Component():
    # Bad: recalculated on every render
    processed_data = process_data(props.data)

    # Better: calculated only when props.data changes
    use_effect_with_deps(
        "process",
        lambda: set_processed.set(process_data(props.data)),
        [props.data]
    )
```

## Minification

ReactPyx automatically minifies HTML, CSS, and JavaScript for production:

```bash
# Minification is enabled by default in production
reactpyx build --env python --output dist
```

To configure minification options, edit `pyx.config.json`:

```json
{
  "compilerOptions": {
    "minify": {
      "html": true,
      "css": true,
      "js": true,
      "removeComments": true,
      "collapseWhitespace": true
    }
  }
}
```

## Code splitting

Split your application into smaller chunks:

```python
# Dynamic component import
MyComponent = dynamic_import("./components/MyComponent.pyx")

def App():
    return <div>
        <Header />
        <MyComponent />  # Will be loaded only when needed
        <Footer />
    </div>
```

## Lazy loading

Load components only when they are needed:

```python
# Create lazy reference
LazyAdmin = lazy_component("./components/Admin.pyx")

def App():
    route = use_route()

    return <div>
        {route == "/admin" ?
            <Suspense fallback={<Loading />}>
                <LazyAdmin />
            </Suspense>
            :
            <MainPage />
        }
    </div>
```

## Profiling

ReactPyx includes tools to analyze performance:

```python
# Enable profiling mode
with profiling_mode():
    html = render_component(MyComponent, props)

# Get results
results = get_profiling_results()
print(f"Render time: {results.render_time}ms")
print(f"Most expensive components: {results.expensive_components}")
```

### Performance Visualization

ReactPyx includes a performance visualizer in development mode:

```bash
# Enable performance visualizer
reactpyx run --profile
```

This displays a graphical interface where you can see:

1. Render time per component
2. Number of re-renders
3. Bottlenecks
4. Optimization suggestions

### Automatic Optimization

ReactPyx can analyze your application and suggest optimizations:

```bash
# Generate optimization report
reactpyx analyze --output report.html
```

This will generate a detailed report with suggestions to improve your application's performance.
