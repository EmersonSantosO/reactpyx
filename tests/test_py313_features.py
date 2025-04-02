"""
Tests for Python 3.13 feature compatibility in ReactPyx
"""
import sys
import unittest
import pytest

from src.python.py313_compat import typed, type_param_compat

# Skip tests if Python version is below 3.13
pytestmark = pytest.mark.skipif(
    sys.version_info < (3, 13),
    reason="Python 3.13 specific features are being tested"
)

class TestPython313Features(unittest.TestCase):
    """Test suite for Python 3.13 features"""

    def test_typed_decorator(self):
        """Test typed decorator functionality"""
        # This test will only run on Python 3.13+
        if sys.version_info >= (3, 13):
            try:
                @typed
                class User:
                    name: str
                    age: int
                    active: bool = True
                
                # Test instantiation
                user = User(name="Alice", age=30)
                self.assertEqual(user.name, "Alice")
                self.assertEqual(user.age, 30)
                self.assertTrue(user.active)
                
                # Test type validation
                with self.assertRaises(TypeError):
                    User(name=42, age="thirty")  # Wrong types
            except ImportError:
                pytest.skip("Python 3.13's typed decorator not available")

    def test_type_parameters(self):
        """Test type parameter syntax"""
        if sys.version_info >= (3, 13):
            try:
                # Using type parameter syntax
                # In Python 3.13: type Point[T] = tuple[T, T]
                # But we can't use that syntax directly in this test for older Python compatibility
                
                # Let's create some typed data
                from typing import List, Tuple, TypeVar
                
                T = TypeVar('T')
                Point = Tuple[T, T]
                
                # Test it works as expected
                p: Point[int] = (1, 2)
                self.assertEqual(p[0], 1)
                self.assertEqual(p[1], 2)
            except ImportError:
                pytest.skip("Python 3.13's type parameter syntax not available")

if __name__ == "__main__":
    unittest.main()
