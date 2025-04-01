from reactpyx import use_state

def Header(props):
    user = props.get('user', 'Guest')
    
    return (
        <header>
            <div className="header-container">
                <h1>ReactPyx Framework</h1>
                <p>Welcome, {user}!</p>
                <nav>
                    <ul>
                        <li><a href="/">Home</a></li>
                        <li><a href="/about">About</a></li>
                    </ul>
                </nav>
            </div>
        </header>
    )
