const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function debug_ocr() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  let ocr_res = await invoke("get_ocr_results");
  // document.querySelector("#debug").textContent = ocr_res;
  let t = JSON.parse( ocr_res  )
  display_tem(t.tem1, document.querySelector("#tem1"))
  display_tem(t.tem2, document.querySelector("#tem2"))
}

function display_tem(tem, element) {
  element.querySelector(':scope > .name').textContent = tem.number + ' ' + tem.name;
  element.querySelector(':scope > .type').textContent = tem.types.join();
  element.querySelector(':scope > .modifier').textContent = tem.type_modifier;
}

setInterval(debug_ocr, 1000);

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
