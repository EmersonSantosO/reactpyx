# src/components/StyledButton.pyx
from reactpyx import use_state

# Simple helper if css_helper.py is not generated/imported
def combine_classes(*args):
    return " ".join(filter(None, args))

def StyledButton(props):
    # State to track hover (optional, for visual feedback)
    hover, set_hover = use_state("styled_button", "hover", False)

    # Get props with defaults
    text = props.get('text', 'Default Button')
    variant = props.get('variant', 'secondary') # Default to secondary
    onClick = props.get('onClick', lambda *args: None) # Default no-op function

    # Event handlers need to potentially accept an event argument from JS frontend
    def handle_mouse_enter(event=None):
        set_hover.set(True)

    def handle_mouse_leave(event=None):
        set_hover.set(False)

    # Combine CSS classes dynamically
    button_class = combine_classes(
        "button", # Base class
        f"button-{variant}", # Variant class (e.g., button-primary)
        hover and "button-hover" # Hover class
    )

    return (
        # Assign classes and event handlers
        <button
            className={button_class}
            onClick={onClick}
            onMouseEnter={handle_mouse_enter}
            onMouseLeave={handle_mouse_leave}
        >
            {text}
        </button>
        # Note: Scoped styles for the button itself are better placed in src/styles/main.css
        # or a dedicated button.css for better organization, but could be added here too.
    )
