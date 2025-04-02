"""
Basic ReactPyx component example
"""
from reactpyx import use_state, use_effect

def Button(props):
    """A simple button component"""
    count, set_count = use_state("button", "count", 0)
    
    def handle_click(event):
        set_count(count + 1)
    
    return (
        <button 
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-700" 
            onClick={handle_click}
        >
            Clicked {count} times
        </button>
    )

def App(props):
    """Main application component"""
    name, set_name = use_state("app", "name", "ReactPyx")
    
    use_effect(lambda: print("App rendered!"))
    
    return (
        <div className="container mx-auto p-4">
            <h1 className="text-3xl font-bold mb-4">Hello, {name}!</h1>
            <p className="mb-4">Welcome to your first ReactPyx application.</p>
            <Button />
        </div>
    )

def MainApp():
    """Entry point for the application"""
    return App()
