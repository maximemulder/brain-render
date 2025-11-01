import { useEffect, useRef } from "react";
import { AnatomicalAxis, ViewerState, getDimension, getCoordinate, setCoordinate, DisplayPolarity, invertPolarity } from "./types";
import { clamp } from "./util";

export default function Controls({state, setState}: {
  state: ViewerState,
  setState: React.Dispatch<React.SetStateAction<ViewerState | null>>,
}) {
  return (
    <div className="controls">
      <PolarityButton state={state} setState={setState} />
      <Slider
        id="window-level-slider"
        name="Window level (brightness)"
        value={state.window.level}
        max={state.window.maximum}
        update={(level) => setState({...state, window: {...state.window, level }})}
      />
      <Slider
        id="window-width-slider"
        name="Window width (contrast)"
        value={state.window.width}
        max={state.window.maximum}
        update={(width) => setState({...state, window: {...state.window, width }})}
      />
      <div className="axis-buttons">
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
      {state.dimensions.timepoints !== 1 && (
        <Slider
          id="time-slider"
          name="Time"
          value={state.focalPoint.t}
          max={state.dimensions.timepoints}
          update={(value) => setState({...state, focalPoint: {...state.focalPoint, t: value}})}
        />
      )}
    </div>
  );
}

function PolarityButton({state, setState}: {
  state: ViewerState,
  setState: React.Dispatch<React.SetStateAction<ViewerState | null>>,
}) {
  const handleClick = () => {
    setState({
      ...state,
      window: {
        ...state.window,
        polarity: invertPolarity(state.window.polarity),
      }
    })
  };

  return (
    <button onClick={handleClick}>
      {state.window.polarity === DisplayPolarity.Positive ? 'Positive' : 'Negative'}
    </button>
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
      className="axis-button"
      onClick={() => handleClick()}
      style={{fontWeight: state.axis === axis ? 'bold' : 'normal'}}
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
  const id    = getAxisId(axis);
  const name  = getAxisName(axis);
  const value = getCoordinate(state.focalPoint, axis);
  const max   = getDimension(state.dimensions, axis) - 1;

  function updateCoordinate(value: number) {
    setState({
      ...state,
      axis,
      focalPoint: setCoordinate(state.focalPoint, value, axis),
    });
  }

  return (
    <Slider
      id={id}
      name={name}
      value={value}
      max={max}
      update={updateCoordinate}
    />
  );
}

function Slider({id, name, value, max, update}: {
  id: string,
  name: string,
  value: number,
  max: number,
  update: (value: number) => void,
}) {
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const input = inputRef.current;
    if (input === null) {
      return;
    }

    const handleWheel = (event: WheelEvent) => {
      event.preventDefault();

      const delta = Math.sign(event.deltaY); // -1 for scroll up, 1 for scroll down
      const newValue = value - delta; // Invert so scroll up increases, scroll down decreases

      const clampedValue = clamp(0, max, newValue);

      // Only update if the value actually changed
      if (clampedValue !== value) {
        update(clampedValue);
      }
    };

    input.addEventListener('wheel', handleWheel, { passive: false });

    return () => {
      input.removeEventListener('wheel', handleWheel);
    };
  }, [value, max, update]);

  function handleChange(event: React.ChangeEvent<HTMLInputElement>) {
    update(parseInt(event.target.value));
  };

  return (
    <div className="control-slider">
      <label htmlFor={id}>{name}: {value}</label>
      <input
        ref={inputRef}
        id={id}
        type="range"
        min={0}
        max={max}
        value={value}
        onChange={handleChange}
      />
    </div>
  );
}

function getAxisId(axis: AnatomicalAxis): string {
  switch (axis) {
    case AnatomicalAxis.Axial:
      return 'axial-slider';
    case AnatomicalAxis.Coronal:
      return 'coronal-slider';
    case AnatomicalAxis.Sagittal:
      return 'sagittal-slider';
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
