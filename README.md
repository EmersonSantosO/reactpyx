<div align="center">

  <h1>ReactPyx</h1>
  <p>A modern framework combining the power of React with Python and Rust</p>
  
  <div>
    <img src="https://img.shields.io/badge/version-0.1.0-blue" alt="Version">
    <img src="https://img.shields.io/badge/status-alpha-orange" alt="Status">
    <img src="https://img.shields.io/badge/rust-1.75+-orange" alt="Rust">
    <img src="https://img.shields.io/badge/python-3.8_|_..._|_3.13_|_3.14-blue" alt="Python">
  </div>
</div>

> **⚠️ EARLY DEVELOPMENT NOTICE**
>
> **This project is in early stages of development and not yet ready for production use.**
>
> ReactPyx is currently undergoing initial development and testing. Features may change
> significantly between versions, and backwards compatibility is not guaranteed at this stage.

> **⚠️ TEMPORARY LICENSE RESTRICTIONS**
>
> **All rights reserved. No commercial use, redistribution or derivative works without permission.**
>
> During this early development phase, all code, documentation, and assets in this repository are
> covered by a temporary restrictive license (see `LICENSE`). This license allows personal and
> internal evaluation and contributions, but forbids commercial use and redistribution until the
> project is ready to transition to the MIT License.

## 🚀 Features

- **Virtual DOM in Rust** - Ultra-fast rendering with diff/patch operations
- **Declarative Components** - Define interfaces using JSX-like syntax
- **Hook System** - Use React-like hooks (`use_state`, `use_effect`, etc.)
- **Hot Module Replacement** - Instant reloading during development
- **Built with Rust** - High-performance core written in Rust
- **Suspense & Async Components** - Elegant handling of asynchronous loading
- **Python 3.14 Support** - Ready for the future of Python:
  - Full compatibility with Python 3.13 and 3.14 features
  - Type parameters with `type` keyword (PEP 695)
  - Enhanced f-strings with debug expressions
  - Improved exception notes and error handling

## 📦 Installation

For development (from source):

Prerequisites:

- Python 3.8+
- Rust 1.75+
- `maturin` build tool

```bash
# Install maturin if you haven't already
pip install maturin

git clone https://github.com/EmersonSantosO/core_reactpyx.git
cd core_reactpyx

# Build and install the package in editable mode
maturin develop
```

## 🛠️ Quick Usage

### Create a new project

```bash
# Using the installed CLI
reactpyx create-project my-app

# Or using python module directly
python -m reactpyx create-project my-app

cd my-app
```

### Run development server

```bash
# Initialize the project environment
reactpyx init --env development

# Run the server
reactpyx run
```

## 📋 Creating Components

Create components in `.pyx` files inside the `src/components` folder.
Note that hooks like `use_state` require a `component_id` and a unique `key` to persist state across renders.

```python
# src/components/Counter.pyx
from reactpyx import use_state

def Counter():
    # use_state(component_id, key, initial_value)
    count, set_count = use_state("Counter", "count", 0)

    def increment():
        set_count.set(count + 1)

    return (
        <div className="counter">
            <h2>Counter: {count}</h2>
            <button onClick={increment}>Increment</button>

            <style>
                .counter { padding: 20px; border: 1px solid #ccc; border-radius: 8px; }
                button { background: #007bff; color: white; border: none; padding: 10px 20px; cursor: pointer; }
            </style>
        </div>
    )
```

## 🖥️ Application example

```python
# src/App.pyx
from components.Counter import Counter

def App():
    return (
        <div className="container">
            <h1>My ReactPyx App</h1>
            <Counter />
        </div>
    )
```

## 🎨 CSS Integration

ReactPyx offers multiple ways to add styles to your application:

### CSS Framework Integration (via CDN)

```bash
# Add Tailwind CSS to your project
reactpyx install tailwind

# Add Bootstrap to your project
reactpyx install bootstrap
```

### Inline Styles

```python
def StyledComponent():
    return <div style={"color": "blue", "fontSize": "24px"}>
        Styled Component
    </div>
```

### CSS Files

Place CSS files in the `src/styles/` directory, and they'll be automatically included:

```
my-app/
  └── src/
      └── styles/
          └── main.css
```

### Component-Specific Styles

```python
def ScopedComponent():
    return <div className="my-component">
        <style>
            .my-component { color: blue; padding: 16px; }
        </style>
        <h1>Component with Scoped Styles</h1>
    </div>
```

## 📚 Documentation

For complete documentation, visit:

- [Getting Started](docs/getting-started.md)
- [API Reference](docs/api-reference.md)
- [Advanced Concepts](docs/advanced-concepts.md)
- [Optimization](docs/optimization.md)
- [CSS Integration](docs/css_integration.md)

## 🏗️ Production builds

```bash
# Build for Python server
reactpyx build --env python --output dist

# Build for Node.js
reactpyx build --env node --output dist
```

## 👨‍💻 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
