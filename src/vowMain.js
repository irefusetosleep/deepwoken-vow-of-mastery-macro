///////vow macro js/////////////
import { iohook } from "iohook";
const commands = ["Live", "Sacrifice", "Fight", "Return", "Locate", "Summon", "Leech", "Run", "Explode"];
let cmdIndex = 0;

// Get the word display element and buttons
const cmdDisplay = document.getElementById("cmdDisplay");
const prevCmd = document.getElementById("prevCmd");
const nextCmd = document.getElementById("nextCmd");

function updateCmd() {
  cmdDisplay.textContent = "Command: " + commands[cmdIndex];
}

updateCmd();

iohook.on("keydown", (event) => {
  console.log("Key pressed:", event.keycode);
  // Map keycodes to actual keys if needed
});

iohook.start();

