from reactpyx import Link

def Header(props=None):
    if not isinstance(props, dict):
        props = {}
    user = props.get("user", "Invitado")
    
    return (
        <header>
            <h1>Bienvenido, {user}</h1>
            <nav>
                <Link to="/">Inicio</Link>
                <Link to="/about">Acerca de</Link>
            </nav>
        </header>
    )
