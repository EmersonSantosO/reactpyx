# components/SimpleEffect.pyx

from reactpyx import use_state, use_effect

def SimpleEffect():
    count, setCount = use_state("simple_count", 0)
    
    # Uso del nuevo hook use_effect sin dependencias
    use_effect(lambda: print("Este efecto se ejecuta en cada renderizado"))
    
    def increment():
        setCount(count + 1)
    
    return (
        <div className="simple-effect">
            <h3>Componente con Efecto Simple</h3>
            <p>Contador: {count}</p>
            <button onClick={increment}>Incrementar</button>
        </div>
    )
