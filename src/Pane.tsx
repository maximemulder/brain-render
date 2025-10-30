import { getCoordinate, getDimension, setCoordinate, ViewerState } from "./types";
import { worker } from "./App";
import { useCallback, useEffect, useRef } from "react";
import { clamp } from "./util";

export default function Pane({state, setState}: {
  state: ViewerState,
  setState: React.Dispatch<React.SetStateAction<ViewerState | null>>,
}) {
  const wrapperRef = useRef<HTMLDivElement>(null);

  // TODO: Clean render request strategy.
  worker.postMessage({
    action: 'render-slice',
    axis: state.axis,
    coordinate: getCoordinate(state.focalPoint, state.axis),
    window: state.window,
  });

  const canvasRef = useCallback((canvasRef: HTMLCanvasElement | null) => {
    if (canvasRef === null) {
      return;
    }

    const offscreen = canvasRef.transferControlToOffscreen();
    worker.postMessage({
      action: 'init-renderer',
      canvas: offscreen
    }, [offscreen]);

    // Send a render request immediatly to not have an empty screen.
    // FIXME: Do something cleaner.
    setTimeout(() => {
      worker.postMessage({
        action: 'render-slice',
        axis: state.axis,
        coordinate: getCoordinate(state.focalPoint, state.axis),
        window: state.window,
      });
    }, 100);
  }, []);

  useEffect(() => {
    const wrapper = wrapperRef.current;
    if (wrapper === null) {
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
    wrapper.addEventListener('wheel', handleWheel, { passive: false });

    // Cleanup function to remove the event listener
    return () => {
      wrapper.removeEventListener('wheel', handleWheel);
    };
  }, [state, setState]);

  return (
    <div ref={wrapperRef} style={{width: 'fit-content', height: 'fit-content'}}>
      <canvas
        id="canvas"
        ref={canvasRef}
        width={600}
        height={600}
      />
    </div>
  );
}
