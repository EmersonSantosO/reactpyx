# default/src/App.pyx

from reactpyx import use_state, use_effect_with_deps
from components.Header import Header
from components.Home import Home
from components.About import About
from components.SimpleEffect import SimpleEffect  # Añadir importación del nuevo componente

def App():
    user, setUser = use_state("user", "ReactPyx Developer")
    
    # Actualizado para usar use_effect_with_deps en lugar de use_effect con argumentos
    use_effect_with_deps("user_effect", lambda deps: print(f"User: {user}") if user else None, [user])
    
    return (
        <div>
            <Header user={user} />
            <main>
                <Home />
                <About />
                <SimpleEffect />
            </main>
        </div>
    )
