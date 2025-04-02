# default/src/App.pyx

# Main application component

from reactpyx import use_state, use_effect_with_deps
from components.Header import Header
from components.Home import Home
from components.About import About
from components.SimpleEffect import SimpleEffect

def App():
    """Main application component that renders the application layout"""
    user, setUser = use_state("user", "ReactPyx Developer")
    
    # Using use_effect_with_deps for effects that depend on specific values
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
