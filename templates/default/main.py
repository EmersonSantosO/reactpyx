# default/main.py

from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
import os

# Create FastAPI application instance
app = FastAPI()


@app.get("/{full_path:path}")
async def serve_app(request: Request, full_path: str):
    """
    Serve the single page application for any route
    This implements the SPA pattern where all routes are handled by the frontend
    """
    index_path = os.path.join(os.path.dirname(__file__), "public", "index.html")
    with open(index_path, "r", encoding="utf-8") as f:
        content = f.read()
    return HTMLResponse(content=content)
