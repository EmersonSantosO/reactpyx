# templates/default/main.py

from fastapi import FastAPI, Request, HTTPException
from fastapi.responses import HTMLResponse, FileResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates # <-- IMPORTADO
import os
import sys # Import sys

# Create FastAPI application instance
app = FastAPI()

# Añadir ruta base al sys.path para poder importar módulos de 'src' si es necesario
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
# sys.path.append(BASE_DIR) # Descomentar si necesitas importar desde src/

# --- Configuración de Jinja2 ---
# Asume que tus plantillas estarán en un directorio 'templates' en la raíz del proyecto
templates_dir = os.path.join(BASE_DIR, "templates")
templates = Jinja2Templates(directory=templates_dir) # <-- INSTANCIA CREADA

# --- Montar archivos estáticos ---
# Los archivos generados por ReactPyx (JS, CSS) irán aquí
static_dir = os.path.join(BASE_DIR, "public", "static")
# Asegúrate que el directorio exista al iniciar
os.makedirs(static_dir, exist_ok=True)
app.mount("/static", StaticFiles(directory=static_dir), name="static") # <-- MANTENIDO

# --- Favicon (Opcional pero bueno tenerlo) ---
favicon_path = os.path.join(BASE_DIR, "public", "favicon.ico")
@app.get("/favicon.ico", include_in_schema=False)
async def favicon():
    """Sirve el favicon si existe."""
    return FileResponse(favicon_path) if os.path.exists(favicon_path) else HTMLResponse(status_code=404)

# --- Ruta principal para servir la SPA ---
@app.get("/{full_path:path}", response_class=HTMLResponse) # <-- response_class=HTMLResponse
async def serve_app(request: Request, full_path: str):
    """
    Serve the single page application using Jinja2 template for any route.
    """
    # Datos que podrías querer pasar a tu plantilla inicial
    context = {
        "request": request, # SIEMPRE requerido por TemplateResponse
        "page_title": "Mi App ReactPyx",
        # "initial_data": {"user": "Invitado"} # Ejemplo para pasar datos
    }
    # Renderiza la plantilla Jinja2 en lugar de leer un archivo estático
    return templates.TemplateResponse("index.jinja2", context) # <-- USA TEMPLATE RESPONSE


if __name__ == "__main__":
    import uvicorn
    # Nota: El reload=True de uvicorn reiniciará en cambios .py,
    # pero necesitarás tu HMR de Rust para cambios .pyx/.css
    # Puedes configurar host y port desde variables de entorno o argumentos CLI si lo deseas
    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)