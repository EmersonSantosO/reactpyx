# CSS Integration Guide for ReactPyx

This guide explains how to integrate CSS frameworks and custom styles with your ReactPyx application.

## Using CSS Frameworks

ReactPyx supports popular CSS frameworks through CDN integration. The two officially supported frameworks are:

### Tailwind CSS

To use Tailwind CSS in your project:

```bash
reactpyx Install tailwind
```

This will:

1. Create a CDN integration file at `public/tailwind-cdn.html`
2. Generate a helper module at `src/tailwind_helper.py`
3. Add sample configuration in `src/styles/tailwind-config.css`

Then in your components:

```python
from src.tailwind_helper import use_tailwind, tw_classes

def MyComponent(props):
    # Enable Tailwind
    use_tailwind()

    # Now use Tailwind classes
    return <div className="flex p-4 bg-blue-100 rounded">
        <h1 className="text-xl font-bold">Hello Tailwind</h1>
    </div>
```

### Bootstrap

To use Bootstrap in your project:

```bash
reactpyx Install bootstrap
```

This will:

1. Create a CDN integration file at `public/bootstrap-cdn.html`
2. Generate a helper module at `src/bootstrap_helper.py`

Then in your components:

```python
from src.bootstrap_helper import use_bootstrap, bs_button

def MyComponent(props):
    # Enable Bootstrap
    use_bootstrap()

    # Now use Bootstrap classes
    return <div className="container mt-4">
        <h1 className="display-4">Hello Bootstrap</h1>
        {bs_button("primary", "lg", "Click Me")}
    </div>
```

## Custom CSS

### Option 1: Inline Styles

You can use inline styles directly in your components:

```python
def MyComponent(props):
    style = {
        "color": "red",
        "fontSize": "16px",
        "fontWeight": "bold"
    }

    return <div style={style}>Custom styled text</div>
```

### Option 2: CSS Files

Create CSS files in the `src/styles/` directory and they will be automatically included:

```python
# The CSS in src/styles/main.css is automatically available
def MyComponent(props):
    return <div className="container">
        <h1 className="header">Styled with CSS from main.css</h1>
    </div>
```

### Option 3: Scoped Styles

You can include styles directly within your components:

```python
def ScopedComponent():
    return <div className="scoped-component">
        <style>
            .scoped-component {
                border: 1px solid blue;
                padding: 16px;
            }
            .scoped-component h2 {
                color: blue;
            }
        </style>
        <h2>Component with Scoped Styles</h2>
        <p>These styles only affect this component</p>
    </div>
```

## Helper Functions

ReactPyx provides helper functions for CSS management:

```python
from src.css_helper import combine_classes, use_styles

def StyledComponent(props):
    is_active = props.get('active', False)

    # Combine class names conditionally
    className = combine_classes(
        "button",
        props.get('size') == 'large' and "button-large",
        is_active and "active"
    )

    # Create style objects
    styles = use_styles({
        "container": "display: flex; padding: 16px;",
        "header": "font-size: 24px; color: blue;"
    })

    return <div className={className} style={styles.container}>
        <h2 style={styles.header}>Styled Component</h2>
    </div>
```

## Best Practices

1. **Organize CSS Files**: Keep your styles organized in the `src/styles` directory
2. **Component-Specific Styles**: Use scoped styles for component-specific styling
3. **Global Styles**: Keep global styles in `main.css`
4. **Responsive Design**: Use CSS variables for responsive design
5. **Framework Integration**: Choose one CSS framework and stick with it to avoid conflicts
