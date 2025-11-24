# main.py - Servidor FastAPI para la aplicación ReactPyx
from fastapi import FastAPI, Request, HTTPException
from fastapi.responses import HTMLResponse, FileResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
import os
import sys

app = FastAPI(title="ReactPyxApp", version="0.1.0")

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
# sys.path.append(BASE_DIR) # Solo si necesitas importar desde aquí

templates_dir = os.path.join(BASE_DIR, "templates")
if not os.path.isdir(templates_dir):
    script_dir = os.path.dirname(os.path.realpath(__file__))
    templates_dir = os.path.join(script_dir, "templates")
    if not os.path.isdir(templates_dir):
        print(
            f"FATAL: Directory 'templates' not found near {script_dir}", file=sys.stderr
        )
        sys.exit(1)
templates = Jinja2Templates(directory=templates_dir)

# Serve static files from build directory (where styles.css and bundle.js are generated)
static_dir = os.path.join(BASE_DIR, "build")
os.makedirs(static_dir, exist_ok=True)
app.mount("/static", StaticFiles(directory=static_dir), name="static")

favicon_path = os.path.join(BASE_DIR, "public", "favicon.ico")


@app.get("/favicon.ico", include_in_schema=False)
async def favicon():
    return (
        FileResponse(favicon_path)
        if os.path.exists(favicon_path)
        else HTMLResponse(status_code=204)
    )


@app.get("/{full_path:path}", response_class=HTMLResponse)
async def serve_spa(request: Request, full_path: str):
    # SSR Implementation
    try:
        # Add build directory to path to import compiled components
        build_dir = os.path.join(BASE_DIR, "build")
        if build_dir not in sys.path:
            sys.path.append(build_dir)

        # Import the entry point (compiled from src/main.pyx)
        # src/main.pyx compiles to build/components/main.py
        from components import main as app_main

        # Reload module in development to pick up changes
        import importlib

        importlib.reload(app_main)

        # Render the app
        app_instance = app_main.MainApp()
        app_html = app_instance.render()

        # Serialize state for hydration
        import json

        app_state = app_instance.to_dict()
        app_state_json = json.dumps(app_state)
    except Exception as e:
        print(f"SSR Error: {e}")
        app_html = f"<!-- SSR Error: {e} -->"
        app_state_json = "{}"

    context = {
        "request": request,
        "page_title": "ReactPyx Application",
        "content": app_html,
        "initial_state": app_state_json,
    }
    return templates.TemplateResponse("index.jinja2", context)


@app.exception_handler(404)
async def custom_404_handler(request: Request, exc: HTTPException):
    context = {"request": request}
    template_404_path = os.path.join(templates_dir, "404.jinja2")
    if os.path.exists(template_404_path):
        return templates.TemplateResponse("404.jinja2", context, status_code=404)
    else:
        return HTMLResponse(content="<h1>404 - Not Found</h1>", status_code=404)


if __name__ == "__main__":
    import uvicorn

    port = int(os.environ.get("PORT", 8000))
    host = os.environ.get("HOST", "0.0.0.0")
    uvicorn.run("main:app", host=host, port=port, reload=True)
