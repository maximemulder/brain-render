import wasm, { init_graphics } from "../src-rust/pkg/brain_render_backend";
import { getCoordinate, getDimension, setCoordinate, ViewerState } from "./types";
import { worker } from "./App";
import { useEffect, useRef } from "react";
import { clamp } from "./util";

export default function Pane({state, setState}: {
  state: ViewerState,
  setState: React.Dispatch<React.SetStateAction<ViewerState | null>>,
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  worker.onmessage = async (message: MessageEvent<any>) => {
    if (canvasRef.current !== null) {
      await wasm();
      init_graphics(message.data.slice, canvasRef.current);
    }
  };

  worker.postMessage({
    action: 'send-file',
    axis: state.axis,
    coordinate: getCoordinate(state.focalPoint, state.axis),
  })

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) {
      return;
    }

    const handleWheel = (event: WheelEvent) => {
      event.preventDefault();

      const delta = Math.sign(event.deltaY); // -1 for scroll up, 1 for scroll down

      const newCoordinate = getCoordinate(state.focalPoint, state.axis) - delta; // Invert so scroll up increases, scroll down decreases

      const clampedCoordiante = clamp(0, getDimension(state.dimensions, state.axis) - 1, newCoordinate);

      setState({
        ...state,
        focalPoint: setCoordinate(state.focalPoint, clampedCoordiante, state.axis),
      })
    };

    // Add the event listener
    canvas.addEventListener('wheel', handleWheel, { passive: false });

    // Cleanup function to remove the event listener
    return () => {
      canvas.removeEventListener('wheel', handleWheel);
    };
  }, [state, setState]);

  return (
    <canvas
      id="canvas"
      ref={canvasRef}
    />
  );
}
