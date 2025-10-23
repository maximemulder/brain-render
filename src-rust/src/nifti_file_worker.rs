use std::cell::RefCell;

use nifti::{InMemNiftiVolume, NiftiObject, ReaderStreamedOptions};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::File;

use crate::{log, nifti_slice::Nifti2DSlice};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct NiftiPoint3D {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct NiftiProperties {
    pub rows:    u16,
    pub columns: u16,
    pub slices:  u16,
}

pub struct NiftiWorkerState {
    properties: NiftiProperties,
    slices: Vec<InMemNiftiVolume>,
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

    let mut slices = Vec::new();
    let mut slices_counter = 0;
    while volume.slices_left() != 0 {
        slices.push(volume.read_slice().expect("Could not read slice."));
        slices_counter += 1;
    }

    log!("Read {} NIfTI slices.", slices_counter);

    STATE.replace(Some(NiftiWorkerState {
        slices,
        properties,
    }));

    properties
}

pub fn send_file(focal_point: NiftiPoint3D) -> JsValue {
    STATE.with_borrow(|state| {
        let Some(state) = state else {
            return JsValue::NULL;
        };

        Nifti2DSlice::from_volume(&state.slices[focal_point.z as usize]).to_js()
    })
}
