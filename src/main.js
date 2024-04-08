// index.js
import { invoke } from '@tauri-apps/api/tauri'
const invoke = window.__TAURI__.invoke
//const { invoke } = require("@tauri-apps/api/tauri");

invoke('user_input')

async function sendDataToRust(event) {
    event.preventDefault(); // Prevent the default form submission behavior

    const formData = new FormData(document.getElementById("greet-form"));
    const employeeId = formData.get("employee-id");
    const password = formData.get("password");

    try {
        const response = await invoke("process_input", { employee_id: employeeId, password: password });
        document.getElementById("greet-msg").innerText = `Employee ID: ${employeeId}`;
        document.getElementById("greet-msg-pswd").innerText = `Password: ${password}`;
    } catch (error) {
        console.error("Error:", error);
    }
}

document.getElementById("greet-form").addEventListener("submit", sendDataToRust);
