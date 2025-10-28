import { useEffect, useRef } from "react";
import { AnatomicalAxis, ViewerState, getDimension, getCoordinate, setCoordinate } from "./types";
import { clamp } from "./util";

export default function Controls({state, setState}: {
  state: ViewerState,
  setState: React.Dispatch<React.SetStateAction<ViewerState | null>>,
}) {
  return (
    <div>
      <div style={{ marginBottom: '1rem' }}>
        <AxisButton
          axis={AnatomicalAxis.Axial}
          state={state}
          setState={setState}
        />
        <AxisButton
          axis={AnatomicalAxis.Coronal}
          state={state}
          setState={setState}
        />
        <AxisButton
          axis={AnatomicalAxis.Sagittal}
          state={state}
          setState={setState}
        />
      </div>
      <AxisSlider
        axis={AnatomicalAxis.Axial}
        state={state}
        setState={setState}
      />
      <AxisSlider
        axis={AnatomicalAxis.Coronal}
        state={state}
        setState={setState}
      />
      <AxisSlider
        axis={AnatomicalAxis.Sagittal}
        state={state}
        setState={setState}
      />
    </div>
  );
}

function AxisButton({axis, state, setState}: {
  axis: AnatomicalAxis,
  state: ViewerState,
  setState: React.Dispatch<React.SetStateAction<ViewerState | null>>,
}) {
  function handleClick() {
    setState({
      ...state,
      axis,
    });
  };

  const name  = getAxisName(axis);
  return (
    <button
      onClick={() => handleClick()}
      style={{
        fontWeight: state.axis === axis ? 'bold' : 'normal',
        margin: '0 0.25rem',
        padding: '0.5rem 1rem'
      }}
    >
      {name}
    </button>
  );
}

function AxisSlider({axis, state, setState}: {
  axis: AnatomicalAxis,
  state: ViewerState,
  setState: React.Dispatch<React.SetStateAction<ViewerState | null>>,
}) {
  const inputRef = useRef<HTMLInputElement>(null);

  const id    = getAxisId(axis);
  const name  = getAxisName(axis);
  const value = getCoordinate(state.focalPoint, axis);
  const max   = getDimension(state.dimensions, axis);

  function updateCoordinate(value: number) {
    setState({
      ...state,
      focalPoint: setCoordinate(state.focalPoint, value, axis),
    });
  }

  function handleChange(event: React.ChangeEvent<HTMLInputElement>) {
    updateCoordinate(parseInt(event.target.value));
  };

  useEffect(() => {
    const input = inputRef.current;
    if (input === null) {
      return;
    }

    const handleWheel = (event: WheelEvent) => {
      event.preventDefault();

      const delta = Math.sign(event.deltaY); // -1 for scroll up, 1 for scroll down
      const newValue = value - delta; // Invert so scroll up increases, scroll down decreases

      const clampedValue = clamp(0, max - 1, newValue);

      // Only update if the value actually changed
      if (clampedValue !== value) {
        updateCoordinate(clampedValue);
      }
    };

    input.addEventListener('wheel', handleWheel, { passive: false });

    return () => {
      input.removeEventListener('wheel', handleWheel);
    };
  }, [value, max, updateCoordinate]);

  return (
    <div>
      <label htmlFor={id}>{name} slice: {value}</label>
      <input
        ref={inputRef}
        id={id}
        type="range"
        min={0}
        max={max - 1}
        value={value}
        onChange={handleChange}
      />
    </div>
  );
}

function getAxisId(axis: AnatomicalAxis): string {
  switch (axis) {
    case AnatomicalAxis.Axial:
      return 'axial';
    case AnatomicalAxis.Coronal:
      return 'coronal';
    case AnatomicalAxis.Sagittal:
      return 'sagittal';
  }
}

function getAxisName(axis: AnatomicalAxis): string {
  switch (axis) {
    case AnatomicalAxis.Axial:
      return 'Axial';
    case AnatomicalAxis.Coronal:
      return 'Coronal';
    case AnatomicalAxis.Sagittal:
      return 'Sagittal';
  }
}
