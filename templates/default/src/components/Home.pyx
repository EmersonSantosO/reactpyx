from reactpyx import use_state, use_effect, use_effect_with_deps

def Home():
    """Home page component with counter demonstration"""
    count, setCount = use_state("home", "count", 0)
    message, setMessage = use_state("home", "message", "")
    
    def increment():
        setCount.set(count + 1)
    
    # Demonstration of use_effect (no dependencies, runs every time)
    use_effect(lambda: print("Rendering Home component"))
    
    # Demonstration of use_effect_with_deps (with dependencies, runs when count changes)
    use_effect_with_deps(
        "count_effect", 
        lambda deps: setMessage.set(f"Counter updated to: {count}"),
        [count]
    )
    
    return (
        <section>
            <h2>Home Page</h2>
            <p>This is the main page of your ReactPyx application.</p>
            <div>
                <p>Counter: {count}</p>
                {message and <div className="message">{message}</div>}
                <button onClick={increment}>Increment</button>
            </div>
        </section>
    )
