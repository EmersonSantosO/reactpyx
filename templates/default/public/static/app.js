/**
 * Client-side JavaScript for ReactPyx applications
 * This file handles basic client-side interaction
 */

// Wait for DOM to be fully loaded
document.addEventListener("DOMContentLoaded", () => {
  console.log("ReactPyx application loaded");

  // Initialize any client-side functionality
  initApp();
});

/**
 * Initialize application functionality
 */
function initApp() {
  // Add click event listeners to any elements with data-client-action attribute
  document.querySelectorAll("[data-client-action]").forEach((element) => {
    element.addEventListener("click", handleClientAction);
  });
}

/**
 * Handle client actions from data attributes
 * @param {Event} event - The click event
 */
function handleClientAction(event) {
  const action = event.currentTarget.getAttribute("data-client-action");
  const value = event.currentTarget.getAttribute("data-value");

  console.log(`Client action: ${action}, value: ${value}`);

  switch (action) {
    case "toggle":
      const targetId = event.currentTarget.getAttribute("data-target");
      const target = document.getElementById(targetId);
      if (target) {
        target.classList.toggle("hidden");
      }
      break;

    case "navigate":
      if (value) {
        window.location.href = value;
      }
      break;

    default:
      console.log(`Unhandled client action: ${action}`);
  }
}

/**
 * Example of a utility function that can be called from Python-generated code
 * @param {string} message - The message to display
 */
function showNotification(message) {
  const notification = document.createElement("div");
  notification.className = "notification";
  notification.textContent = message;

  document.body.appendChild(notification);

  setTimeout(() => {
    notification.classList.add("show");
  }, 10);

  setTimeout(() => {
    notification.classList.remove("show");
    setTimeout(() => {
      notification.remove();
    }, 300);
  }, 3000);
}
