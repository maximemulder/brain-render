use ndarray::{ShapeBuilder};
use nifti::{NiftiObject, ReaderStreamedOptions};
use web_sys::File;
use nifti::volume::ndarray::IntoNdArray;

use crate::nifti::Nifti;

pub async fn read_file(file: File) -> Nifti {
    crate::debug!("[file-reader] reading the nifti file");
    let nifti = ReaderStreamedOptions::new().read_web_file(file).expect("Cannot read NIfTI");
    let mut volume_reader = nifti.into_volume();
    let dimensions = volume_reader.dim();

   // Pre-allocate a 3D array for the entire volume
    let mut volume = ndarray::Array3::<f32>::zeros((
        dimensions[0] as usize,
        dimensions[1] as usize,
        dimensions[2] as usize,
    ).f());

    let mut slices_counter = 0;
    while volume_reader.slices_left() != 0 {
        let slice = volume_reader.read_slice().expect("Could not read slice");

        // Convert the slice to a 2D array and insert it into the 3D volume
        let slice_array = slice.into_ndarray::<f32>()
            .expect("Could not convert slice to ndarray")
            .into_dimensionality::<ndarray::Ix2>()
            .expect("Could not convert slice to 2D array");

        // Copy the slice data into the appropriate position in the 3D volume
        volume.slice_mut(ndarray::s![.., .., slices_counter])
            .assign(&slice_array);

        slices_counter += 1;
    }

    crate::log!("[file-reader] read {} nifti slices", slices_counter);

    Nifti {
        volume,
    }
}
