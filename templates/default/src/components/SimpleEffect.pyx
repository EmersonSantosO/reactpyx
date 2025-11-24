# components/SimpleEffect.pyx

from reactpyx import use_state, use_effect

def SimpleEffect():
    """Simple component demonstrating the use_effect hook"""
    count, setCount = use_state("simple_effect", "count", 0)
    
    # Using use_effect without dependencies (runs every render)
    use_effect(lambda: print("This effect runs on every render"))
    
    def increment():
        setCount.set(count + 1)
    
    return (
        <div className="simple-effect">
            <h3>Simple Effect Component</h3>
            <p>Counter: {count}</p>
            <button onClick={increment}>Increment</button>
        </div>
    )
