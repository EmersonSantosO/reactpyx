# Referencia de API de ReactPyx

<div align="center">
  <img src="assets/api-reference.png" alt="API Reference" width="300">
</div>

Esta documentación detalla todas las APIs disponibles en ReactPyx.

## Índice

- [Virtual DOM](#virtual-dom)
- [Hooks](#hooks)
- [Componentes Especiales](#componentes-especiales)
- [CLI](#cli)
- [Precompilador JSX](#precompilador-jsx)
- [Sistema de Eventos](#sistema-de-eventos)

---

## Virtual DOM

### VNode

La clase base para los nodos virtuales.

```python
VNode(
    tag: str,
    props: dict,
    children: list,
    is_critical: bool = False,
    cache_duration_secs: int = 0,
    key: str = None
)
```

#### Métodos

| Método     | Descripción                 |
| ---------- | --------------------------- |
| `render()` | Renderiza el nodo como HTML |

### Patch

Tipos de modificaciones para nodos virtuales.

| Tipo           | Descripción                       |
| -------------- | --------------------------------- |
| `AddProp`      | Añade una propiedad a un nodo     |
| `RemoveProp`   | Elimina una propiedad de un nodo  |
| `UpdateProp`   | Actualiza una propiedad existente |
| `AddChild`     | Añade un nodo hijo                |
| `RemoveChild`  | Elimina un nodo hijo              |
| `ReplaceChild` | Reemplaza un nodo hijo            |

---

## Hooks

### use_state

```python
value, set_state = use_state(component_id: str, key: str, initial_value)
```

Crea un estado local para el componente.

### use_effect_with_deps

```python
use_effect_with_deps(effect_id: str, effect_function, dependencies: list)
```

Ejecuta efectos secundarios cuando cambian las dependencias.

### use_context

```python
value = use_context(component_id: str, key: str)
```

Accede a un valor de contexto compartido.

### use_reducer

```python
state, dispatch = use_reducer(component_id: str, key: str, reducer, initial_state)
```

Gestiona estados complejos con un patrón reducer.

### use_lazy_state

```python
value = use_lazy_state(component_id: str, key: str, initial_value=None)
```

Inicializa un estado solo cuando se necesita.

---

## Componentes Especiales

### SuspenseComponent

Maneja estados de carga y errores.

```python
suspense = SuspenseComponent()
suspense.load_data()

if suspense.is_loading():
    # Mostrar indicador de carga
elif suspense.has_error():
    # Mostrar mensaje de error
else:
    # Mostrar contenido
```

### LazyComponent

Carga componentes de forma asíncrona.

```python
lazy = LazyComponent()
await lazy.load_resource_async(delay=2)

if await lazy.is_loading():
    # Mostrar carga
else:
    resultado = await lazy.get_result()
    # Usar el resultado
```

---

## CLI

### Comandos disponibles

| Comando                    | Descripción                          |
| -------------------------- | ------------------------------------ |
| `create-project <nombre>`  | Crea un nuevo proyecto               |
| `init [--env]`             | Inicializa dependencias del proyecto |
| `run`                      | Ejecuta el servidor de desarrollo    |
| `build [--env] [--output]` | Compila el proyecto para producción  |
| `install <librería>`       | Instala una librería/plugin          |

---

## Precompilador JSX

```python
precompiler = JSXPrecompiler()
python_code = precompiler.precompile_jsx("ruta/al/archivo.jsx")
```

---

## Sistema de Eventos

```python
handler = EventHandler()

# Añadir listener
handler.add_event_listener("click", callback_function)

# Disparar evento
handler.trigger_event("click", [arg1, arg2], py)

# Eliminar listeners
handler.remove_event_listeners("click")
```

---

Para más ejemplos y casos de uso, consulta la sección de [conceptos avanzados](conceptos-avanzados.md).
