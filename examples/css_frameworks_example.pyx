"""
Example demonstrating CSS framework integration in ReactPyx
"""
from reactpyx import use_state, use_effect

# Simulate our helper modules
def use_tailwind():
    """This would be imported from the tailwind helper in a real app"""
    return {"class": "tailwind-enabled"}

def use_bootstrap():
    """This would be imported from the bootstrap helper in a real app"""
    return {"class": "bootstrap-enabled"}

def TailwindComponent(props):
    """A component using Tailwind CSS"""
    # Enable Tailwind in this component
    use_tailwind()
    
    counter, set_counter = use_state("tailwind_component", "counter", 0)
    
    def increment():
        set_counter.set(counter + 1)
    
    return (
        <div className="p-6 max-w-sm mx-auto bg-white rounded-xl shadow-md flex items-center space-x-4">
            <div className="flex-shrink-0">
                <img className="h-12 w-12" src="/img/logo.svg" alt="Logo" />
            </div>
            <div>
                <div className="text-xl font-medium text-black">Tailwind Component</div>
                <p className="text-gray-500">Count: {counter}</p>
                <button 
                    className="mt-2 px-4 py-1 text-sm text-purple-600 font-semibold rounded-full border border-purple-200 hover:text-white hover:bg-purple-600 hover:border-transparent"
                    onClick={increment}
                >
                    Increment
                </button>
            </div>
        </div>
    )

def BootstrapComponent(props):
    """A component using Bootstrap"""
    # Enable Bootstrap in this component
    use_bootstrap()
    
    counter, set_counter = use_state("bootstrap_component", "counter", 0)
    
    def increment():
        set_counter.set(counter + 1)
    
    return (
        <div className="card" style="width: 18rem;">
            <div className="card-body">
                <h5 className="card-title">Bootstrap Component</h5>
                <p className="card-text">Count: {counter}</p>
                <button 
                    className="btn btn-primary"
                    onClick={increment}
                >
                    Increment
                </button>
            </div>
        </div>
    )

def App(props):
    """Main application demonstrating both frameworks together"""
    return (
        <div className="container mx-auto p-4">
            <h1 className="text-3xl mb-4">CSS Framework Examples</h1>
            
            <div className="mb-8">
                <h2 className="text-xl mb-2">Tailwind CSS Example:</h2>
                <TailwindComponent />
            </div>
            
            <div className="mb-8">
                <h2 className="text-xl mb-2">Bootstrap Example:</h2>
                <BootstrapComponent />
            </div>
        </div>
    )

def MainApp():
    """Entry point for the application"""
    return App()
