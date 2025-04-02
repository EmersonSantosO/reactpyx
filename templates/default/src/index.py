# default/src/index.py

import os
import importlib.util
import logging
import sys

# Configure logging for component loading
logger = logging.getLogger(__name__)
logger.setLevel(logging.DEBUG)
handler = logging.StreamHandler(sys.stdout)
formatter = logging.Formatter("[%(levelname)s] %(message)s")
handler.setFormatter(formatter)
logger.addHandler(handler)


def load_all_components():
    """Dynamically load all component modules from the components directory"""
    components = {}
    base_dir = os.path.dirname(__file__)
    components_dir = os.path.join(base_dir, "components")

    for root, _, files in os.walk(components_dir):
        for file in files:
            if file.endswith(".pyx") and not file.startswith("__"):
                file_path = os.path.join(root, file)
                module_name = os.path.splitext(os.path.relpath(file_path, base_dir))[
                    0
                ].replace(os.sep, ".")
                try:
                    spec = importlib.util.spec_from_file_location(
                        module_name, file_path
                    )
                    module = importlib.util.module_from_spec(spec)
                    spec.loader.exec_module(module)
                    component_name = os.path.splitext(file)[0]
                    component = getattr(module, component_name)
                    components[component_name] = component
                    logger.debug(
                        f"Component loaded: {component_name} from {module_name}"
                    )
                except Exception as e:
                    logger.error(f"Error loading component '{module_name}': {e}")
    return components


# Load all components and make them available globally
components = load_all_components()
globals().update(components)
