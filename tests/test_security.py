import pytest
import sys
import os
from unittest.mock import MagicMock

# Add python/ directory to sys.path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "../python")))

try:
    from reactpyx import use_state, use_effect, VNode
    from reactpyx.context import set_current_session_id, reset_current_session_id
    from reactpyx.registry import register_handler
except ImportError:
    pytest.skip("reactpyx module not found", allow_module_level=True)


class TestSecurity:
    def test_input_type_safety(self):
        """
        Verify that passing invalid types to Rust functions raises TypeError
        instead of crashing the interpreter.
        """
        token = set_current_session_id("sec_test_session")
        try:
            # component_id and key expect strings
            with pytest.raises(TypeError):
                use_state(123, "key", 0)  # type: ignore

            with pytest.raises(TypeError):
                use_state("comp", 456, 0)  # type: ignore

        finally:
            reset_current_session_id(token)

    def test_state_isolation_security(self):
        """
        Security check: Ensure strict state isolation.
        A user in Session B should NEVER be able to access Session A's state,
        even if they guess the component ID and key.
        """
        comp_id = "sensitive_comp"
        key = "secret_token"
        secret_value = "super_secret_123"

        # 1. User A sets a secret
        token_a = set_current_session_id("user_a")
        try:
            val, setter = use_state(comp_id, key, "initial")
            setter.set(secret_value)
        finally:
            reset_current_session_id(token_a)

        # 2. User B tries to access it
        token_b = set_current_session_id("user_b")
        try:
            # Even using the same IDs, they should get the initial value (or None/default),
            # NOT User A's value.
            val_b, _ = use_state(comp_id, key, "default")
            assert val_b == "default"
            assert val_b != secret_value
        finally:
            reset_current_session_id(token_b)

    def test_large_payload_handling(self):
        """
        DoS check: Ensure the system handles large state values without crashing.
        """
        token = set_current_session_id("large_payload_session")
        try:
            # Create a 10MB string
            large_string = "x" * (10 * 1024 * 1024)

            comp_id = "large_comp"
            key = "large_data"

            # Should not crash
            val, setter = use_state(comp_id, key, "")
            setter.set(large_string)

            # Retrieve it
            new_val, _ = use_state(comp_id, key, "")
            assert len(new_val) == len(large_string)

        finally:
            reset_current_session_id(token)

    def test_injection_characters_in_ids(self):
        """
        Verify that special characters in IDs don't cause issues (e.g. path traversal style).
        Since these are used as keys in a HashMap/DashMap, they should be treated literally.
        """
        token = set_current_session_id("injection_session")
        try:
            # Try path traversal looking keys
            nasty_id = "../../../etc/passwd"
            nasty_key = "<script>alert(1)</script>"

            val, setter = use_state(nasty_id, nasty_key, "safe")
            assert val == "safe"

            setter.set("updated")

            val_new, _ = use_state(nasty_id, nasty_key, "safe")
            assert val_new == "updated"

            # Ensure it didn't overwrite a normal key
            val_normal, _ = use_state("normal_id", "normal_key", "normal")
            assert val_normal == "normal"

        finally:
            reset_current_session_id(token)

    def test_registry_handler_cleanup(self):
        """
        Resource Exhaustion check: Ensure we can register many handlers
        (simulating a long running session) without issues,
        and that they are distinct.
        """
        # Register 1000 handlers
        handlers = []
        for i in range(1000):

            def handler(e):
                pass

            hid = register_handler(handler)
            handlers.append(hid)

        # Ensure all IDs are unique
        assert len(set(handlers)) == 1000
