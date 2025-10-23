use std::cell::RefCell;

use nifti::{InMemNiftiVolume, NiftiObject, ReaderStreamedOptions};
use wasm_bindgen::JsValue;
use web_sys::File;

use crate::{log, nifti_slice::Nifti2DSlice};

// NOTE: A web file cannot be sent between threads.
// In an ideal architecture, the file is kept in the web worker, and the main thread asynchronously
// calls the web worker whenever it needs to read the file.
// static NIFTI: Mutex<Option<GenericNiftiObject<StreamedNiftiVolume<Either<BufReader<WebSysFile>, GzDecoder<BufReader<WebSysFile>>>>>>> = Mutex::new(None);

thread_local! {
    static NIFTI_SLICE: RefCell<Option<InMemNiftiVolume>> = RefCell::new(None);
}

pub async fn read_file(file: File) {
    log!("Starting to read the NIfTI file.");
    let nifti = ReaderStreamedOptions::new().read_web_file(file).expect("Cannot read NIfTI");
    let mut volume = nifti.into_volume();
    for _ in 0..50 {
        volume.read_slice().expect("Could not read slice.");
    }

    match volume.read_slice() {
        Ok(slice) => {
            {
                NIFTI_SLICE.replace(Some(slice));
                log!("Successfully read NIfTI slice, slices left: {}", volume.slices_left());
            }
            NIFTI_SLICE.with_borrow(|slice| log!("Cell is some: {}", slice.is_some()));
        },
        Err(error) => {
            log!("Error while reading NIfTI slice: {:?}", error);
        },
    }
}

pub fn send_file() -> JsValue {
    NIFTI_SLICE.with_borrow(|slice| {
        let Some(slice) = slice else {
            return JsValue::NULL;
        };

        Nifti2DSlice::from_volume(&slice).to_js()
    })
}
