export type ViewerState = {
  dimensions: VoxelDimensions,
  focalPoint: VoxelPoint,
  axis: AnatomicalAxis,
}

export type VoxelDimensions = {
  rows:    number,
  columns: number,
  slices:  number,
}

export type VoxelDimension = keyof VoxelDimensions;

export type VoxelPoint = {
  x: number,
  y: number,
  z: number,
}

export type VoxelAxis = keyof VoxelPoint;

export enum AnatomicalAxis {
  Axial    = 'Axial',
  Coronal  = 'Coronal',
  Sagittal = 'Sagittal',
}

export function createViewerState(dimensions: VoxelDimensions): ViewerState {
  return {
    dimensions: dimensions,
    axis: AnatomicalAxis.Axial,
    focalPoint: {
      x: dimensions.rows    / 2,
      y: dimensions.columns / 2,
      z: dimensions.slices  / 2,
    },
  };
}

export function getVoxelDimension(axis: AnatomicalAxis): VoxelDimension {
  switch (axis) {
    case AnatomicalAxis.Axial:
      return 'slices';
    case AnatomicalAxis.Coronal:
      return 'columns';
    case AnatomicalAxis.Sagittal:
      return 'rows';
  }
}

export function getVoxelAxis(axis: AnatomicalAxis): VoxelAxis {
  switch (axis) {
    case AnatomicalAxis.Axial:
      return 'z';
    case AnatomicalAxis.Coronal:
      return 'y';
    case AnatomicalAxis.Sagittal:
      return 'x';
  }
}

export function getDimension(dimensions: VoxelDimensions, axis: AnatomicalAxis): number {
  return dimensions[getVoxelDimension(axis)];
}

export function getCoordinate(point: VoxelPoint, axis: AnatomicalAxis): number {
  return point[getVoxelAxis(axis)];
}

export function setCoordinate(point: VoxelPoint, coordinate: number, axis: AnatomicalAxis): VoxelPoint {
  return {...point, [getVoxelAxis(axis)]: coordinate}
}
