# Contributing to ReactPyx

First off, thank you for your interest in contributing to ReactPyx!
This project is still in an early alpha stage, and feedback, ideas and
code contributions are all very welcome.

## Code of Conduct

By participating in this project you agree to uphold a respectful,
inclusive and collaborative environment.

- Be kind and professional.
- Assume good intent.
- Focus on technical issues, not on people.

If you experience or witness unacceptable behavior, please open an issue
or contact the maintainers privately.

## Project Structure

- `src/` — Rust core (virtual DOM, hooks, compiler, CLI).
- `python/` — Python package (`reactpyx`) and runtime helpers.
- `docs/` — Documentation and guides.
- `examples/` — Example ReactPyx applications and snippets.
- `templates/` — Project templates used by the CLI.
- `tests/` — Python and Rust tests.

## Development Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/EmersonSantosO/core_reactpyx.git
   cd core_reactpyx
   ```

2. Create and activate a virtualenv (optional but recommended) and install
   development dependencies:

   ```bash
   pip install -e .[dev]
   ```

3. Build the Rust extension with `maturin` in editable mode:

   ```bash
   pip install maturin
   maturin develop
   ```

4. Run tests:

   ```bash
   pytest
   cargo test
   ```

## Pull Request Guidelines

- Keep changes focused: one feature or bugfix per pull request when
  possible.
- Add or update tests when you change behavior.
- Run `pytest` and `cargo test` locally before opening a PR.
- Update documentation (`docs/` and `README.md`) when you introduce
  user-visible changes.

## Issue Reporting

When opening an issue, please include:

- A clear description of the problem or proposal.
- Steps to reproduce (if it is a bug).
- Environment details (OS, Python version, Rust version, and how you built
  ReactPyx).

Feature proposals are welcome; please try to explain how the feature fits
with the goals of ReactPyx (Python + Rust, JSX-like syntax, React-style
hooks, etc.).

## License Notice

During the early development phase, ReactPyx is distributed under a
temporary restrictive license (see `LICENSE`). By contributing to this
repository you agree that your contributions may be relicensed under the
MIT License in the future when the project transitions to that license.

Thank you for helping to improve ReactPyx!
