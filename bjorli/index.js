import init, { greet } from "./pkg/bjorli.js";

// Initialize the WASM module
async function run() {
    await init();

    document.getElementById("greet-button").addEventListener("click", () => {
        const name = document.getElementById("name-input").value;
        const greeting = greet(name);
        document.getElementById("greeting-output").textContent = greeting;
    });
}

run();

