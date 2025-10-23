import wasm, { init_graphics } from "../src-rust/pkg/brain_render_backend";
import { NiftiPoint3D, ViewerState } from "./types";
import { worker } from "./App";

export default function Pane({state, setState}: {state: ViewerState, setState: (state: ViewerState) => void}) {
  worker.onmessage = async (message: MessageEvent<any>) => {
    await wasm();
    init_graphics(message.data.slice);
  };

  worker.postMessage({
    action: 'send-file',
    focalPoint: state.focalPoint,
  })

  const updateFocalPoint = (axis: keyof NiftiPoint3D, value: number) => {
    setState({
      ...state,
      focalPoint: {
        ...state.focalPoint,
        [axis]: value,
      }
    });
  };

  const handleWheel = (event: React.WheelEvent<HTMLCanvasElement>) => {
    event.preventDefault();

    const delta = Math.sign(event.deltaY); // -1 for scroll up, 1 for scroll down
    const newZ = state.focalPoint.z - delta; // Invert so scroll up increases, scroll down decreases

    // Clamp the value between 0 and state.properties.slices
    const clampedZ = Math.max(0, Math.min(state.properties.slices - 1, newZ));

    // Only update if the value actually changed
    if (clampedZ !== state.focalPoint.z) {
      updateFocalPoint('z', clampedZ);
    }
  };

  return (
    <canvas
      id="canvas"
      onWheel={handleWheel}
    />
  );
}
