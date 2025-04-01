from reactpyx import use_state, use_effect, use_effect_with_deps

def Home():
    count, setCount = use_state("count", 0)
    message, setMessage = use_state("message", "")
    
    def increment():
        setCount(count + 1)
    
    # Demostración de use_effect (sin dependencias, se ejecuta siempre)
    use_effect(lambda: print("Renderizando componente Home"))
    
    # Demostración de use_effect_with_deps (con dependencias, se ejecuta cuando cambia count)
    use_effect_with_deps(
        "count_effect", 
        lambda deps: setMessage(f"Contador actualizado a: {count}"),
        [count]
    )
    
    return (
        <section>
            <h2>Página de Inicio</h2>
            <p>Esta es la página principal de tu aplicación ReactPyx.</p>
            <div>
                <p>Contador: {count}</p>
                {message and <div className="message">{message}</div>}
                <button onClick={increment}>Incrementar</button>
            </div>
        </section>
    )
