import wasm, { init_graphics } from "../src-rust/pkg/brain_render_backend";
import { ViewerState } from "./types";
import { worker } from "./App";

export default function Pane({state}: {state: ViewerState}) {
  worker.onmessage = async (message: MessageEvent<any>) => {
    await wasm();
    init_graphics(message.data.slice);
  };

  worker.postMessage({
    action: 'send-file',
    focalPoint: state.focalPoint,
  })

  return (
    <canvas id="canvas" />
  );
}
