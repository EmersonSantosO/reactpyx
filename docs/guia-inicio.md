# Guía de Inicio de ReactPyx

<div align="center">
  <img src="assets/getting-started.png" alt="Getting Started" width="400">
</div>

## Contenido

- [Instalación](#instalación)
- [Crear un proyecto](#crear-un-proyecto)
- [Estructura del proyecto](#estructura-del-proyecto)
- [Creando componentes](#creando-componentes)
- [Hooks disponibles](#hooks-disponibles)
- [Compilación](#compilación)

## Instalación

Asegúrate de tener Python 3.8+ y Rust 1.75+ instalados en tu sistema.

```bash
# Instalar ReactPyx desde PyPI
pip install reactpyx

# Verificar la instalación
reactpyx --version
```

## Crear un proyecto

```bash
# Crear un nuevo proyecto
reactpyx create-project mi-app

# Navegar al directorio del proyecto
cd mi-app

# Inicializar el proyecto (instala las dependencias)
reactpyx init --env development
```

## Estructura del proyecto

Un proyecto ReactPyx tiene la siguiente estructura:

```
mi-app/
├── public/
│   ├── index.html
│   └── static/
│       ├── app.js
│       └── styles.css
├── src/
│   ├── components/
│   │   └── ... (archivos .pyx)
│   ├── App.pyx
│   └── index.py
└── pyx.config.json
```

## Creando componentes

Los componentes en ReactPyx se definen en archivos `.pyx`:

```python
# src/components/Saludo.pyx

def Saludo(props):
    nombre = props.get("nombre", "Mundo")

    return (
        <div className="saludo">
            <h1>¡Hola, {nombre}!</h1>
        </div>
    )
```

Y puedes usarlos en tu aplicación:

```python
# src/App.pyx
from components.Saludo import Saludo

def App():
    return (
        <div className="app">
            <Saludo nombre="ReactPyx" />
            <p>¡Bienvenido a tu primera app!</p>
        </div>
    )
```

## Hooks disponibles

ReactPyx proporciona hooks similares a React:

```python
# Estado
valor, set_valor = use_state("clave", valor_inicial)

# Efecto
use_effect_with_deps("efecto-id", funcion_efecto, [dependencias])

# Contexto
valor = use_context("componente-id", "clave")

# Reducer
estado, dispatch = use_reducer("id", "clave", reducer_fn, estado_inicial)

# Estado perezoso
valor = use_lazy_state("id", "clave", valor_inicial_opcional)
```

## Compilación

Para compilar tu aplicación para producción:

```bash
# Para servidores Python (FastAPI)
reactpyx build --env python --output dist

# Para Node.js
reactpyx build --env node --output dist
```

¡Felicitaciones! Has creado tu primera aplicación con ReactPyx. Para más información, consulta la [documentación completa de la API](api-reference.md).
