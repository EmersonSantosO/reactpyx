"""
ReactPyx with Python 3.13+ Features Example

This example demonstrates how to use Python 3.13 and 3.14 features in ReactPyx components.
"""
from reactpyx import use_state, use_effect
from typing import typed

# Using Python 3.13's type parameter declaration syntax (PEP 695)
type Point[T] = tuple[T, T]
type UserData[T] = dict[str, T]

# Using Python 3.13's typed decorator for TypedDict-like functionality
@typed
class User:
    name: str
    age: int
    active: bool = True

def UserCard(props):
    """
    Component that shows user information using Python 3.13+ features
    """
    # Create a user with Python 3.13's typed class
    user = User(name=props.get("name", "Guest"), age=props.get("age", 30))
    is_expanded, set_is_expanded = use_state("user_card", "expanded", False)
    
    # Define a point using the new type parameter syntax
    position: Point[int] = (10, 20)
    
    # Create user data with the parameterized type
    user_data: UserData[str] = {
        "location": "New York",
        "department": "Engineering"
    }
    
    def toggle_expand():
        set_is_expanded.set(not is_expanded)
    
    use_effect(lambda: print(f"UserCard rendered for {user.name}"))
    
    # Use Python 3.13's enhanced f-string capabilities
    user_info = f"{user.name=}, {user.age=}"
    
    return (
        <div className="user-card">
            <style>
                .user-card {
                    border: 1px solid #e2e8f0;
                    border-radius: 8px;
                    padding: 16px;
                    margin-bottom: 16px;
                    background-color: #f8fafc;
                }
                .user-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 8px;
                }
                .user-name {
                    font-weight: bold;
                    color: #2d3748;
                }
                .expand-btn {
                    background-color: #edf2f7;
                    border: none;
                    padding: 4px 8px;
                    border-radius: 4px;
                    cursor: pointer;
                }
                .user-details {
                    margin-top: 12px;
                    padding-top: 12px;
                    border-top: 1px dashed #e2e8f0;
                }
            </style>
            
            <div className="user-header">
                <span className="user-name">{user.name}</span>
                <button className="expand-btn" onClick={toggle_expand}>
                    {is_expanded ? "Hide" : "Show"} Details
                </button>
            </div>
            
            <div>Age: {user.age}</div>
            
            {is_expanded and (
                <div className="user-details">
                    <p>Debug info: {user_info}</p>
                    <p>Position: ({position[0]}, {position[1]})</p>
                    <p>Location: {user_data["location"]}</p>
                    <p>Department: {user_data["department"]}</p>
                    <p>Active: {str(user.active)}</p>
                </div>
            )}
        </div>
    )

def App():
    """Main app component demonstrating Python 3.13 features in ReactPyx"""
    return (
        <div className="container">
            <h1>Python 3.13 Features in ReactPyx</h1>
            <p>This example demonstrates the latest Python 3.13 features in ReactPyx components.</p>
            
            <h2>User Components</h2>
            <UserCard name="Alice Johnson" age={32} />
            <UserCard name="Bob Smith" age={28} />
            
            <div className="info-box">
                <h3>Python 3.13 Features Demonstrated</h3>
                <ul>
                    <li>Type parameter syntax with <code>type</code> keyword (PEP 695)</li>
                    <li>Typed class decorator for TypedDict-like functionality</li>
                    <li>Enhanced f-string debug expressions</li>
                    <li>Better type annotations</li>
                </ul>
            </div>
        </div>
    )

def MainApp():
    """Entry point for the application"""
    return <App />
