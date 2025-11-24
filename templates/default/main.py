# templates/default/main.py

from fastapi import FastAPI, Request, HTTPException, WebSocket, WebSocketDisconnect
from fastapi.responses import HTMLResponse, FileResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
import os
import sys
import json

# from reactpyx import ConnectionManager # Assuming reactpyx is installed


# Simple ConnectionManager if reactpyx is not yet installed in this env context
# In a real scenario, import from reactpyx
class ConnectionManager:
    def __init__(self):
        self.active_connections: list[WebSocket] = []

    async def connect(self, websocket: WebSocket):
        await websocket.accept()
        self.active_connections.append(websocket)

    def disconnect(self, websocket: WebSocket):
        self.active_connections.remove(websocket)

    async def send_personal_message(self, message: str, websocket: WebSocket):
        await websocket.send_text(message)

    async def handle_event(self, websocket: WebSocket, data: dict):
        print(f"Received event: {data}")
        # Echo back
        response = {
            "type": "patch",
            "payload": {"message": "Event received", "original_event": data},
        }
        await self.send_personal_message(json.dumps(response), websocket)


manager = ConnectionManager()

# Create FastAPI application instance
app = FastAPI()

# Añadir ruta base al sys.path para poder importar módulos de 'src' si es necesario
BASE_DIR = os.path.dirname(os.path.abspath(__file__))

# --- ReactPyx Configuration ---
# Import the main component from the build directory
# Note: You need to run the ReactPyx compiler first
try:
    # Assuming your main component is named 'App' in 'src/main.pyx'
    # which compiles to 'build/components/main.py'
    # Adjust the import according to your project structure
    sys.path.append(os.path.join(BASE_DIR, "build"))
    from components.main import App
    import reactpyx

    reactpyx.set_root(App)
except ImportError:
    print("Warning: Could not import App component. Make sure to compile your project.")

# sys.path.append(BASE_DIR) # Descomentar si necesitas importar desde src/
# sys.path.append(BASE_DIR) # Descomentar si necesitas importar desde src/

# --- Configuración de Jinja2 ---
# Asume que tus plantillas estarán en un directorio 'templates' en la raíz del proyecto
templates_dir = os.path.join(BASE_DIR, "templates")
templates = Jinja2Templates(directory=templates_dir)  # <-- INSTANCIA CREADA

# --- Montar archivos estáticos ---
# Los archivos generados por ReactPyx (JS, CSS) irán aquí
static_dir = os.path.join(BASE_DIR, "public", "static")
# Asegúrate que el directorio exista al iniciar
os.makedirs(static_dir, exist_ok=True)
app.mount("/static", StaticFiles(directory=static_dir), name="static")  # <-- MANTENIDO

# --- Favicon (Opcional pero bueno tenerlo) ---
favicon_path = os.path.join(BASE_DIR, "public", "favicon.ico")


@app.get("/favicon.ico", include_in_schema=False)
async def favicon():
    """Sirve el favicon si existe."""
    return (
        FileResponse(favicon_path)
        if os.path.exists(favicon_path)
        else HTMLResponse(status_code=404)
    )


# --- Ruta principal para servir la SPA ---
@app.get(
    "/{full_path:path}", response_class=HTMLResponse
)  # <-- response_class=HTMLResponse
async def serve_app(request: Request, full_path: str):
    """
    Serve the single page application using Jinja2 template for any route.
    """
    # Datos que podrías querer pasar a tu plantilla inicial
    context = {
        "request": request,  # SIEMPRE requerido por TemplateResponse
        "page_title": "Mi App ReactPyx",
        # "initial_data": {"user": "Invitado"} # Ejemplo para pasar datos
    }
    # Renderiza la plantilla Jinja2 en lugar de leer un archivo estático
    return templates.TemplateResponse(
        "index.jinja2", context
    )  # <-- USA TEMPLATE RESPONSE


@app.websocket("/_reactpyx/ws")
async def websocket_endpoint(websocket: WebSocket):
    await manager.connect(websocket)
    try:
        while True:
            data = await websocket.receive_text()
            message = json.loads(data)
            if message.get("type") == "event":
                await manager.handle_event(websocket, message.get("payload"))
    except WebSocketDisconnect:
        manager.disconnect(websocket)


if __name__ == "__main__":
    import uvicorn

    # Nota: El reload=True de uvicorn reiniciará en cambios .py,
    # pero necesitarás tu HMR de Rust para cambios .pyx/.css
    # Puedes configurar host y port desde variables de entorno o argumentos CLI si lo deseas
    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)
