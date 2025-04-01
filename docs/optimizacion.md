# Optimización en ReactPyx

<div align="center">
  <img src="assets/optimization.png" alt="Optimización" width="350">
</div>

## Contenido

- [Optimización de rendimiento](#optimización-de-rendimiento)
- [Minificación](#minificación)
- [Code splitting](#code-splitting)
- [Lazy loading](#lazy-loading)
- [Profiling](#profiling)

---

## Optimización de rendimiento

### Memoización de componentes

```python
@memoize(["prop1", "prop2"])
def ComponenteCostoso(props):
    # Se renderiza solo si prop1 o prop2 cambian
    return <div>...</div>
```

### Uso correcto de Hooks

```python
def Componente():
    # ❌ MAL: Lógica costosa en cada renderizado
    datos_procesados = procesar_datos(props.datos)

    # ✅ BIEN: Solo se ejecuta cuando cambian los datos
    use_effect_with_deps(
        "procesar",
        lambda deps: set_procesados(procesar_datos(props.datos)),
        [props.datos]
    )

    # ✅ BIEN: Uso del hook use_effect para registros sin dependencias
    use_effect(lambda: console_log("Renderizado completado"))
```

### Optimizar re-renderizados

```python
def ComponenteOptimizado(props):
    # Uso del estado local para memorizar datos costosos
    memo, set_memo = use_state("memo", None)

    # Calcular resultado solo cuando cambia props.datos
    use_effect_with_deps(
        "calcular-memo",
        lambda deps: set_memo(calculo_costoso(props.datos)) if props.datos != None else None,
        [props.datos]
    )

    # Registrar cada renderizado
    use_effect(lambda: print("Componente renderizado"))

    return <div>{memo}</div>
```

### Evitar cálculos innecesarios

```python
def Componente():
    # Mal: se recalcula en cada renderizado
    datos_procesados = procesar_datos(props.datos)

    # Mejor: se calcula solo cuando props.datos cambia
    use_effect_with_deps(
        "procesar",
        lambda: set_procesados(procesar_datos(props.datos)),
        [props.datos]
    )
```

## Minificación

ReactPyx automáticamente minifica HTML, CSS y JavaScript para producción:

```bash
# La minificación está habilitada por defecto en producción
reactpyx build --env python --output dist
```

Para configurar las opciones de minificación, edita `pyx.config.json`:

```json
{
  "compilerOptions": {
    "minify": {
      "html": true,
      "css": true,
      "js": true,
      "removeComments": true,
      "collapseWhitespace": true
    }
  }
}
```

## Code splitting

Divide tu aplicación en fragmentos más pequeños:

```python
# Importación dinámica de componentes
MiComponente = dynamic_import("./components/MiComponente.pyx")

def App():
    return <div>
        <Header />
        <MiComponente />  # Se cargará solo cuando sea necesario
        <Footer />
    </div>
```

## Lazy loading

Carga componentes solo cuando se necesitan:

```python
# Crear referencia lazy
LazyAdmin = lazy_component("./components/Admin.pyx")

def App():
    ruta = use_route()

    return <div>
        {ruta == "/admin" ?
            <Suspense fallback={<Cargando />}>
                <LazyAdmin />
            </Suspense>
            :
            <PaginaPrincipal />
        }
    </div>
```

## Profiling

ReactPyx incluye herramientas para analizar el rendimiento:

```python
# Activar el modo de profiling
with profiling_mode():
    html = render_component(MiComponente, props)

# Obtener resultados
resultados = get_profiling_results()
print(f"Tiempo de renderizado: {resultados.render_time}ms")
print(f"Componentes más costosos: {resultados.expensive_components}")
```

### Visualización de rendimiento

ReactPyx incluye un visualizador de rendimiento en el modo de desarrollo:

```bash
# Activar visualizador de rendimiento
reactpyx run --profile
```

Esto muestra una interfaz gráfica donde podrás ver:

1. Tiempo de renderizado por componente
2. Número de re-renderizados
3. Cuellos de botella
4. Sugerencias de optimización

### Optimización automática

ReactPyx puede analizar tu aplicación y sugerir optimizaciones:

```bash
# Generar reporte de optimización
reactpyx analyze --output informe.html
```

Esto generará un informe detallado con sugerencias para mejorar el rendimiento de tu aplicación.
