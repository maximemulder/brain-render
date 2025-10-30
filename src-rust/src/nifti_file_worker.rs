use std::cell::RefCell;

use ndarray::{Array3, ShapeBuilder};
use nifti::{NiftiObject, ReaderStreamedOptions};
use serde::{Deserialize, Serialize};
use web_sys::File;
use nifti::volume::ndarray::IntoNdArray;

use crate::{log, nifti_slice::Nifti2DSlice};

pub struct NiftiWorkerState {
    pub volume: ndarray::Array3<f32>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct NiftiProperies {
    pub dimensions: VoxelDimensions,
    pub maximum: f32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct VoxelDimensions {
    pub rows:    u16,
    pub columns: u16,
    pub slices:  u16,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct NiftiPoint3D {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum AnatomicalAxis {
    Axial    = 0,    // XY plane (constant Z)
    Coronal  = 1,  // XZ plane (constant Y)
    Sagittal = 2, // YZ plane (constant X)
}

thread_local! {
    pub static STATE: RefCell<Option<NiftiWorkerState>> = RefCell::new(None);
}

pub async fn read_file(file: File) -> NiftiProperies {
    log!("Starting to read the NIfTI file.");
    let nifti = ReaderStreamedOptions::new().read_web_file(file).expect("Cannot read NIfTI");
    let mut volume = nifti.into_volume();
    let dimensions = volume.dim();

    let voxel_dimensions = VoxelDimensions {
        rows:    dimensions[0],
        columns: dimensions[1],
        slices:  dimensions[2],
    };

   // Pre-allocate a 3D array for the entire volume
    let mut volume_array = ndarray::Array3::<f32>::zeros((
        dimensions[0] as usize,
        dimensions[1] as usize,
        dimensions[2] as usize,
    ).f());

    let mut slices_counter = 0;
    while volume.slices_left() != 0 {
        let slice = volume.read_slice().expect("Could not read slice");

        // Convert the slice to a 2D array and insert it into the 3D volume
        let slice_array = slice.into_ndarray::<f32>()
            .expect("Could not convert slice to ndarray")
            .into_dimensionality::<ndarray::Ix2>()
            .expect("Could not convert slice to 2D array");

        // Copy the slice data into the appropriate position in the 3D volume
        volume_array.slice_mut(ndarray::s![.., .., slices_counter])
            .assign(&slice_array);

        slices_counter += 1;
    }

    log!("Read {} NIfTI slices.", slices_counter);

    let maximum = get_max_data_value(&volume_array);

    STATE.replace(Some(NiftiWorkerState {
        volume: volume_array,
    }));

    NiftiProperies {
        dimensions: voxel_dimensions,
        maximum,
    }
}

fn get_max_data_value(array: &Array3<f32>) -> f32 {
    array.fold(0.0, |max, &x| max.max(x))
}

impl NiftiWorkerState {
    pub fn get_slice(&self, slice_index: usize, axis: AnatomicalAxis) -> Nifti2DSlice {
        match axis {
            AnatomicalAxis::Axial => self.get_axial_slice(slice_index),
            AnatomicalAxis::Coronal => self.get_coronal_slice(slice_index),
            AnatomicalAxis::Sagittal => self.get_sagittal_slice(slice_index),
        }
    }

    pub fn get_axial_slice(&self, slice_index: usize) -> Nifti2DSlice {
        let width = self.volume.shape()[0];
        let height = self.volume.shape()[1];

        // Axial: XY plane at constant Z
        let data = self.volume.slice(ndarray::s![.., .., slice_index])
            .reversed_axes()
            .to_owned();

        Nifti2DSlice {
            width: width as u16,
            height: height as u16,
            data
        }
    }

    pub fn get_coronal_slice(&self, slice_index: usize) -> Nifti2DSlice {
        let width = self.volume.shape()[0];
        let depth = self.volume.shape()[2];

        // Coronal: XZ plane at constant Y
        // We need to reverse the axes to get proper axis for display
        let data = self.volume.slice(ndarray::s![.., slice_index, ..])
            .reversed_axes()  // This makes it [X, Z] for proper display
            .to_owned();

        Nifti2DSlice {
            width: width as u16,
            height: depth as u16,
            data
        }
    }

    pub fn get_sagittal_slice(&self, slice_index: usize) -> Nifti2DSlice {
        let height = self.volume.shape()[1];
        let depth = self.volume.shape()[2];

        // Sagittal: YZ plane at constant X
        // We need to reverse the axes to get proper axis for display
        let data = self.volume.slice(ndarray::s![slice_index, .., ..])
            .reversed_axes()  // This makes it [Y, Z] for proper display
            .to_owned();

        Nifti2DSlice {
            width: height as u16,
            height: depth as u16,
            data
        }
    }
}

impl NiftiPoint3D {
    pub fn get_coordinate(self, axis: AnatomicalAxis) -> u16 {
        match axis {
            AnatomicalAxis::Axial    => self.z,
            AnatomicalAxis::Coronal  => self.y,
            AnatomicalAxis::Sagittal => self.x,
        }
    }
}
