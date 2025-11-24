import os
import sys
import shutil
import subprocess
import pytest
import tempfile
from pathlib import Path

# Path to the python package
PYTHON_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "../python"))


class TestCLIIntegration:
    @pytest.fixture(autouse=True)
    def setup_teardown(self):
        # Create a temporary directory for the test project
        self.test_dir = tempfile.mkdtemp()
        self.original_cwd = os.getcwd()

        # Set PYTHONPATH to include our local package
        self.env = os.environ.copy()
        if "PYTHONPATH" in self.env:
            self.env["PYTHONPATH"] = f"{PYTHON_DIR}{os.pathsep}{self.env['PYTHONPATH']}"
        else:
            self.env["PYTHONPATH"] = PYTHON_DIR

        yield

        # Cleanup
        os.chdir(self.original_cwd)
        shutil.rmtree(self.test_dir)

    def run_command(self, args):
        """Helper to run reactpyx command via python -m"""
        cmd = [sys.executable, "-m", "reactpyx"] + args
        result = subprocess.run(
            cmd, cwd=self.test_dir, env=self.env, capture_output=True, text=True
        )
        return result

    def test_create_project(self):
        """
        Test 'reactpyx create-project <name>'
        """
        project_name = "my_test_app"
        result = self.run_command(["create-project", project_name])

        assert result.returncode == 0, f"Command failed: {result.stderr}"

        project_path = Path(self.test_dir) / project_name
        assert project_path.exists()
        assert (project_path / "src").exists()
        assert (project_path / "src" / "main.pyx").exists()
        assert (project_path / "templates").exists()

    def test_init_project_development(self):
        """
        Test 'reactpyx init --env development'
        """
        # First create a project structure manually or via create-project
        # Let's use create-project to be realistic
        self.run_command(["create-project", "dev_app"])
        app_dir = os.path.join(self.test_dir, "dev_app")

        # Run init inside the project
        cmd = [sys.executable, "-m", "reactpyx", "init", "--env", "development"]
        result = subprocess.run(
            cmd, cwd=app_dir, env=self.env, capture_output=True, text=True
        )

        assert result.returncode == 0, f"Init failed: {result.stderr}"

        # Check for development artifacts
        styles_path = Path(app_dir) / "src" / "styles" / "main.css"
        assert styles_path.exists()

        content = styles_path.read_text()
        assert ":root" in content

    def test_build_production_python(self):
        """
        Test 'reactpyx build --env python'
        """
        # Setup project
        self.run_command(["create-project", "prod_app"])
        app_dir = os.path.join(self.test_dir, "prod_app")

        # Create a dummy config file if needed (cli_build_project mentions pyx.config.json)
        # The create-project command might not create it yet?
        # Let's check cli_create_project.rs content again.
        # It creates src/main.pyx, src/App.pyx.
        # It does NOT seem to create pyx.config.json in the snippet I read.
        # But templates/default/pyx.config.json exists in workspace.

        # Let's create a dummy config
        config_path = Path(app_dir) / "pyx.config.json"
        config_path.write_text('{"compiler": {"target": "python"}}')

        # Run build
        cmd = [sys.executable, "-m", "reactpyx", "build", "--env", "python"]
        result = subprocess.run(
            cmd, cwd=app_dir, env=self.env, capture_output=True, text=True
        )

        # Note: This might fail if the compiler implementation is not fully ready
        # or if it requires more files. But we want to test the CLI flow.
        # If it fails, we check stderr.

        if result.returncode != 0:
            # If build fails due to missing implementation details, we might skip or adjust.
            # For now, let's assert success and see.
            print(f"Build stderr: {result.stderr}")

        assert result.returncode == 0

        # Check for build artifacts (assuming build/ or dist/)
        # The Rust code says: "This will generate Python files in build/components"
        # Let's check if build directory exists
        # build_dir = Path(app_dir) / "build"
        # assert build_dir.exists()
        # (Commented out until we verify exact output location)
