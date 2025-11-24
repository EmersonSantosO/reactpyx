from typing import Callable, Dict, Any, Optional
from .registry import get_handler
from ._core import VNode

_GLOBAL_ROOT: Optional[Callable] = None


def set_root(component: Callable):
    global _GLOBAL_ROOT
    _GLOBAL_ROOT = component


class RuntimeManager:
    def __init__(self, root_component: Optional[Callable] = None):
        self.root_component = root_component or _GLOBAL_ROOT
        self.current_vdom: Optional[VNode] = None

    def set_root(self, component_func: Callable):
        self.root_component = component_func

    def render(self) -> str:
        if not self.root_component:
            return ""

        # Execute component function to get VNode
        # Note: This assumes the component function returns a VNode directly
        # In a real React-like system, we need to handle the component tree recursively.
        # For this prototype, we assume a simple tree.
        vnode = self.root_component()
        self.current_vdom = vnode

        # Render to HTML string
        # We need to call the Rust .render() method on the VNode
        return vnode.render()

    def handle_event(self, event_data: Dict[str, Any]) -> Dict[str, Any]:
        target_id = event_data.get("target_id")
        if not target_id:
            return {"error": "No target_id"}

        handler = get_handler(target_id)
        if not handler:
            return {"error": "Handler not found"}

        # Execute handler
        # We pass the event data as argument?
        # The handler signature in Python usually expects 'event'
        try:
            handler(event_data)
        except Exception as e:
            print(f"Error executing handler: {e}")
            return {"error": str(e)}

        # Re-render
        # After state updates (triggered by handler), we re-render the root
        new_vdom = self.root_component()

        # Diff
        # We need a diffing algorithm.
        # For now, we will just send the full HTML (Server-Side Rendering replacement)
        # This is inefficient but functional for a prototype.
        new_html = new_vdom.render()

        self.current_vdom = new_vdom

        return {"type": "full_replace", "html": new_html}  # Or "patch" if we had diffs


# Global runtime instance
runtime = RuntimeManager()
