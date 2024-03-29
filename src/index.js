import React from 'react';
import ReactDOM from 'react-dom/client';
import reportWebVitals from './reportWebVitals';
import { invoke } from "@tauri-apps/api";

import './index.module.css';
import FirstRunScreen from './FirstRun';
import HomePage from './HomePage';

let first_run_complete = await invoke("first_run_complete");

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    {first_run_complete ? <HomePage /> : <FirstRunScreen />}
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
