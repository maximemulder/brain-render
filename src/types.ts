export type NiftiProperties = {
  dimensions: ImageDimensions,
  maximum: number,
}

export type ViewerState = {
  rendererInitialied: boolean,
  dimensions: ImageDimensions,
  focalPoint: ImagePoint,
  axis: AnatomicalAxis,
  window: DisplayWindow,
  rotation: Rotation,
}

export type ImageDimensions = {
  rows:       number,
  columns:    number,
  slices:     number,
  timepoints: number,
}

export type ImageDimension = keyof ImageDimensions;

export type ImagePoint = {
  x: number,
  y: number,
  z: number,
  t: number,
}

export type ImageAxis = keyof ImagePoint;

export enum AnatomicalAxis {
  Axial    = 'Axial',
  Coronal  = 'Coronal',
  Sagittal = 'Sagittal',
}

export type DisplayWindow = {
  maximum: number,
  level: number,
  width: number,
  polarity: DisplayPolarity,
}

export enum DisplayPolarity {
  Positive = 'Positive',
  Negative = 'Negative',
}

export enum Rotation {
  Rotate0   = 'Rotate0',
  Rotate90  = 'Rotate90',
  Rotate180 = 'Rotate180',
  Rotate270 = 'Rotate270',
}

export function incrementRotation(rotation: Rotation): Rotation {
  switch (rotation) {
    case Rotation.Rotate0:
      return Rotation.Rotate90;
    case Rotation.Rotate90:
      return Rotation.Rotate180;
    case Rotation.Rotate180:
      return Rotation.Rotate270;
    case Rotation.Rotate270:
      return Rotation.Rotate0;
  }
}

export function decrementRotation(rotation: Rotation): Rotation {
  switch (rotation) {
    case Rotation.Rotate0:
      return Rotation.Rotate270;
    case Rotation.Rotate90:
      return Rotation.Rotate0;
    case Rotation.Rotate180:
      return Rotation.Rotate90;
    case Rotation.Rotate270:
      return Rotation.Rotate180;
  }
}

export function createViewerState({dimensions, maximum}: NiftiProperties): ViewerState {
  return {
    rendererInitialied: false,
    dimensions,
    axis: AnatomicalAxis.Axial,
    focalPoint: {
      x: Math.round(dimensions.rows    / 2),
      y: Math.round(dimensions.columns / 2),
      z: Math.round(dimensions.slices  / 2),
      t: 0,
    },
    window: {
      maximum,
      level: Math.round(maximum * 0.25),
      width: Math.round(maximum * 0.5),
      polarity: DisplayPolarity.Positive,
    },
    rotation: Rotation.Rotate0,
  };
}

export function getVoxelDimension(axis: AnatomicalAxis): ImageDimension {
  switch (axis) {
    case AnatomicalAxis.Axial:
      return 'slices';
    case AnatomicalAxis.Coronal:
      return 'columns';
    case AnatomicalAxis.Sagittal:
      return 'rows';
  }
}

export function getVoxelAxis(axis: AnatomicalAxis): ImageAxis {
  switch (axis) {
    case AnatomicalAxis.Axial:
      return 'z';
    case AnatomicalAxis.Coronal:
      return 'y';
    case AnatomicalAxis.Sagittal:
      return 'x';
  }
}

export function getDimension(dimensions: ImageDimensions, axis: AnatomicalAxis): number {
  return dimensions[getVoxelDimension(axis)];
}

export function getCoordinate(point: ImagePoint, axis: AnatomicalAxis): number {
  return point[getVoxelAxis(axis)];
}

export function setCoordinate(point: ImagePoint, coordinate: number, axis: AnatomicalAxis): ImagePoint {
  return {...point, [getVoxelAxis(axis)]: coordinate}
}

export function invertPolarity(polarity: DisplayPolarity): DisplayPolarity {
  switch (polarity) {
    case DisplayPolarity.Positive:
      return DisplayPolarity.Negative;
    case DisplayPolarity.Negative:
      return DisplayPolarity.Positive;
  }
}
