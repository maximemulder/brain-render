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
      <div>
        <label htmlFor="rows-slider">Rows: {state.properties.rows}</label>
        <input
          id="rows-slider"
          type="range"
          min={0}
          max={state.properties.rows - 1}
          value={state.focalPoint.x}
          onChange={handleFocalPointChange('x')}
        />
      </div>

      <div>
        <label htmlFor="columns-slider">Columns: {state.properties.columns}</label>
        <input
          id="columns-slider"
          type="range"
          max={state.properties.columns - 1}
          value={state.focalPoint.y}
          onChange={handleFocalPointChange('y')}
        />
      </div>

      <div>
        <label htmlFor="slices-slider">Slices: {state.properties.slices}</label>
        <input
          id="slices-slider"
          type="range"
          min="0"
          max={state.properties.slices - 1}
          value={state.focalPoint.z}
          onChange={handleFocalPointChange('z')}
        />
      </div>
    </div>
  );
}
