import sys
import os
import pytest
import json
from unittest.mock import MagicMock, AsyncMock

# Add python/ directory to sys.path to allow importing reactpyx
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "../python")))

try:
    import reactpyx
    from reactpyx.registry import register_handler, get_handler, clear_registry
    from reactpyx.runtime import RuntimeManager, set_root
    from reactpyx.server import ConnectionManager
    from reactpyx.context import set_current_session_id, reset_current_session_id
    from reactpyx import (
        use_state,
        use_effect,
        use_effect_with_deps,
        use_reducer,
        use_context,
        VNode,
    )
except ImportError:
    pytest.skip(
        "reactpyx module not found. Ensure the Rust extension is built and installed.",
        allow_module_level=True,
    )


class TestRegistry:
    def setup_method(self):
        clear_registry()

    def test_register_and_get(self):
        def my_handler(e):
            pass

        handler_id = register_handler(my_handler)
        assert handler_id is not None
        assert isinstance(handler_id, str)

        retrieved = get_handler(handler_id)
        assert retrieved == my_handler


class TestHooks:
    def test_use_effect_with_deps(self):
        """
        Verifies that use_effect_with_deps runs only when dependencies change.
        """
        effect_id = "test_effect_1"
        mock_effect = MagicMock()

        # 1. First run - should execute
        deps_1 = [1, "a"]
        use_effect_with_deps(effect_id, mock_effect, deps_1)
        mock_effect.assert_called_once()
        mock_effect.reset_mock()

        # 2. Same deps - should NOT execute
        use_effect_with_deps(effect_id, mock_effect, deps_1)
        mock_effect.assert_not_called()

        # 3. Different deps - should execute
        deps_2 = [1, "b"]
        use_effect_with_deps(effect_id, mock_effect, deps_2)
        mock_effect.assert_called_once()

    def test_use_reducer(self):
        """
        Verifies use_reducer state transitions.
        """
        component_id = "test_reducer_comp"
        key = "count_reducer"

        def reducer(state, action):
            if action["type"] == "INC":
                return state + 1
            elif action["type"] == "DEC":
                return state - 1
            return state

        token = set_current_session_id("session_reducer")
        try:
            # Initial state
            state, dispatch = use_reducer(component_id, key, reducer, 0)
            assert state == 0

            # Dispatch INC
            dispatch.dispatch({"type": "INC"})

            # Verify new state
            new_state, _ = use_reducer(component_id, key, reducer, 0)
            assert new_state == 1

            # Dispatch DEC
            dispatch.dispatch({"type": "DEC"})

            # Verify new state
            final_state, _ = use_reducer(component_id, key, reducer, 0)
            assert final_state == 0
        finally:
            reset_current_session_id(token)

    def test_use_context(self):
        """
        Verifies sharing state between 'components' via use_context.
        """
        provider_id = "provider_comp"
        consumer_id = "consumer_comp"
        key = "shared_data"

        token = set_current_session_id("session_context")
        try:
            # 1. Set state in Provider
            val, setter = use_state(provider_id, key, "initial")
            setter.set("updated_value")

            # 2. Read state in Consumer using use_context
            # Note: use_context(component_id, key) reads from that component's state
            context_val = use_context(provider_id, key)
            assert context_val == "updated_value"

            # 3. Verify isolation (different key shouldn't exist)
            missing_val = use_context(provider_id, "non_existent")
            # The Rust implementation returns None if key doesn't exist?
            # Let's check the Rust code:
            # component_state.get(key).map(...) -> returns Option
            # So it should be None.
            assert missing_val is None

        finally:
            reset_current_session_id(token)

    def test_use_lazy_state(self):
        """
        Verifies use_lazy_state initializes only once.
        """
        from reactpyx import (
            use_lazy_state,
        )  # Ensure import if not at top level, though it is.

        component_id = "lazy_comp"
        key = "lazy_val"
        token = set_current_session_id("session_lazy")

        try:
            # 1. Initial call
            val1 = use_lazy_state(component_id, key, "initial")
            assert val1 == "initial"

            # 2. Modify state manually (simulating an update via set_state elsewhere)
            # We can use use_state to get the setter
            _, setter = use_state(component_id, key, "initial")
            setter.set("updated")

            # 3. Call lazy_state again with DIFFERENT initial value
            # It should ignore the new initial value and return the current state
            val2 = use_lazy_state(component_id, key, "different_initial")
            assert val2 == "updated"

        finally:
            reset_current_session_id(token)


class TestSessionIsolation:
    def test_state_isolation(self):
        """
        Verifies that state is isolated between different session IDs.
        """
        component_id = "test_comp_isolation"
        key = "count"

        # --- Session A ---
        token_a = set_current_session_id("session_a")
        try:
            # Initialize state to 0
            val_a, set_a = use_state(component_id, key, 0)
            assert val_a == 0

            # Update state to 10
            set_a.set(10)

            # Verify update
            val_a_new, _ = use_state(component_id, key, 0)
            assert val_a_new == 10
        finally:
            reset_current_session_id(token_a)

        # --- Session B ---
        token_b = set_current_session_id("session_b")
        try:
            # Should be initial value 0, NOT 10 from Session A
            val_b, set_b = use_state(component_id, key, 0)
            assert val_b == 0

            # Update state to 20
            set_b.set(20)

            # Verify update
            val_b_new, _ = use_state(component_id, key, 0)
            assert val_b_new == 20
        finally:
            reset_current_session_id(token_b)

        # --- Verify Session A again ---
        token_a = set_current_session_id("session_a")
        try:
            # Should still be 10
            val_a_final, _ = use_state(component_id, key, 0)
            assert val_a_final == 10
        finally:
            reset_current_session_id(token_a)


@pytest.mark.asyncio
class TestServerFlow:
    async def test_connection_manager_flow(self):
        """
        Tests the WebSocket connection flow and event handling.
        """
        manager = ConnectionManager()
        mock_ws = AsyncMock()

        # 1. Connect
        await manager.connect(mock_ws)
        assert mock_ws in manager.active_connections
        assert mock_ws in manager.sessions

        # 2. Setup a simple component
        def MyComponent():
            return VNode("div", {}, [], False, 0, None)

        set_root(MyComponent)

        # 3. Simulate an event
        # We use a fake handler ID, so it might return an error in payload,
        # but the flow should complete and send a message back.
        event_data = {"target_id": "fake_id", "type": "click"}

        await manager.handle_event(mock_ws, event_data)

        # 4. Verify response
        mock_ws.send_text.assert_called()
        call_args = mock_ws.send_text.call_args[0][0]
        response = json.loads(call_args)

        assert response["type"] == "patch"
        # Since handler wasn't found, payload might contain error, but that's expected
        assert "payload" in response

    async def test_event_handler_execution(self):
        """
        Verifies that a registered event handler is actually executed.
        """
        manager = ConnectionManager()
        mock_ws = AsyncMock()
        await manager.connect(mock_ws)

        # 1. Register a real handler
        handler_mock = MagicMock()
        handler_id = register_handler(handler_mock)

        # 2. Simulate event targeting that handler
        event_data = {"target_id": handler_id, "type": "click", "value": "test"}

        # We need to set a root component so the runtime doesn't crash when trying to render after event
        def MyComponent():
            return VNode("div", {}, [], False, 0, None)

        set_root(MyComponent)

        await manager.handle_event(mock_ws, event_data)

        # 3. Verify handler was called
        handler_mock.assert_called_once()
        # The argument passed to handler is the event data
        args = handler_mock.call_args[0][0]
        assert args["target_id"] == handler_id
        assert args["value"] == "test"

    async def test_disconnect(self):
        manager = ConnectionManager()
        mock_ws = AsyncMock()

        await manager.connect(mock_ws)
        manager.disconnect(mock_ws)

        assert mock_ws not in manager.active_connections
        assert mock_ws not in manager.sessions
