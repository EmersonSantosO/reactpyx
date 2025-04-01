from reactpyx import use_state

def Home():
    count, setCount = use_state("count", 0)
    
    def increment():
        setCount(count + 1)
    
    return (
        <section>
            <h2>Página de Inicio</h2>
            <p>Esta es la página principal de tu aplicación ReactPyx.</p>
            <div>
                <p>Contador: {count}</p>
                <button onClick={increment}>Incrementar</button>
            </div>
        </section>
    )
