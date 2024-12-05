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
  element.querySelector(':scope > .name').textContent = tem.number + ' ' + tem.name
  element.querySelector(':scope > img').src = tem.portraitWikiUrl
  // element.querySelector(':scope > .type').textContent = tem.types.join();
  for (let t of tem.types) {
    let img_url = getObjectByName(window.data.types, t).icon
    element.querySelector(':scope > .type > .icon').src = "https://raw.githubusercontent.com/maael/temtem-api/refs/heads/master/public" + img_url
  }
  // let x4  = []
  // let x2  = []
  // let x_5  = []
  // let x_25  = []

  function resetModifierColumn(element, text) {
    element.replaceChildren()
    let a = document.createElement('p')
    a.className = "modifier_tag"
    a.innerHTML = text
    element.appendChild(a)
  }

  resetModifierColumn(element.querySelector(':scope > .modifier > .row > .column.x4'), 'x 4')
  resetModifierColumn(element.querySelector(':scope > .modifier > .row > .column.x2'), 'x 2')
  resetModifierColumn(element.querySelector(':scope > .modifier > .row > .column.xhalf'), 'x 0.5')
  resetModifierColumn(element.querySelector(':scope > .modifier > .row > .column.xquarter'), 'x 0.25')
  
  for (let [type, mod] of Object.entries(tem.type_modifier) ) {

    let thistype = getObjectByName(window.data.types, type)
    
    let img_url = thistype.icon
    let img_container = document.createElement('div');
    img_container.className = "image-container";
    // img_container.style.backgroundColor =  thistype.color // `rgba(${thistype.color}, 0.5)` 
    img_container.style.backgroundColor =  `rgba(${thistype.color}, 0.75)` 

    let img = document.createElement('img');
    img.src = "https://raw.githubusercontent.com/maael/temtem-api/refs/heads/master/public" + img_url;
    img.width = 50
    img_container.appendChild(img)

    let txt = document.createElement('p')
    txt.className = 'text'
    txt.innerHTML = type
    img_container.appendChild(txt)

    if (mod == 4) {
      element.querySelector(':scope > .modifier > .row > .column.x4').appendChild(img_container);
    } else if (mod == 2) {
      element.querySelector(':scope > .modifier > .row > .column.x2').appendChild(img_container);
    } else if (mod == .5) {
      element.querySelector(':scope > .modifier > .row > .column.xhalf').appendChild(img_container);
    } else if (mod == .25) {
      element.querySelector(':scope > .modifier > .row > .column.xquarter').appendChild(img_container);
    }
    // debugger
  }
  // element.querySelector(':scope > .modifier').textContent = tem.type_modifier
}

function getObjectByName(array, name) {
  return array.find(obj => obj.name === name);
}

setInterval(debug_ocr, 1000);

