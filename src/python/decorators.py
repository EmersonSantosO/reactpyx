"""
Enhanced decorators for ReactPyx components with Python 3.13 support
"""
from functools import wraps
import inspect
import sys
from typing import Any, Callable, Dict, List, Optional, TypeVar, Union, cast

from .py313_compat import PY313_PLUS

# Type definitions
ComponentType = TypeVar('ComponentType', bound=Callable[..., Any])


def component(func: ComponentType) -> ComponentType:
    """
    Decorator that marks a function as a ReactPyx component.
    Enhances components with Python 3.13 features when available.
    
    Example:
    
    @component
    def Button(props):
        return <button>{props.children}</button>
    """
    @wraps(func)
    def wrapper(*args, **kwargs):
        result = func(*args, **kwargs)
        
        # Add component metadata
        if not hasattr(result, "__reactpyx_component"):
            result.__reactpyx_component = True
            
        # Add Python version info
        result.__py_version = f"{sys.version_info.major}.{sys.version_info.minor}"
            
        return result
            
    # Store original signature and annotations
    wrapper.__signature__ = inspect.signature(func)
    wrapper.__annotations__ = func.__annotations__.copy()
    wrapper.__reactpyx_component = True
    
    # Python 3.13 specific enhancements
    if PY313_PLUS:
        # In Python 3.13, we can add better type information
        wrapper.__py313_enhanced = True
        
    return cast(ComponentType, wrapper)


def memo(dependencies: Optional[List[str]] = None):
    """
    Decorator to memoize a ReactPyx component based on dependencies.
    Optimized for Python 3.13's improved function inspection.
    
    Example:
    
    @memo(["count", "status"])
    def ExpensiveComponent(props):
        return <div>{props.count}</div>
    
    @memo()  # Memoize based on all props
    def AnotherComponent(props):
        return <div>{props.value}</div>
    """
    def decorator(func: ComponentType) -> ComponentType:
        # Store previous renders based on props
        memo_cache = {}
        
        @wraps(func)
        def wrapper(*args, **kwargs):
            # Extract props (first argument in component functions)
            props = args[0] if args else {}
            
            # Create a cache key from dependencies or all props
            if dependencies:
                cache_key = tuple((k, props.get(k)) for k in dependencies if k in props)
            else:
                # Sort for consistent key generation
                cache_key = tuple(sorted((k, v) for k, v in props.items()))
                
            # Return cached result if available
            if cache_key in memo_cache:
                return memo_cache[cache_key]
                
            # Compute and cache result
            result = func(*args, **kwargs)
            memo_cache[cache_key] = result
            
            # Add memo metadata to result
            if hasattr(result, "__dict__"):  # Only if result supports attribute assignment
                result.__memoized = True
                
            return result
                
        # Mark as memoized component
        wrapper.__memoized = True
        wrapper.__memo_dependencies = dependencies
        
        # Python 3.13 specific optimizations
        if PY313_PLUS:
            # In Python 3.13, we might leverage better type inspection
            wrapper.__py313_optimized = True
            
        return cast(ComponentType, wrapper)
        
    return decorator
