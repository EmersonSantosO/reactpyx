# Guía de Contribución para ReactPyx

<div align="center">
  <img src="docs/assets/contributing.png" alt="Contribución" width="300">
</div>

¡Gracias por tu interés en contribuir a ReactPyx! Esta guía te ayudará a configurar tu entorno de desarrollo y entender nuestro proceso de contribución.

## Código de Conducta

Este proyecto se adhiere a un [Código de Conducta](CODE_OF_CONDUCT.md). Al participar, se espera que mantengas este código.

## Configuración del Entorno de Desarrollo

### Requisitos previos

- Python 3.8+
- Rust 1.75+
- Cargo (incluido con Rust)
- Git

### Pasos para configurar

1. **Clonar el repositorio**

```bash
git clone https://github.com/tu-usuario/core_reactpyx.git
cd core_reactpyx
```

2. **Configurar entorno virtual de Python**

```bash
python -m venv venv
source venv/bin/activate  # En Windows: venv\Scripts\activate
pip install -e .[dev]
```

3. **Compilar los componentes Rust**

```bash
maturin develop
```

## Estructura del proyecto

La estructura del proyecto es la siguiente:
