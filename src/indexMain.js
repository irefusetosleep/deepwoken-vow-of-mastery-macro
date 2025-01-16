const { invoke } = window.__TAURI__.core;
const { WebviewWindow } = window.__TAURI__.webviewWindow;
const { emit, listen } = window.__TAURI__.event;

//makes the tab buttons do shit

const links = document.querySelectorAll('.sidebar a');
const tabs = document.querySelectorAll('.tab-content');

links.forEach(link => {
  link.addEventListener('click', event => {
    event.preventDefault();

    // Remove active class from all tabs
    tabs.forEach(tab => tab.classList.remove('active'));

    // Get the corresponding tab ID
    const tabId = link.getAttribute('data-tab');

    // Add active class to the selected tab
    document.getElementById(tabId).classList.add('active');
  });
});

//makes the keybind set inputs work

function setupKeybindChange(listId) {
  let list = document.getElementById(listId);
  let keys = list.querySelectorAll("li input");
  let keyArray = Array.from(keys);

  keyArray.forEach(keyInput => {
    // Add a keydown event listener to the entire document
    keyInput.addEventListener('keydown', (event) => {
      // Get the key name (e.g., "F3")
      const keyName = event.key;
      console.log(keyName);
      if (keyName == "Control" || keyName == "Shift" || keyName == "Alt") {
        return //lets user add modifiers, without thsi it would set the key to ctrl or smth
      }

      // Get additional key information (e.g., Shift, Ctrl, Alt)
      const modifiers = [];
      if (event.ctrlKey) modifiers.push('Ctrl');
      if (event.shiftKey) modifiers.push('Shift');
      if (event.altKey) modifiers.push('Alt');

      // Combine modifiers and key name
      const fullKeyName = modifiers.length > 0 ? `${modifiers.join('+')}+${keyName}` : keyName;

      // Display the key in the input box
      keyInput.value = fullKeyName;
      console.log(keyInput.name);
      
      emit("keybind_changed", {'name':keyInput.name, 'key': fullKeyName});

      // Prevent default action for special keys
      event.preventDefault();

      keyInput.blur();
    });
  });
}

setupKeybindChange("vow-command-list");
setupKeybindChange("cycle-command-list");

const vowStartButton = document.getElementById("vow-start-button");

vowStartButton.addEventListener("click", (event) => {
  if (vowStartButton.textContent == "Start Macro") {
    invoke("start_vow_macro");
    vowStartButton.textContent = "Stop Macro";
  } else if (vowStartButton.textContent == "Stop Macro") {
    invoke("stop_vow_macro");
    vowStartButton.textContent = "Start Macro";
  }
})

listen("vow-window-closed", () => {
  vowStartButton.textContent = "Start Macro";
})


