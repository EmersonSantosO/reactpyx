# ReactPyx VS Code Extension

This extension provides language support for **ReactPyx** (`.pyx` files), enabling syntax highlighting for Python files that contain JSX-like syntax.

## Features

- **Syntax Highlighting**: Correctly highlights Python code and embedded JSX tags.
- **Bracket Matching**: Supports `< >` as brackets.
- **Snippets**: (Coming soon)

## Installation

1. Open the `vscode-extension` folder in VS Code.
2. Press `F5` to launch a new Extension Development Host window with the extension loaded.
3. Open any `.pyx` file to see the syntax highlighting in action.

## Development

To build the extension package (`.vsix`):

```bash
npm install -g vsce
vsce package
```
