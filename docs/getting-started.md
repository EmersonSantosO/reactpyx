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
# Install maturin build tool
pip install maturin

# Clone and install ReactPyx from source
git clone https://github.com/EmersonSantosO/core_reactpyx.git
cd core_reactpyx
maturin develop

# Verify installation
python -m reactpyx --help
```

## Creating a project

```bash
# Create a new project
reactpyx create-project my-app

# Navigate to project directory
cd my-app

# Initialize project environment
reactpyx init --env development

# Run the development server
reactpyx run
```

## Project structure

A ReactPyx project has the following structure:

```
my-app/
├── public/
│   └── favicon.ico
├── build/ (generated)
│   ├── components/
│   ├── bundle.js
│   └── styles.css
├── src/
│   ├── components/
│   │   └── ... (.pyx files)
│   ├── styles/
│   │   └── main.css
│   ├── App.pyx
│   └── main.pyx
├── templates/
│   ├── base.jinja2
│   ├── index.jinja2
│   └── 404.jinja2
├── main.py (FastAPI server)
└── pyx.config.json
```

## Creating components

Components in ReactPyx are defined in `.pyx` files. You can include scoped styles directly in your components:

```python
# src/components/Greeting.pyx

def Greeting(props):
    name = props.get("name", "World")

    return (
        <div className="greeting">
            <h1>Hello, {name}!</h1>

            <style>
                .greeting {
                    background-color: #f0f9ff;
                    padding: 1.5rem;
                    border-radius: 0.5rem;
                    border: 1px solid #bae6fd;
                }
                h1 { color: #0369a1; }
            </style>
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

ReactPyx provides hooks similar to React, but adapted for the Python/Rust hybrid environment. Note that state hooks require a `component_id` and a `key` to uniquely identify the state atom.

```python
# State
# Returns (value, setter_object)
# Usage: setter.set(new_value)
value, set_value = use_state("component_id", "key", initial_value)

# Effect (runs on every render)
use_effect(lambda: print("Component rendered"))

# Effect with dependencies (runs only when dependencies change)
use_effect_with_deps("effect-id", effect_function, [dependencies])

# Context
value = use_context("component-id", "key")

# Reducer
# Returns (state, dispatcher_object)
# Usage: dispatch.dispatch(action)
state, dispatch = use_reducer("component_id", "key", reducer_fn, initial_state)

# Lazy state
value = use_lazy_state("component_id", "key", optional_initial_value)
```

## Complete example

```python
# src/components/Counter.pyx
from reactpyx import use_state, use_effect, use_effect_with_deps

def Counter():
    # use_state returns the current value and a SetState object
    count, set_count = use_state("Counter", "count", 0)
    message, set_message = use_state("Counter", "message", "")

    # Effect that runs on every render
    use_effect(lambda: print("Counter rendered"))

    # Effect that runs only when count changes
    use_effect_with_deps("counter-change",
                         lambda deps: set_message.set(f"Counter changed to: {count}"),
                         [count])

    def increment():
        # Use .set() method on the setter object
        set_count.set(count + 1)

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
