# ReactPyx Getting Started Guide

<div align="center">
  <img src="assets/getting-started.png" alt="Getting Started" width="400">
</div>

## Contents

- [Installation](#installation)
- [Creating a project](#creating-a-project)
- [Project structure](#project-structure)
- [Creating components](#creating-components)
- [Available hooks](#available-hooks)
- [Complete example](#complete-example)
- [Building](#building)

## Installation

Make sure you have Python 3.8+ and Rust 1.75+ installed on your system.

```bash
# Install ReactPyx from PyPI
pip install reactpyx

# Verify installation
reactpyx --version
```

## Creating a project

```bash
# Create a new project
reactpyx create-project my-app

# Navigate to project directory
cd my-app

# Initialize project (installs dependencies)
reactpyx init --env development
```

## Project structure

A ReactPyx project has the following structure:

```
my-app/
├── public/
│   ├── index.html
│   └── static/
│       ├── app.js
│       └── styles.css
├── src/
│   ├── components/
│   │   └── ... (.pyx files)
│   ├── App.pyx
│   └── index.py
└── pyx.config.json
```

## Creating components

Components in ReactPyx are defined in `.pyx` files:

```python
# src/components/Greeting.pyx

def Greeting(props):
    name = props.get("name", "World")

    return (
        <div className="greeting">
            <h1>Hello, {name}!</h1>
        </div>
    )
```

And you can use them in your application:

```python
# src/App.pyx
from components.Greeting import Greeting

def App():
    return (
        <div className="app">
            <Greeting name="ReactPyx" />
            <p>Welcome to your first app!</p>
        </div>
    )
```

## Available hooks

ReactPyx provides hooks similar to React:

```python
# State
value, set_value = use_state("key", initial_value)

# Effect (runs on every render)
use_effect(lambda: print("Component rendered"))

# Effect with dependencies (runs only when dependencies change)
use_effect_with_deps("effect-id", effect_function, [dependencies])

# Context
value = use_context("component-id", "key")

# Reducer
state, dispatch = use_reducer("id", "key", reducer_fn, initial_state)

# Lazy state
value = use_lazy_state("id", "key", optional_initial_value)
```

## Complete example

```python
# src/components/Counter.pyx
from reactpyx import use_state, use_effect, use_effect_with_deps

def Counter():
    count, set_count = use_state("counter", 0)
    message, set_message = use_state("message", "")

    # Effect that runs on every render
    use_effect(lambda: print("Counter rendered"))

    # Effect that runs only when count changes
    use_effect_with_deps("counter-change",
                         lambda deps: set_message(f"Counter changed to: {count}"),
                         [count])

    def increment():
        set_count(count + 1)

    return (
        <div className="counter">
            <h2>Counter: {count}</h2>
            {message and <p>{message}</p>}
            <button onClick={increment}>Increment</button>
        </div>
    )
```

## Building

To build your application for production:

```bash
# For Python servers (FastAPI)
reactpyx build --env python --output dist

# For Node.js
reactpyx build --env node --output dist
```

Congratulations! You've created your first ReactPyx application. For more information, see the [complete API documentation](api-reference.md).
