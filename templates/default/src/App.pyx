# default/src/App.pyx

from reactpyx import use_state, use_effect
from components.Header import Header
from components.Home import Home
from components.About import About

def App():
    user, setUser = use_state("user", "ReactPyx Developer")
    
    # Mejor manejo del efecto
    use_effect(lambda: print(f"User: {user}") if user else None, [user])
    
    return (
        <div>
            <Header user={user} />
            <main>
                <Home />
                <About />
            </main>
        </div>
    )
