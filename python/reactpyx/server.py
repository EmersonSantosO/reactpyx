from fastapi import WebSocket
from typing import List, Dict, Any
import json
import asyncio
import uuid
from .runtime import RuntimeManager
from .context import set_current_session_id, reset_current_session_id


class ConnectionManager:
    def __init__(self):
        self.active_connections: Dict[WebSocket, RuntimeManager] = {}
        self.sessions: Dict[WebSocket, str] = {}

    async def connect(self, websocket: WebSocket):
        await websocket.accept()
        session_id = str(uuid.uuid4())
        self.sessions[websocket] = session_id
        self.active_connections[websocket] = RuntimeManager()
        print(f"Client connected: {session_id}")

    def disconnect(self, websocket: WebSocket):
        if websocket in self.active_connections:
            del self.active_connections[websocket]
        if websocket in self.sessions:
            del self.sessions[websocket]

    async def send_personal_message(self, message: str, websocket: WebSocket):
        await websocket.send_text(message)

    async def broadcast(self, message: str):
        for connection in self.active_connections.keys():
            await connection.send_text(message)

    async def handle_event(self, websocket: WebSocket, data: Dict[str, Any]):
        """
        Handle incoming events from the client.
        This is where we would trigger the component update logic.
        """
        if websocket not in self.active_connections:
            return

        runtime = self.active_connections[websocket]
        session_id = self.sessions[websocket]

        print(f"Received event from {session_id}: {data}")

        # Set context for this session
        token = set_current_session_id(session_id)
        try:
            # Delegate to runtime
            result = runtime.handle_event(data)

            response = {
                "type": "patch",
                "payload": result,
            }
            await self.send_personal_message(json.dumps(response), websocket)
        finally:
            reset_current_session_id(token)


manager = ConnectionManager()
