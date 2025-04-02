# ReactPyx Examples

This directory contains example code demonstrating different aspects of ReactPyx.

## Available Examples

### 1. Basic Component (`basic_component.pyx`)

A simple example showing the fundamental structure of ReactPyx components with state management using hooks.

```python
from reactpyx import use_state, use_effect

def Button(props):
    count, set_count = use_state("button", "count", 0)

    def handle_click(event):
        set_count(count + 1)

    return (
        <button onClick={handle_click}>
            Clicked {count} times
        </button>
    )
```

### 2. CSS Framework Integration (`css_frameworks_example.pyx`)

Demonstrates how to integrate CSS frameworks like Tailwind and Bootstrap using the CDN approach.

```bash
# Install the CSS integration helpers
reactpyx Install tailwind
# or
reactpyx Install bootstrap
```

### 3. CSS Scoping (`css_scoping_example.pyx`)

Shows different approaches to styling components in ReactPyx, including:

- Scoped CSS with `<style>` tags
- Inline styles with dynamic values
- CSS helper functions

## Running the Examples

To run any example:

1. Create a new project:

```bash
reactpyx CreateProject my-example
cd my-example
```

2. Copy the example code into your project:

```bash
# Replace the content of src/App.pyx with the example code
```

3. Initialize and run the project:

```bash
reactpyx Init --env development
reactpyx Run
```

## CSS Integration

ReactPyx offers multiple ways to integrate CSS:

1. **CDN Integration**: Install CSS frameworks via CDN links

```bash
reactpyx Install tailwind
```

2. **Scoped Styles**: Add styles directly within your components

```python
def MyComponent():
    return (
        <div className="my-component">
            <style>
                .my-component { color: blue; }
            </style>
            <h1>Hello World</h1>
        </div>
    )
```

3. **CSS Files**: Create `.css` files in the `src/styles` directory

```
my-project/
  └── src/
      └── styles/
          ├── main.css
          └── components.css
```

4. **Inline Styles**: Apply styles directly to elements

```python
def MyComponent():
    style = {"color": "blue", "fontSize": "16px"}
    return <div style={style}>Hello World</div>
```
