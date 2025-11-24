# src/App.pyx
from reactpyx import use_state, use_effect
# Asegúrate que la ruta de importación sea correcta según tu estructura
from components.Header import Header
from components.StyledButton import StyledButton

def App():
    count, set_count = use_state("app", "count", 0)

    def increment():
        set_count.set(count + 1)

    # Example effect hook
    use_effect(lambda: print("App component rendered or updated"))

    return (
        <div className="container">
            <Header title="Welcome to ReactPyx" subtitle="Built with Python, Rust, and JSX-like syntax!" />
            <main>
                <h2>Interactive Counter</h2>
                <p>Current count: {count}</p>
                <StyledButton onClick={increment} text="Click Me!" variant="primary" />
            </main>
            <footer className="footer">
                <p>Powered by ReactPyx</p>
            </footer>
            {/* Example of scoped styles within a component */}
            <style>
              .container {{ max-width: 960px; margin: 2rem auto; padding: 0 1rem; font-family: sans-serif; }}
              main {{ background-color: #fff; padding: 1.5rem; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.05); }}
              h2 {{ color: #333; margin-top: 0; }}
              .footer {{ margin-top: 2rem; border-top: 1px solid #e2e8f0; padding-top: 1rem; color: #a0aec0; text-align: center; font-size: 0.9em; }}
            </style>
        </div>
    )
