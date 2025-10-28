import wasm, { init_graphics } from "../src-rust/pkg/brain_render_backend";
import { NiftiSliceOrientation } from "./types";
import { worker } from "./App";
import { useEffect, useRef } from "react";
import { clamp } from "./util";

export default function Pane({orientation, coordinate, maxCoordinate, setCoordinate}: {
  orientation: NiftiSliceOrientation,
  coordinate: number,
  maxCoordinate: number,
  setCoordinate: (coordinate: number) => void,
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
    orientation,
    coordinate,
  })

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) {
      return;
    }

    const handleWheel = (event: WheelEvent) => {
      event.preventDefault();

      const delta = Math.sign(event.deltaY); // -1 for scroll up, 1 for scroll down

      const newCoordinate = coordinate - delta; // Invert so scroll up increases, scroll down decreases

      const clampedCoordiante = clamp(0, maxCoordinate - 1, newCoordinate);

      setCoordinate(clampedCoordiante);
    };

    // Add the event listener
    canvas.addEventListener('wheel', handleWheel, { passive: false });

    // Cleanup function to remove the event listener
    return () => {
      canvas.removeEventListener('wheel', handleWheel);
    };
  }, [coordinate, maxCoordinate, setCoordinate]);

  return (
    <canvas
      id="canvas"
      ref={canvasRef}
    />
  );
}
