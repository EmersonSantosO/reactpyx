import sys
import os
import pytest
import importlib

# Ensure we can import from the installed package or local source
# If running in dev, local source is usually preferred if in sys.path
# But we want to verify the structure that WOULD be in production.


class TestProductionStructure:
    def test_package_import(self):
        """
        Verify that the main package can be imported.
        """
        import reactpyx

        assert reactpyx is not None
        assert hasattr(reactpyx, "__version__")
        assert isinstance(reactpyx.__version__, str)

    def test_core_extension(self):
        """
        Verify that the Rust extension is available and exposes expected members.
        """
        import reactpyx

        # It might be exposed as reactpyx._core or just _core if in path
        # The __init__.py logic handles the import.

        # Check if _core is accessible via reactpyx (if we decide to expose it,
        # currently __init__.py imports FROM it but doesn't necessarily export the module itself
        # unless we check sys.modules or try to import it directly)

        try:
            from reactpyx import _core
        except ImportError:
            # It might be a top-level module in some install layouts,
            # but in the python package layout it should be reactpyx._core
            import _core

        assert _core is not None
        # Check for some known classes
        assert hasattr(_core, "VNode")
        assert hasattr(_core, "Patch")
        assert hasattr(_core, "LazyComponent")

    def test_cli_module(self):
        """
        Verify that the CLI module exists and exposes the entry point.
        """
        import reactpyx.cli

        assert hasattr(reactpyx.cli, "run_cli_py")
        assert callable(reactpyx.cli.run_cli_py)

    def test_hooks_export(self):
        """
        Verify that hooks are exported from the top-level package.
        """
        import reactpyx

        hooks = [
            "use_state",
            "use_effect",
            "use_effect_with_deps",
            "use_context",
            "use_reducer",
            "use_lazy_state",
        ]
        for hook in hooks:
            assert hasattr(reactpyx, hook), f"reactpyx should export {hook}"
            assert callable(getattr(reactpyx, hook))

    def test_server_export(self):
        """
        Verify that server components are exported.
        """
        import reactpyx

        assert hasattr(reactpyx, "ConnectionManager")
