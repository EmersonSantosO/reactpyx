# Conceptos Avanzados en ReactPyx

<div align="center">
  <img src="assets/advanced-concepts.png" alt="Conceptos Avanzados" width="350">
</div>

## Contenido

- [Memoización](#memoización)
- [Arquitectura de Plugins](#arquitectura-de-plugins)
- [SSR (Server-Side Rendering)](#ssr-server-side-rendering)
- [Estrategias de Caché](#estrategias-de-caché)
- [Patrones de Diseño](#patrones-de-diseño)

---

## Memoización

ReactPyx permite memoizar componentes para evitar renderizados innecesarios:

```python
@memoize
def ComponenteCostoso(props):
    # Cálculos costosos...
    return <div>Resultado: {resultado}</div>
```

## Arquitectura de Plugins

Puedes extender ReactPyx con plugins personalizados:

```python
# Definir plugin
def mi_plugin_function(arg1, arg2):
    # Implementación del plugin
    return resultado

# Registrar plugin
register_plugin("mi-plugin", mi_plugin_function)

# Usar plugin
run_plugin("mi-plugin")
```

## SSR (Server-Side Rendering

ReactPyx soporta renderizado en el servidor:

```python
# En el servidor
html_string = render_to_string(App, props)

# Respuesta FastAPI
@app.get("/")
def read_root():
    return HTMLResponse(render_to_string(App, {"inicial": True}))
```

## Estrategias de Caché

### Componentes con caché

```python
def MiComponente():
    return <div is_critical={True} cache_duration_secs={60}>
        <h1>Contenido cacheado por 60 segundos</h1>
    </div>
```

### Invalidación selectiva

```python
# Marcar componentes para invalidación de caché
EventHandler().trigger_event("invalidate_cache", ["MiComponente"], py)
```

## Patrones de Diseño

### Patrón Contenedor/Presentación

```python
# Componente contenedor (lógica)
def UsuarioContenedor(props):
    datos, set_datos = use_state("usuario", {})

    # Efecto que se ejecuta cuando cambia el ID
    use_effect_with_deps(
        "cargar-usuario",
        lambda deps: cargar_datos_usuario(props.id, set_datos),
        [props.id]
    )

    # Efecto de registro que se ejecuta en cada renderizado
    use_effect(lambda: print(f"Renderizando usuario: {props.id}"))

    return <UsuarioPresentacion datos={datos} />

# Componente presentación (UI)
def UsuarioPresentacion(props):
    return <div className="usuario">
        <h2>{props.datos.nombre}</h2>
        <p>{props.datos.email}</p>
    </div>
```

### Componentes de orden superior (HOC)

```python
def withAutenticacion(Componente):
    def ComponenteAutenticado(props):
        usuario = use_context("auth", "usuario")

        if not usuario:
            return <Redirect to="/login" />

        return <Componente {...props} usuario={usuario} />

    return ComponenteAutenticado

# Uso
AdminPanel = withAutenticacion(AdminPanel)
```

### Render Props

```python
def ListaConRenderProp(props):
    items = props.get("items", [])
    render_item = props.get("renderItem")

    return <ul>
        {[render_item(item) for item in items]}
    </ul>

# Uso
def MiComponente():
    items = ["uno", "dos", "tres"]

    return <ListaConRenderProp
        items={items}
        renderItem={lambda item: <li key={item}>{item.upper()}</li>}
    />
```

Para más información sobre estrategias de optimización, consulta la [guía de optimización](optimizacion.md).
