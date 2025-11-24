from typing import Callable, Dict, Any, Optional, List
from .registry import get_handler
from ._core import VNode, diff_nodes, Patch

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

        # Re-render después de las actualizaciones de estado
        new_vdom = self.root_component()

        # Si no hay VDOM previo, enviamos reemplazo completo
        if self.current_vdom is None:
            html = new_vdom.render()
            self.current_vdom = new_vdom
            return {"type": "full_replace", "html": html}

        old_vdom = self.current_vdom

        # Calcular diff utilizando la implementación en Rust
        patches: List[Patch] = diff_nodes(old_vdom, new_vdom)

        # Actualizar VDOM actual
        self.current_vdom = new_vdom

        # Si no hay patches, no es necesario que el cliente haga nada
        if not patches:
            return {"type": "noop"}

        # Serializar patches a una representación JSON-friendly
        serialized_patches = []
        for p in patches:
            # Creamos un dict mínimo basado en el tipo
            variant_name = p.__class__.__name__
            if variant_name == "AddProp":
                serialized_patches.append(
                    {
                        "op": "add_prop",
                        "key": getattr(p, "key", None),
                        "value": str(getattr(p, "value", "")),
                    }
                )
            elif variant_name == "RemoveProp":
                serialized_patches.append(
                    {
                        "op": "remove_prop",
                        "key": getattr(p, "key", None),
                    }
                )
            elif variant_name == "UpdateProp":
                serialized_patches.append(
                    {
                        "op": "update_prop",
                        "key": getattr(p, "key", None),
                        "value": str(getattr(p, "value", "")),
                    }
                )
            elif variant_name == "AddChild":
                serialized_patches.append({"op": "add_child"})
            elif variant_name == "RemoveChild":
                serialized_patches.append(
                    {
                        "op": "remove_child",
                        "index": getattr(p, "index", None),
                    }
                )
            elif variant_name == "ReplaceChild":
                serialized_patches.append(
                    {
                        "op": "replace_child",
                        "index": getattr(p, "index", None),
                    }
                )

        return {"type": "patches", "patches": serialized_patches}


# Global runtime instance
runtime = RuntimeManager()
