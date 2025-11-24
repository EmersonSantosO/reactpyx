"""
Advanced example demonstrating CSS scoping in ReactPyx
"""
from reactpyx import use_state, use_effect

def ScopedCSS():
    """
    Component that demonstrates CSS scoping with <style> tags
    """
    count, set_count = use_state("scoped_css", "count", 0)
    
    def increment():
        set_count.set(count + 1)
    
    # This style is scoped to this component only
    return (
        <div className="scoped-component">
            <style>
                /* These styles only apply to elements within this component */
                .scoped-component {
                    border: 2px solid #e53e3e;
                    border-radius: 8px;
                    padding: 16px;
                    margin-bottom: 16px;
                }
                
                .scoped-component h3 {
                    color: #e53e3e;
                    margin-top: 0;
                }
                
                .scoped-component .count {
                    font-size: 24px;
                    font-weight: bold;
                    color: #3182ce;
                }
                
                .scoped-component button {
                    background-color: #e53e3e;
                    color: white;
                    border: none;
                    padding: 8px 16px;
                    border-radius: 4px;
                    cursor: pointer;
                    transition: background-color 0.2s;
                }
                
                .scoped-component button:hover {
                    background-color: #c53030;
                }
            </style>
            
            <h3>Scoped CSS Example</h3>
            <p>This component has its own scoped styles.</p>
            <p>Count: <span className="count">{count}</span></p>
            <button onClick={increment}>Increment</button>
        </div>
    )

def InlineStyleComponent():
    """
    Component that demonstrates inline styles
    """
    color, set_color = use_state("inline_style", "color", "#4299e1")
    
    def toggle_color():
        if color == "#4299e1":
            set_color("#48bb78")
        else:
            set_color("#4299e1")
    
    # Define styles as variables
    container_style = {
        "border": f"2px solid {color}",
        "borderRadius": "8px",
        "padding": "16px",
        "marginBottom": "16px"
    }
    
    heading_style = {
        "color": color,
        "marginTop": "0"
    }
    
    button_style = {
        "backgroundColor": color,
        "color": "white",
        "border": "none",
        "padding": "8px 16px",
        "borderRadius": "4px",
        "cursor": "pointer"
    }
    
    return (
        <div style={container_style}>
            <h3 style={heading_style}>Inline Style Example</h3>
            <p>This component uses inline styles with dynamic values.</p>
            <button style={button_style} onClick={toggle_color}>
                Toggle Color
            </button>
        </div>
    )

def App():
    """Main application demonstrating CSS approaches"""
    return (
        <div className="container">
            <h1>CSS in ReactPyx</h1>
            <p>Different ways to style components in ReactPyx:</p>
            
            <h2>1. Scoped CSS with Style Tags</h2>
            <ScopedCSS />
            
            <h2>2. Inline Styles</h2>
            <InlineStyleComponent />
        </div>
    )

def MainApp():
    """Entry point for the application"""
    return App()
