from reactpyx import use_state

def Header(props):
    """Header component that displays the navigation and user information"""
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
