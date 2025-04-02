"""
Compatibility layer for Python 3.13 features in older Python versions
"""
import sys
import typing
from typing import Any, Dict, Generic, List, Optional, Tuple, TypeVar, Union, get_type_hints

# Check if we're running on Python 3.13+
PY313_PLUS = sys.version_info >= (3, 13)

# For Python versions < 3.13, we'll use typing_extensions if available
try:
    from typing_extensions import TypedDict as _TypedDict
except ImportError:
    # Fallback if typing_extensions is not available
    from typing import TypedDict as _TypedDict


def typed(cls):
    """
    Python 3.13 @typed decorator compatibility function
    
    In Python 3.13+, uses the native implementation.
    In older versions, emulates similar behavior using TypedDict or class modification.
    """
    if PY313_PLUS:
        # In Python 3.13+, use the native implementation
        from typing import typed as py313_typed
        return py313_typed(cls)
    else:
        # For older Python versions, emulate similar behavior
        annotations = get_type_hints(cls)
        if hasattr(cls, "__annotations__"):
            cls.__annotations__ = annotations
        
        # Create a function to validate inputs based on annotations
        original_init = cls.__init__
        
        def __init__(self, **kwargs):
            # Validate types if possible
            for key, value in kwargs.items():
                if key in annotations:
                    expected_type = annotations[key]
                    # Simple type checking (not as comprehensive as Python 3.13's)
                    if not isinstance(value, expected_type) and expected_type != Any:
                        raise TypeError(f"Argument '{key}' must be {expected_type}, got {type(value)}")
            
            # Call the original __init__
            original_init(self, **kwargs)
        
        cls.__init__ = __init__
        return cls


# Type parameter implementation for older Python versions
T = TypeVar('T')

def type_param_compat(name, bases=None):
    """
    Implementation of PEP 695's type parameter syntax for older Python versions
    
    Example:
    # Instead of: type Point[T] = tuple[T, T]
    # Use: Point = type_param_compat('Point', tuple)
    """
    if PY313_PLUS:
        # In Python 3.13, this would be handled directly by the interpreter
        pass
    else:
        # For older Python versions, return a generic type
        if bases is None:
            # For simple type aliases
            return TypeVar(name)
        else:
            # For generic type definitions
            return Generic[TypeVar(name)]
