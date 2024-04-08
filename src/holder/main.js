const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let greetInputE2;
let greetMsgE2;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { employeeid: greetInputEl.value, pswd:greetInputE2.value });
  //greetMsgE2.textContent = await invoke("greet", { pswd: greetInputE2.value })
}

/*async function getpwsd() {
  greetMsgE2.textContent = await invoke("getpswd", { pswd: greetInputE2.value })
}*/

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetInputE2 = document.querySelector("#greet-pswd");
  greetMsgEl = document.querySelector("#greet-msg");
  greetMsgE2 = document.querySelector("#greet-msg-pswd")
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
