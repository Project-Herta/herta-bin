import styles from "./HomePage.module.css";
import { invoke } from "@tauri-apps/api";
import { useState, useEffect } from "react";

// For some god awful reason, the HomePage
// component renders on the DOM for 0.0000001
// microseconds even though a check for
// the first run is already done on the
// index.js page
if (await invoke("first_run_complete")) {
  var characters = await invoke("get_characters");
}

function HomePage() {
  return (
    <div className={styles.main}>
      <div className={styles.clist}>
        <h4 className="h4">Characters:</h4>
        <div className={styles.ccontainer}>
          {characters.map((char) => (<button key={char.id} onClick={() => { clicky(char) }}>{char.name}</button>))}
        </div>
      </div>
      <div>
        <h4>Enemies:</h4>
        <div></div>
      </div>
    </div>)
}

function clicky(data) {
  let content = document.getElementsByClassName(styles.main)[0];

  for (let i = content.children.length - 1; i >= 0; i--) {
    let child_node = content.children.item(i);

    // console.log({ "index": i, "value": child_node, "type": typeof child_node });
    content.removeChild(child_node);
  }

  // console.log(content)
  // console.log(data)
}

export default HomePage;