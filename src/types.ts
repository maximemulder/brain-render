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

export function getCoordinate(orientation: NiftiSliceOrientation, point: NiftiPoint3D): number {
  switch (orientation) {
    case NiftiSliceOrientation.Axial:
      return point.z;
    case NiftiSliceOrientation.Coronal:
      return point.y;
    case NiftiSliceOrientation.Sagittal:
      return point.x;
  }
}

export function setCoordinate(orientation: NiftiSliceOrientation, point: NiftiPoint3D, coordinate: number): NiftiPoint3D {
  switch (orientation) {
    case NiftiSliceOrientation.Axial:
      return { ...point, z: coordinate };
    case NiftiSliceOrientation.Coronal:
      return { ...point, y: coordinate }
    case NiftiSliceOrientation.Sagittal:
      return { ...point, x: coordinate }
  }
}

export function getMaxCoordinate(orientation: NiftiSliceOrientation, properties: NiftiProperties): number {
  switch (orientation) {
    case NiftiSliceOrientation.Axial:
      return properties.slices;
    case NiftiSliceOrientation.Coronal:
      return properties.columns;
    case NiftiSliceOrientation.Sagittal:
      return properties.rows;
  }
}
