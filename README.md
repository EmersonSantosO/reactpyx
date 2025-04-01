<div align="center">

  <h1>ReactPyx</h1>
  <p>A modern framework combining the power of React with Python and Rust</p>
  
  <div>
    <img src="https://img.shields.io/badge/version-0.1.0-blue" alt="Version">
    <img src="https://img.shields.io/badge/status-alpha-orange" alt="Status">
    <img src="https://img.shields.io/badge/rust-1.75+-orange" alt="Rust">
    <img src="https://img.shields.io/badge/python-3.8_|_3.9_|_3.10_|_3.11_|_3.12_|_3.13-blue" alt="Python">
  </div>
</div>

## 🚀 Features

- **Virtual DOM in Rust** - Ultra-fast rendering with diff/patch operations
- **Declarative Components** - Define interfaces using JSX-like syntax
- **Hook System** - Use React-like hooks (`use_state`, `use_effect`, etc.)
- **Hot Module Replacement** - Instant reloading during development
- **Built with Rust** - High-performance core written in Rust
- **Suspense & Async Components** - Elegant handling of asynchronous loading

## 📦 Installation

```bash
pip install reactpyx
```

## 🛠️ Quick Usage

### Create a new project

```bash
reactpyx create-project my-app
cd my-app
```

### Initialize the project

```bash
reactpyx init --env development
```

### Run development server

```bash
reactpyx run
```

## 📋 Creating Components

Create components in `.pyx` files inside the `src/components` folder:

```python
# src/components/Counter.pyx

def Counter():
    count, set_count = use_state("counter", 0)

    def increment():
        set_count(count + 1)

    return (
        <div className="counter">
            <h2>Counter: {count}</h2>
            <button onClick={increment}>Increment</button>
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

## 📚 Documentation

For complete documentation, visit:

- [Getting Started](docs/getting-started.md)
- [API Reference](docs/api-reference.md)
- [Advanced Concepts](docs/advanced-concepts.md)
- [Optimization](docs/optimization.md)

## 🧩 Plugins

ReactPyx supports plugins to extend its functionality:

```bash
reactpyx install tailwind
reactpyx install bootstrap
```

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
