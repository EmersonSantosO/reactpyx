"""
Demo showing the complete CSS integration capabilities in ReactPyx
"""
from reactpyx import use_state, use_effect
from src.css_helper import combine_classes

def InlineStyleDemo(props):
    """Component demonstrating dynamic inline styles"""
    color, set_color = use_state("inline_demo", "color", "#3182CE")
    
    def toggle_color():
        set_color.set("#68D391" if color == "#3182CE" else "#3182CE")
    
    styles = {
        "container": {
            "padding": "16px",
            "border": f"2px solid {color}",
            "borderRadius": "8px",
            "marginBottom": "24px"
        },
        "heading": {
            "color": color,
            "fontSize": "20px",
            "marginTop": "0"
        },
        "button": {
            "backgroundColor": color,
            "color": "white",
            "border": "none",
            "padding": "8px 16px",
            "borderRadius": "4px",
            "cursor": "pointer"
        }
    }
    
    return (
        <div style={styles["container"]}>
            <h3 style={styles["heading"]}>Inline Style Demo</h3>
            <p>This component uses dynamic inline styles with state</p>
            <button 
                style={styles["button"]} 
                onClick={toggle_color}
            >
                Toggle Color
            </button>
        </div>
    )

def ClassNameDemo(props):
    """Component demonstrating dynamic class names"""
    active, set_active = use_state("class_demo", "active", False)
    size, set_size = use_state("class_demo", "size", "medium")
    
    def toggle_active():
        set_active.set(not active)
    
    def cycle_size():
        sizes = ["small", "medium", "large"]
        current_index = sizes.index(size)
        next_index = (current_index + 1) % len(sizes)
        set_size.set(sizes[next_index])
    
    # Combine class names dynamically
    button_class = combine_classes(
        "btn",
        size == "small" and "btn-sm",
        size == "medium" and "btn-md",
        size == "large" and "btn-lg",
        active and "btn-active"
    )
    
    return (
        <div className="class-demo">
            <style>
                .class-demo {
                    padding: 16px;
                    border: 2px solid #805AD5;
                    border-radius: 8px;
                    margin-bottom: 24px;
                }
                .btn {
                    background-color: #805AD5;
                    color: white;
                    border: none;
                    margin-right: 8px;
                    border-radius: 4px;
                    cursor: pointer;
                }
                .btn-sm { padding: 4px 8px; font-size: 12px; }
                .btn-md { padding: 8px 16px; font-size: 14px; }
                .btn-lg { padding: 12px 24px; font-size: 16px; }
                .btn-active { outline: 2px solid #553C9A; }
            </style>
            
            <h3 style={{"color": "#805AD5", "marginTop": 0}}>Class Name Demo</h3>
            <p>Current size: <b>{size}</b>, Active: <b>{str(active)}</b></p>
            
            <button className={button_class} onClick={toggle_active}>
                Toggle Active
            </button>
            
            <button className="btn btn-md" onClick={cycle_size}>
                Change Size
            </button>
        </div>
    )

def StyledComponentsDemo(props):
    """Component demonstrating scoped styles with the style tag"""
    counter, set_counter = use_state("styled", "counter", 0)
    
    def increment():
        set_counter.set(counter + 1)
    
    return (
        <div className="styled-component-demo">
            <style>
                .styled-component-demo {
                    padding: 16px;
                    border: 2px solid #ED8936;
                    border-radius: 8px;
                    margin-bottom: 24px;
                }
                
                .styled-component-demo h3 {
                    color: #ED8936;
                    margin-top: 0;
                }
                
                .styled-component-demo .counter {
                    font-size: 24px;
                    font-weight: bold;
                    color: #DD6B20;
                    margin: 12px 0;
                }
                
                .styled-component-demo button {
                    background-color: #ED8936;
                    color: white;
                    border: none;
                    padding: 8px 16px;
                    border-radius: 4px;
                    cursor: pointer;
                    transition: background-color 0.2s ease;
                }
                
                .styled-component-demo button:hover {
                    background-color: #DD6B20;
                }
            </style>
            
            <h3>Styled Component Demo</h3>
            <p>This component has scoped CSS styles</p>
            <div className="counter">Count: {counter}</div>
            <button onClick={increment}>Increment</button>
        </div>
    )

def App(props):
    """Main application component demonstrating CSS integration"""
    return (
        <div className="container">
            <h1>ReactPyx CSS Integration Demo</h1>
            <p>This example showcases different ways to style components in ReactPyx</p>
            
            <h2>1. Inline Styles</h2>
            <InlineStyleDemo />
            
            <h2>2. Dynamic Class Names</h2>
            <ClassNameDemo />
            
            <h2>3. Scoped Styles with Style Tags</h2>
            <StyledComponentsDemo />
            
            <div className="info-box">
                <h3>CSS Integration</h3>
                <p>ReactPyx supports multiple styling approaches without external dependencies:</p>
                <ul>
                    <li>Inline styles with JavaScript objects</li>
                    <li>Class names with conditional logic</li>
                    <li>Scoped CSS with style tags</li>
                    <li>CSS files in the src/styles directory</li>
                    <li>CSS framework integration via CDN</li>
                </ul>
            </div>
        </div>
    )

def MainApp():
    """Entry point for the application"""
    return <App />
