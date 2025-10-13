// import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import {init_graphics, read_file} from "../brain-render-backend/pkg/brain_render_backend";

function App() {
  async function init() {
    init_graphics();
  }

  async function handleFileChange(e) {
    console.log(e.target.files)
    read_file(e.target.files[0])
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
