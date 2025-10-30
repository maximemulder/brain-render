import "./App.css";
import {ChangeEvent, useState} from "react";
import NiftiFileWorker from './worker?worker';
import Controls from "./Controls";
import { createViewerState, ViewerState } from "./types";
import Pane from "./Pane";

/** Web worker that handles the loading and reading of NIfTI files. */
export const worker = new NiftiFileWorker();

function App() {
  let [state, setState] = useState<ViewerState | null>(null);

  async function handleFileChange(e: ChangeEvent<HTMLInputElement>) {
    if (e.target.files === null) {
      return;
    }

    worker.onmessage = (event: MessageEvent<any>) => {
      if (event.data.action === 'read-file') {
        setState(createViewerState(event.data.properties))
      }
    }

    worker.postMessage({action: 'read-file', file: e.target.files[0]})
  }

  async function handleLoadDemo() {
    try {
      const response = await fetch('./assets/demo.nii');
      const blob = await response.blob();

      // Create a File object
      const demoFile = new File([blob], 'demo.nii', { type: 'application/octet-stream' });

      const syntheticEvent = {
        target: {
          files: [demoFile]
        }
      } as any;

      // Reuse the existing handler
      handleFileChange(syntheticEvent);

    } catch (error) {
      console.error('Failed to load demo file:', error);
    }
  }

  return (
    <div id="app">
      <header id="header">
        <h1 className="app-title">Brain Render</h1>
      </header>
      {state !== null ?
        <main className="viewer">
          <Pane state={state} setState={setState} />
          <Controls state={state} setState={setState} />
        </main>
        : null
      }
      <div className="file-loaders">
        <div className="file-loader">
          <label htmlFor="demo-file-input">Load custom file</label>
          <input id="demo-file-input" type="file" onChange={handleFileChange} />
        </div>
        <div className="file-loader">
          <button type="button" onClick={handleLoadDemo}>Load demonstration file</button>
        </div>
      </div>
    </div>
  );
}

export default App;
