import { useEffect, useRef } from "react";
import { NiftiPoint3D, NiftiSliceOrientation, ViewerState } from "./types";
import { clamp } from "./util";

export default function Controls({state, setState}: {state: ViewerState, setState: (state: ViewerState) => void}) {
  const handleOrientationChange = (orientation: NiftiSliceOrientation) => (_: React.MouseEvent<HTMLButtonElement>) => {
    setState({
      ...state,
      orientation
    });
  };

  const handleFocalPointChange = (axis: keyof NiftiPoint3D) => (event: React.ChangeEvent<HTMLInputElement>) => {
    updateFocalPoint(axis, parseInt(event.target.value));
  };

  const updateFocalPoint = (axis: keyof NiftiPoint3D, value: number) => {
    setState({
      ...state,
      focalPoint: {
        ...state.focalPoint,
        [axis]: value,
      }
    });
  };

  return (
    <div>
      <div style={{ marginBottom: '1rem' }}>
        <button
          onClick={handleOrientationChange(NiftiSliceOrientation.Axial)}
          style={{
            fontWeight: state.orientation === NiftiSliceOrientation.Axial ? 'bold' : 'normal',
            margin: '0 0.25rem',
            padding: '0.5rem 1rem'
          }}
        >
          Axial
        </button>
        <button
          onClick={handleOrientationChange(NiftiSliceOrientation.Coronal)}
          style={{
            fontWeight: state.orientation === NiftiSliceOrientation.Coronal ? 'bold' : 'normal',
            margin: '0 0.25rem',
            padding: '0.5rem 1rem'
          }}
        >
          Coronal
        </button>
        <button
          onClick={handleOrientationChange(NiftiSliceOrientation.Sagittal)}
          style={{
            fontWeight: state.orientation === NiftiSliceOrientation.Sagittal ? 'bold' : 'normal',
            margin: '0 0.25rem',
            padding: '0.5rem 1rem'
          }}
        >
          Sagittal
        </button>
      </div>
      <Slider
        id="rows-slider"
        name="Rows"
        max={state.properties.rows - 1}
        value={state.focalPoint.x}
        onChange={handleFocalPointChange('x')}
      />
      <Slider
        id="columns-slider"
        name="Columns"
        max={state.properties.columns - 1}
        value={state.focalPoint.y}
        onChange={handleFocalPointChange('y')}
      />
      <Slider
        id="slices-slider"
        name="Slices"
        max={state.properties.slices - 1}
        value={state.focalPoint.z}
        onChange={handleFocalPointChange('z')}
      />
    </div>
  );
}

function Slider({id, name, max, value, onChange}: {
  id: string,
  name: string,
  max: number,
  value: number,
  onChange: (event: React.ChangeEvent<HTMLInputElement>) => void,
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

      const clampedValue = clamp(0, max - 1, newValue);

      // Only update if the value actually changed
      if (clampedValue !== value) {
        // Create a synthetic React change event
        const syntheticEvent = {
          target: {
            value: clampedValue.toString(),
          }
        } as React.ChangeEvent<HTMLInputElement>;

        onChange(syntheticEvent);
      }
    };

    console.log("Add event");

    input.addEventListener('wheel', handleWheel, { passive: false });

    return () => {
      input.removeEventListener('wheel', handleWheel);
    };
  }, [value, max, onChange]);

  return (
    <div>
      <label htmlFor={id}>{name}: {value}</label>
      <input
        ref={inputRef}
        id={id}
        type="range"
        min={0}
        max={max - 1}
        value={value}
        onChange={onChange}
      />
    </div>
  );
}
