import "./App.css";
import {ChangeEvent} from "react";
import {init_graphics} from "../src-rust/pkg/brain_render_backend";
import NiftiFileWorker from './worker.ts?worker';

/** Web worker that handles the loading and reading of NIfTI files. */
const worker = new NiftiFileWorker();

function App() {
  async function handleStart() {
    init_graphics(worker);
  }

  async function handleFileChange(e: ChangeEvent<HTMLInputElement>) {
    if (e.target.files === null) {
      return;
    }

    worker.postMessage({action: 'read-file', file: e.target.files[0]})
  }

  return (
    <main className="container">
      <h1>Welcome to Brain-Render</h1>
      <canvas id="canvas"></canvas>
        <button type="submit" onClick={handleStart}>Start</button>
        <input id="file" type="file" onChange={handleFileChange} />
    </main>
  );
}

export default App;
