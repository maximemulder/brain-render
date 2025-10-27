import wasm, { init_graphics } from "../src-rust/pkg/brain_render_backend";
import { NiftiPoint3D, ViewerState } from "./types";
import { worker } from "./App";
import { useEffect, useRef } from "react";
import { clamp } from "./util";

export default function Pane({state, setState}: {state: ViewerState, setState: React.Dispatch<React.SetStateAction<ViewerState | null>>}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  worker.onmessage = async (message: MessageEvent<any>) => {
    if (canvasRef.current !== null) {
      await wasm();
      init_graphics(message.data.slice, canvasRef.current);
    }
  };

  worker.postMessage({
    action: 'send-file',
    focalPoint: state.focalPoint,
    orientation: state.orientation,
  })

  const updateFocalPoint = (axis: keyof NiftiPoint3D, value: number) => {
    setState((state) => {
      if (state === null) {
        return null
      }

      return {
        ...state,
        focalPoint: {
          ...state.focalPoint,
          [axis]: value,
        }
      }
    });
  };

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) {
      return;
    }

    const handleWheel = (event: WheelEvent) => {
      event.preventDefault();

      const delta = Math.sign(event.deltaY); // -1 for scroll up, 1 for scroll down
      const newZ = state.focalPoint.z - delta; // Invert so scroll up increases, scroll down decreases

      const clampedZ = clamp(0, state.properties.slices - 1, newZ);

      // Only update if the value actually changed
      updateFocalPoint('z', clampedZ);
    };

    // Add the event listener
    canvas.addEventListener('wheel', handleWheel, { passive: false });

    // Cleanup function to remove the event listener
    return () => {
      canvas.removeEventListener('wheel', handleWheel);
    };
  }, [state.focalPoint.z, state.properties.slices]); // Dependencies for the effect

  return (
    <canvas
      id="canvas"
      ref={canvasRef}
    />
  );
}
