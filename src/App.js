import './App.css';
import logo from './icon.png';

import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

function App() {
  return (
    <div className="app" >
      <img className="herta-logo" src={logo} alt="Project Herta Logo" />
      <h1>Herta needs to get stuff!</h1>
      <p>Miss Herta needs to collect information before everything starts</p>
      <progress id="first-run" max="10" value="0"></progress>
      <p id="first-run-label"></p>
      <button onClick={() => invoke("test_progress")}>Click Me!</button>
    </div>
  );
}

listen("download-progress", (e) => {
  let { progress, message } = e.payload;
  let progress_bar = document.getElementById("first-run");
  let progress_label = document.getElementById("first-run-label");

  progress_bar.value += progress;
  progress_label.innerText = `Currently on: ${message};`
})

export default App;
