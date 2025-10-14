import "./App.css";
import {ChangeEvent} from "react";
import {init_graphics} from "../src-rust/pkg/brain_render_backend";
import MyWorker from './worker.ts?worker';

const worker = new MyWorker();

function App() {
  async function init() {
    init_graphics();
  }

  async function handleFileChange(e: ChangeEvent<HTMLInputElement>) {
    if (e.target.files === null) {
      return;
    }

    worker.postMessage({file: e.target.files[0]})
  }

  return (
    <main className="container">
      <h1>Welcome to Brain-Render</h1>
      <canvas id="canvas"></canvas>
        <button type="submit" onClick={init}>Start</button>
        <input id="file" type="file" onChange={handleFileChange} />
    </main>
  );
}

export default App;
