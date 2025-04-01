from reactpyx import use_state

def Header(props):
    user = props.get('user', 'Invitado')
    
    return (
        <header>
            <div className="header-container">
                <h1>ReactPyx Framework</h1>
                <p>Bienvenido, {user}!</p>
                <nav>
                    <ul>
                        <li><a href="/">Inicio</a></li>
                        <li><a href="/about">Acerca de</a></li>
                    </ul>
                </nav>
            </div>
        </header>
    )
