// ReactPyx Client Runtime
// Handles hydration and event delegation for server-rendered components

(function () {
  console.log("ReactPyx Client Runtime Initialized");

  // Configuration
  const config = {
    serverUrl: window.REACTPYX_SERVER_URL || "/_reactpyx",
    reconnectInterval: 1000,
  };

  // State
  let socket = null;
  let reconnectTimer = null;

  function init() {
    attachEventListeners();
    connectWebSocket();
  }

  function connectWebSocket() {
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const host = window.location.host;
    const url = `${protocol}//${host}${config.serverUrl}/ws`;

    console.log("Connecting to ReactPyx server:", url);
    socket = new WebSocket(url);

    socket.onopen = () => {
      console.log("ReactPyx connected");
      if (reconnectTimer) {
        clearTimeout(reconnectTimer);
        reconnectTimer = null;
      }
    };

    socket.onmessage = (event) => {
      const message = JSON.parse(event.data);
      handleServerMessage(message);
    };

    socket.onclose = () => {
      console.log("ReactPyx disconnected. Reconnecting...");
      socket = null;
      reconnectTimer = setTimeout(connectWebSocket, config.reconnectInterval);
    };

    socket.onerror = (error) => {
      console.error("ReactPyx socket error:", error);
    };
  }

  function handleServerMessage(message) {
    console.log("Received message:", message);
    if (message.type === "patch") {
      // Legacy format: { type: "patch", payload: { type: "full_replace", html: "..." } }
      applyPatch(message.payload);
    } else if (message.type === "patches") {
      // New format: { type: "patches", patches: [...] }
      applyPatches(message.patches || []);
    } else if (message.type === "full_replace") {
      applyPatch(message);
    }
  }

  function applyPatch(patch) {
    if (!patch) return;

    if (patch.type === "full_replace") {
      // Replace the entire app content
      // Assuming the app is mounted on #app or similar
      // But wait, the patch.html is likely the innerHTML of the root component
      // We need to know where to mount it.
      // For now, let's assume we replace the content of the element that triggered the event?
      // No, full re-render usually means replacing the root.

      // Ideally, we should have a root container ID.
      // Let's assume the first child of body or a specific container.
      const appContainer = document.getElementById("app") || document.body;
      appContainer.innerHTML = patch.html;

      // No necesitamos re-registrar listeners por usar delegación global.
    }
  }

  function applyPatches(patches) {
    if (!Array.isArray(patches) || patches.length === 0) {
      return;
    }

    // Por ahora, como el cliente no tiene una representación del VDOM,
    // aplicamos solo un fallback sencillo: si viene un replace_child en
    // el índice 0, tratamos como full replace del contenedor raíz.
    const hasStructuralChange = patches.some(
      (p) =>
        p.op === "replace_child" ||
        p.op === "add_child" ||
        p.op === "remove_child"
    );

    if (hasStructuralChange) {
      // Hasta que tengamos un VDOM cliente, pedimos al servidor
      // que vuelva a mandar un full_replace en la siguiente
      // interacción. Este es un modo degradado pero seguro.
      console.warn(
        "ReactPyx: received structural patches without client-side VDOM; consider enabling full_replace fallback."
      );
      return;
    }

    // Patch de atributos simples: buscar elementos por data-on-* no es trivial
    // sin más información (por ejemplo un selector). Por ahora no se aplican
    // al DOM directo; aquí se deja el hook para futuras mejoras.
  }

  function attachEventListeners() {
    // Global event delegation
    const events = ["click", "change", "input", "submit", "keydown", "keyup"];

    events.forEach((eventType) => {
      document.body.addEventListener(eventType, handleEvent);
    });
  }

  function handleEvent(event) {
    const target = event.target.closest(`[data-on-${event.type}]`);

    if (target) {
      const handlerId = target.getAttribute(`data-on-${event.type}`);
      // console.log(`Event triggered: ${event.type} on`, target);

      const eventData = {
        type: event.type,
        target_id: handlerId,
        value: target.value,
        checked: target.checked,
        // Add more properties as needed
      };

      sendEvent(eventData);

      // Visual feedback
      target.classList.add("reactpyx-active");
      setTimeout(() => target.classList.remove("reactpyx-active"), 200);
    }
  }

  function sendEvent(data) {
    if (socket && socket.readyState === WebSocket.OPEN) {
      socket.send(
        JSON.stringify({
          type: "event",
          payload: data,
        })
      );
    } else {
      console.warn("Socket not connected, cannot send event:", data);
    }
  }

  // Initialize when DOM is ready
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
