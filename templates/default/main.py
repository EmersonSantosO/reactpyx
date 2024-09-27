# default/main.py

from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
import os

app = FastAPI()


@app.get("/{full_path:path}")
async def serve_app(request: Request, full_path: str):
    index_path = os.path.join(os.path.dirname(__file__), "public", "index.html")
    with open(index_path, "r", encoding="utf-8") as f:
        content = f.read()
    return HTMLResponse(content=content)
