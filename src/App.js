import './App.css';
import logo from './icon.png';

import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

function App() {
  return (
    <div className="app" >
      <img onLoad={() => { invoke("begin_first_run") }} className="herta-logo" src={logo} alt="Project Herta Logo" />
      <h1>Herta needs to get stuff!</h1>
      <p>Miss Herta needs to collect information before everything starts.</p>
      <p className="warn">This will take ~10 minutes, but will never happen again</p>
      <progress id="first-run" value="0"></progress>
      <p id="first-run-label"></p>
    </div>
  );
}

listen("download-progress", (e) => {
  let { current_progress, message } = e.payload;
  let progress_bar = document.getElementById("first-run");
  let progress_label = document.getElementById("first-run-label");

  progress_bar.value = current_progress;
  progress_label.innerText = message
})

listen("start-progress", (e) => {
  let { total } = e.payload;
  let progress_bar = document.getElementById("first-run");

  progress_bar.setAttribute("max", `${total}`);
})

export default App;
