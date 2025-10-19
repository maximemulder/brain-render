use nifti::{InMemNiftiVolume, NiftiVolume};
use crate::log;

pub struct NiftiSlice {
    pub width: u16,
    pub height: u16,
    pub data: Vec<f32>,
}

pub fn get_slice_from_volume(volume: InMemNiftiVolume) -> NiftiSlice {
    log!("VOLUME: {:?}", volume);
    log!("DIM: {:?}", volume.dim());
    let width  = volume.dim()[0];
    let height = volume.dim()[1];
    let data: Vec<f32> = volume.raw_data().chunks(4)
        .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();
    NiftiSlice { width, height, data }
}
