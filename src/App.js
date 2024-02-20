import './App.css';
import logo from './icon.png';

import { invoke } from "@tauri-apps/api";

function App() {
  return (
    <div className="app" onLoad={() => { invoke("hello_world"); }}>
      <img className="herta-logo" src={logo} alt="Project Herta Logo" />
      <h1>Herta needs to get stuff!</h1>
      <p>Miss Herta needs to collect information before everything starts</p>
      <progress id="first-run" value="0"></progress>
      <p id="first-run-label">Currently on:</p>
    </div>
  );
}

export default App;
