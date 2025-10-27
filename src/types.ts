export type ViewerState = {
  properties: NiftiProperties
  orientation: NiftiSliceOrientation
  focalPoint: NiftiPoint3D
}

export type NiftiPoint3D = {
  x: number,
  y: number,
  z: number,
}

export type NiftiProperties = {
  rows:    number,
  columns: number,
  slices:  number,
}

export enum NiftiSliceOrientation {
  Axial    = 'Axial',
  Coronal  = 'Coronal',
  Sagittal = 'Sagittal',
}

export function createViewerState(properties: NiftiProperties): ViewerState {
  return {
    properties,
    orientation: NiftiSliceOrientation.Axial,
    focalPoint: {
      x: properties.rows    / 2,
      y: properties.columns / 2,
      z: properties.slices  / 2,
    },
  };
}
