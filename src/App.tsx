// import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import init, {greet2, start_graphics} from "../brain-render-backend/pkg/brain_render_backend";

await init()

function App() {
  async function greet() {
    start_graphics();
  }

  return (
    <main className="container">
      <h1>Welcome to Brain-Render</h1>
      <canvas id="canvas"></canvas>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <button type="submit">Start</button>
      </form>
    </main>
  );
}

export default App;
