import styles from "./HomePage.module.css";
import { invoke } from "@tauri-apps/api";
import { useState, useEffect } from "react";

let characters = await invoke("get_characters");

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