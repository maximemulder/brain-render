use ndarray::ShapeBuilder;
use nifti::{NiftiObject, ReaderStreamedOptions};
use web_sys::File;
use nifti::volume::ndarray::IntoNdArray;

use crate::nifti::Nifti;

pub async fn read_nifti_file(file: File) -> Nifti {
    crate::debug!("[file-reader] reading the nifti file");
    let nifti = ReaderStreamedOptions::new().read_web_file(file).expect("Cannot read NIfTI");
    let mut volume_reader = nifti.into_volume();
    let dimensions = volume_reader.dim().to_owned();
    let is_4d = dimensions.get(3).is_some();
    let timepoints = if is_4d {
        crate::debug!("[file-reader] found 3d nifti file");
        dimensions[3] as usize
    } else {
        crate::debug!("[file-reader] found 4d nifti file");
        1
    };

    let dimensions = ndarray::Ix4(
        dimensions[0] as usize,
        dimensions[1] as usize,
        dimensions[2] as usize,
        timepoints,
    );

    let mut volume = ndarray::Array4::<f32>::zeros(dimensions.f());
    let mut slice_counter = 0;
    while volume_reader.slices_left() != 0 {
        let slice = volume_reader.read_slice().expect("Could not read slice");
        let slice_array = slice.into_ndarray::<f32>()
            .expect("Could not convert slice to ndarray");

        if is_4d {
            volume.slice_mut(ndarray::s![.., .., .., slice_counter]).assign(&slice_array);
        } else {
            volume.slice_mut(ndarray::s![.., .., slice_counter, 0]).assign(&slice_array);
        }

        slice_counter += 1;
    }

    crate::debug!("[file-reader] read {} nifti slices", slice_counter);
    Nifti { volume }
}
