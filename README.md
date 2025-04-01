<div align="center">

  <h1>ReactPyx</h1>
  <p>Un framework moderno que combina la potencia de React con Python y Rust</p>
  
  <div>
    <img src="https://img.shields.io/badge/versión-0.1.0-blue" alt="Versión">
    <img src="https://img.shields.io/badge/estado-alpha-orange" alt="Estado">
    <img src="https://img.shields.io/badge/rust-1.75+-orange" alt="Rust">
    <img src="https://img.shields.io/badge/python-3.8_|_3.9_|_3.10_|_3.11_|_3.12_|_3.13-blue" alt="Python">
  </div>
</div>

## 🚀 Características

- **Virtual DOM en Rust** - Renderizado ultrarrápido con operaciones de diff/patch
- **Componentes declarativos** - Define tus interfaces usando sintaxis similar a JSX
- **Sistema de Hooks** - Usa hooks similares a React (`use_state`, `use_effect`, etc.)
- **Hot Module Replacement** - Recarga instantánea durante desarrollo
- **Compilado con Rust** - Core de alto rendimiento escrito en Rust
- **Suspense y componentes asíncronos** - Manejo elegante de carga asíncrona

## 📦 Instalación

```bash
pip install reactpyx
```

## 🛠️ Uso Rápido

### Crear un nuevo proyecto

```bash
reactpyx create-project mi-aplicacion
cd mi-aplicacion
```

### Inicializar el proyecto

```bash
reactpyx init --env development
```

### Ejecutar servidor de desarrollo

```bash
reactpyx run
```

## 📋 Creando componentes

Crea componentes en archivos `.pyx` dentro de la carpeta `src/components`:

```python
# src/components/Contador.pyx

def Contador():
    count, set_count = use_state("contador", 0)

    def incrementar():
        set_count(count + 1)

    return (
        <div className="contador">
            <h2>Contador: {count}</h2>
            <button onClick={incrementar}>Incrementar</button>
        </div>
    )
```

## 🖥️ Ejemplo de aplicación

```python
# src/App.pyx
from components.Contador import Contador

def App():
    return (
        <div className="container">
            <h1>Mi aplicación ReactPyx</h1>
            <Contador />
        </div>
    )
```

## 📚 Documentación

Para documentación completa, visita:

- [Guía de inicio](docs/guia-inicio.md)
- [API Reference](docs/api-reference.md)
- [Conceptos avanzados](docs/conceptos-avanzados.md)
- [Optimización](docs/optimizacion.md)

## 🧩 Plugins

ReactPyx soporta plugins para extender su funcionalidad:

```bash
reactpyx install tailwind
reactpyx install bootstrap
```

## 🏗️ Compilación para producción

```bash
# Compilar para servidor Python
reactpyx build --env python --output dist

# Compilar para Node.js
reactpyx build --env node --output dist
```

## 👨‍💻 Contribuir

¡Las contribuciones son bienvenidas! Por favor lee [CONTRIBUTING.md](CONTRIBUTING.md) para detalles sobre nuestro código de conducta y el proceso para enviarnos pull requests.

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.
