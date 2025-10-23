use std::cell::RefCell;

use nifti::{InMemNiftiVolume, NiftiObject, ReaderStreamedOptions};
use wasm_bindgen::JsValue;
use web_sys::File;

use crate::{log, nifti_slice::Nifti2DSlice};

pub struct NiftiPoint3D {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

pub struct NiftiFileWorkerState {
    focal_point: NiftiPoint3D,
    volumes: Vec<InMemNiftiVolume>,
}

thread_local! {
    static STATE: RefCell<Option<NiftiFileWorkerState>> = RefCell::new(None);
}

pub async fn read_file(file: File) {
    log!("Starting to read the NIfTI file.");
    let nifti = ReaderStreamedOptions::new().read_web_file(file).expect("Cannot read NIfTI");
    let mut volume = nifti.into_volume();

    let mut volumes = Vec::new();
    let mut slices_counter = 0;
    while volume.slices_left() != 0 {
        volumes.push(volume.read_slice().expect("Could not read slice."));
        slices_counter += 1;
    }

    log!("Read {} NIfTI slices.", slices_counter);

    STATE.replace(Some(NiftiFileWorkerState {
        focal_point: NiftiPoint3D { x: 50, y: 50, z: 50 },
        volumes,
    }));
}

pub fn send_file() -> JsValue {
    STATE.with_borrow(|state| {
        let Some(state) = state else {
            return JsValue::NULL;
        };

        Nifti2DSlice::from_volume(&state.volumes[50]).to_js()
    })
}
