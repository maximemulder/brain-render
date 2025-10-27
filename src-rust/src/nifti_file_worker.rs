use std::cell::RefCell;

use ndarray::ShapeBuilder;
use nifti::{NiftiObject, ReaderStreamedOptions};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::File;
use nifti::volume::ndarray::IntoNdArray;

use crate::{log, nifti_slice::Nifti2DSlice};

pub struct NiftiWorkerState {
    properties: NiftiProperties,
    volume: ndarray::Array3<f32>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct NiftiProperties {
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
pub enum SliceOrientation {
    Axial,    // XY plane (constant Z)
    Coronal,  // XZ plane (constant Y)
    Sagittal, // YZ plane (constant X)
}

thread_local! {
    static STATE: RefCell<Option<NiftiWorkerState>> = RefCell::new(None);
}

pub async fn read_file(file: File) -> NiftiProperties {
    log!("Starting to read the NIfTI file.");
    let nifti = ReaderStreamedOptions::new().read_web_file(file).expect("Cannot read NIfTI");
    let mut volume = nifti.into_volume();
    let dimensions = volume.dim();
    let properties = NiftiProperties {
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

    STATE.replace(Some(NiftiWorkerState {
        volume: volume_array,
        properties,
    }));

    properties
}

impl NiftiWorkerState {
    pub fn get_slice(&self, slice_index: usize, orientation: SliceOrientation) -> Nifti2DSlice {
        match orientation {
            SliceOrientation::Axial => self.get_axial_slice(slice_index),
            SliceOrientation::Coronal => self.get_coronal_slice(slice_index),
            SliceOrientation::Sagittal => self.get_sagittal_slice(slice_index),
        }
    }

    pub fn get_axial_slice(&self, slice_index: usize) -> Nifti2DSlice {
        let width = self.volume.shape()[0];
        let height = self.volume.shape()[1];

        // Axial: XY plane at constant Z
        let data = self.volume.slice(ndarray::s![.., .., slice_index]).to_owned();

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
        // We need to reverse the axes to get proper orientation for display
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
        // We need to reverse the axes to get proper orientation for display
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
    pub fn get_coordinate(self, orientation: SliceOrientation) -> u16 {
        match orientation {
            SliceOrientation::Axial    => self.z,
            SliceOrientation::Coronal  => self.y,
            SliceOrientation::Sagittal => self.x,
        }
    }
}

pub fn send_file(focal_point: NiftiPoint3D, orientation: SliceOrientation) -> JsValue {
    STATE.with_borrow(|state| {
        let Some(state) = state else {
            return JsValue::NULL;
        };

        state.get_slice(focal_point.get_coordinate(orientation) as usize, orientation).to_js()
    })
}
