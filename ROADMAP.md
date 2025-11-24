# ReactPyx Roadmap

> **Status:** Early alpha – APIs and internals may change.

This document outlines high-level goals and planned work for ReactPyx.
It is intentionally short and focused on major themes rather than
exhaustive tasks.

## 1. Core Runtime and Rendering

- [ ] Improve diff/patch integration between Rust virtual DOM and
      Python runtime, with optional fine-grained patches on the client.
- [ ] Explore client-side VDOM representation for more efficient
      updates (optional, behind a flag).
- [ ] Stabilize hooks behavior (state, effects, context, reducer) and
      document edge cases (session isolation, concurrent events).

## 2. Developer Experience (DX)

- [ ] Enhance VS Code extension for `.pyx`:
  - Better JSX-like highlighting inside Python files.
  - Snippets for common patterns (components, hooks, CSS in components).
- [ ] Improve error messages from the compiler and CLI (clearer
      diagnostics when transforming `.pyx` files).
- [ ] Provide more complete examples (SSR, async components, CSS
      frameworks, production deploys).

## 3. CLI and Build Pipeline

- [ ] Solidify `reactpyx build` for both `--env python` and
      `--env node` targets.
- [ ] Optional commands for profiling/analyzing builds (planned,
      not implemented yet).
- [ ] Better integration with FastAPI templates and deployment stories.

## 4. Documentation

- [ ] Keep English documentation as the primary source of truth.
- [ ] Add more troubleshooting/FAQ sections (common mistakes with
      hooks, imports, and runtime).
- [ ] Document the temporary license model and future MIT transition.

## 5. Stability and Testing

- [ ] Expand test coverage for runtime, hooks, and CLI.
- [ ] Add more security-related tests (event handling, registry,
      context isolation).
- [ ] Prepare for a future 0.1 beta release with a frozen public API.

If you are interested in contributing to any of these areas, please
open an issue or a discussion topic so we can coordinate efforts.
