import { NiftiPoint3D, ViewerState } from "./types";

export default function Controls({state, setState}: {state: ViewerState, setState: (state: ViewerState) => void}) {
  const updateFocalPoint = (axis: keyof NiftiPoint3D, value: number) => {
    setState({
      ...state,
      focalPoint: {
        ...state.focalPoint,
        [axis]: value,
      }
    });
  };

  const handleFocalPointChange = (axis: keyof NiftiPoint3D) => (event: React.ChangeEvent<HTMLInputElement>) => {
    updateFocalPoint(axis, parseInt(event.target.value));
  };

  return (
    <div>
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
  return (
    <div>
      <label htmlFor={id}>{name}: {value}</label>
      <input
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
