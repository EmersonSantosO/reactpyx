"""Python 3.13 features examples for ReactPyx

This file demonstrates the Python 3.13 features that ReactPyx supports.
"""

# Type parameter syntax (PEP 695)
type Point[T] = tuple[T, T]
type UserID = int
type UserDict[T] = dict[str, T]

# Typed class decorator for TypedDict-like functionality 
from typing import typed

@typed
class User:
    name: str
    age: int
    active: bool = True
    
    def greet(self) -> str:
        return f"Hello, {self.name}!"

# Creating instances with type validation
def create_user(name: str, age: int) -> User:
    return User(name=name, age=age)

# Enhanced f-strings for debugging
def debug_user(user: User) -> str:
    return f"{user.name=}, {user.age=}, {user.active=}"

# Using type parameters in functions
def create_point[T](x: T, y: T) -> Point[T]:
    """Create a point with components of type T"""
    return (x, y)

# Improved exception handling
def divide_safely(a: int, b: int) -> float:
    try:
        result = a / b
        return result
    except ZeroDivisionError as e:
        # Add notes to exceptions (Python 3.11+)
        e.add_note(f"Attempted to divide {a} by zero")
        e.add_note("This is handled gracefully in ReactPyx")
        
        # In Python 3.13, this will use the enhanced exception format
        return 0.0

# Example of using these features
def py313_showcase():
    # Create a typed user
    user = create_user("Alice", 30)
    
    # Use type parameters
    point_int: Point[int] = create_point(10, 20)
    point_float: Point[float] = create_point(10.5, 20.5)
    
    # Create a typed dictionary
    user_data: UserDict[str] = {
        "location": "New York",
        "department": "Engineering"
    }
    
    # Return formatted output
    return {
        "user": user,
        "user_debug": debug_user(user),
        "point_int": point_int,
        "point_float": point_float,
        "user_data": user_data,
        "divide_result": divide_safely(10, 2),
        "divide_error": divide_safely(10, 0)
    }
