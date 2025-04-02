# Contributing Guide for ReactPyx

<div align="center">
  <img src="docs/assets/contributing.png" alt="Contributing" width="300">
</div>

Thank you for your interest in contributing to ReactPyx! This guide will help you set up your development environment and understand our contribution process.

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Development Environment Setup

### Prerequisites

- Python 3.8+
- Rust 1.75+
- Cargo (included with Rust)
- Git

### Setup Steps

1. **Clone the repository**

```bash
git clone https://github.com/your-username/reactpyx.git
cd reactpyx
```

2. **Configure Python virtual environment**

```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -e .[dev]
```

3. **Compile the Rust components**

```bash
maturin develop
```

## Project Structure

The project structure is as follows:

```
reactpyx/
├── src/                      # Rust source code
│   ├── python/               # Python modules embedded in Rust
│   ├── cli/                  # CLI implementation
│   ├── compiler/             # PyX compiler
│   ├── hooks/                # ReactPyx hooks implementation
│   └── ...
├── templates/                # Project templates
│   └── default/              # Default project template
├── examples/                 # Example applications
├── docs/                     # Documentation
└── tests/                    # Test suite
```

## Development Workflow

1. **Create a feature branch**

```bash
git checkout -b feature/your-feature-name
```

2. **Make your changes**

3. **Run the tests**

```bash
pytest
cargo test
```

4. **Format your code**

```bash
black .
cargo fmt
```

5. **Submit a pull request**

## Testing

We use pytest for Python tests and cargo's built-in test framework for Rust:

```bash
# Run Python tests
pytest

# Run Rust tests
cargo test
```

## Documentation

Documentation is written in Markdown and stored in the `docs/` directory.

To build the documentation locally:

```bash
cd docs
mkdocs serve
```

## CSS Integration

If you're working on CSS integration features:

1. All CSS processing is done via our CDN-based approach
2. The `src/python/css_compiler.py` module handles CSS compilation
3. See the `docs/css_integration.md` for detailed documentation

## Submitting Pull Requests

1. Ensure all tests pass
2. Update documentation if needed
3. Add your name to CONTRIBUTORS.md
4. Submit your PR with a clear description of the changes

Thank you for contributing to ReactPyx!
