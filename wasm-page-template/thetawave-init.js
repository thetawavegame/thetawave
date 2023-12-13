import init from "./thetawave.js";

// prevent right click
document.addEventListener("contextmenu", (event) =>
    event.preventDefault(),
);

// Chrome and firefox do not want sound starting unless there is a user gesture. Hence, we start the game onClick
const initGameButton = document.getElementById("trigger_game_button");

initGameButton.onclick = () => {
  // Permit the browser to play audio
  const audioContext = new (window.AudioContext ||
      window.webkitAudioContext)();
  // Check if the AudioContext is suspended
  if (audioContext.state === "suspended") {
    audioContext.resume().then(() => {
      console.log("AudioContext resumed successfully.");
    });
  }
  // Hide the screen so we can play the game.
  const screenElements = Array.from(document.getElementsByClassName("screen"));

  screenElements.forEach((element) => {
    element.style.display = "none";
  });

  init();
};
