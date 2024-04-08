const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let greetInputE2;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

  const employeeid = greetInputEl.value;
  const pswd = greetInputE2.value;

   try {
    // Call Rust function and await response
    const employeeId = await invoke("user_input", { employeeid, pswd });
    greetMsgEl.textContent = "Welcome : " + employeeId;
  } catch (error) {
    greetMsgEl.textContent = "Try again: " + error;
  }
  
}



window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetInputE2 = document.querySelector("#greet-pswd");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
