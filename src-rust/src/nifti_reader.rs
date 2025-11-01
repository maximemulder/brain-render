use ndarray::{ShapeBuilder};
use nifti::{NiftiObject, ReaderStreamedOptions};
use web_sys::File;
use nifti::volume::ndarray::IntoNdArray;

use crate::nifti::Nifti;

pub async fn read_file(file: File) -> Nifti {
    crate::debug!("[file-reader] reading the nifti file");
    let nifti = ReaderStreamedOptions::new().read_web_file(file).expect("Cannot read NIfTI");
    let mut volume_reader = nifti.into_volume();
    let dimensions = volume_reader.dim().to_owned();

    crate::log!("nifti dimensions: {:?}", dimensions);

    /* let timepoints = dimensions.get(3).copied().unwrap_or(1);

   // Pre-allocate a 3D array for the entire volume
    let mut volume = ndarray::Array4::<f32>::zeros((
        dimensions[0] as usize,
        dimensions[1] as usize,
        dimensions[2] as usize,
        timepoints as usize,
    ).f());

    let mut slices_counter = 0;
    let total_slices = dimensions[2] as usize * timepoints as usize;  // slices × timepoints

    while volume_reader.slices_left() != 0 {
        let slice = volume_reader.read_slice().expect("Could not read slice");

        // Convert the slice to a 2D array
        let slice_original_array = slice.into_ndarray::<f32>()
            .expect("Could not convert slice to ndarray");

        crate::log!("slice dimensions: {:?}", slice_original_array.dim());

        let slice_array = slice_original_array.into_dimensionality::<ndarray::Ix2>()
            .expect("Could not convert slice to 2D array");

        // Calculate 3D and 4D indices from linear slice counter
        let timepoint = slices_counter / dimensions[2] as usize;
        let slice_in_timepoint = slices_counter % dimensions[2] as usize;

        // Copy the slice data into the appropriate position in the 4D volume
        volume.slice_mut(ndarray::s![.., .., slice_in_timepoint, timepoint])
            .assign(&slice_array);

        slices_counter += 1;
    }

    crate::log!("[file-reader] read {} nifti slices across {} timepoints",
                slices_counter, timepoints);

    Nifti {
        volume,
    } */

       // Handle both 3D and 4D files
    match dimensions.len() {
        3 => {
            // 3D file: dimensions are [x, y, z]
            let mut volume = ndarray::Array4::<f32>::zeros((
                dimensions[0] as usize,
                dimensions[1] as usize,
                dimensions[2] as usize,
                1, // Single timepoint for 3D data
            ).f());

            let mut slice_counter = 0;
            while volume_reader.slices_left() != 0 {
                let slice = volume_reader.read_slice().expect("Could not read slice");

                // For 3D files, slices are 2D (x, y)
                let slice_array = slice.into_ndarray::<f32>()
                    .expect("Could not convert slice to ndarray")
                    .into_dimensionality::<ndarray::Ix2>()
                    .expect("Could not convert slice to 2D array");

                // Copy to volume at timepoint 0
                volume.slice_mut(ndarray::s![.., .., slice_counter, 0])
                    .assign(&slice_array);

                slice_counter += 1;
            }

            crate::log!("[file-reader] read {} 2D slices from 3D volume", slice_counter);
            Nifti { volume }
        }
        4 => {
            // 4D file: dimensions are [x, y, z, t]
            let mut volume = ndarray::Array4::<f32>::zeros((
                dimensions[0] as usize,
                dimensions[1] as usize,
                dimensions[2] as usize,
                dimensions[3] as usize,
            ).f());

            let mut volume_counter = 0;
            while volume_reader.slices_left() != 0 {
                let slice = volume_reader.read_slice().expect("Could not read slice");

                // For 4D files, slices are 3D (x, y, z)
                let slice_array = slice.into_ndarray::<f32>()
                    .expect("Could not convert slice to ndarray")
                    .into_dimensionality::<ndarray::Ix3>()
                    .expect("Could not convert slice to 3D array");

                // Calculate timepoint and volume index
                let timepoint = volume_counter; // / dimensions[2] as usize;
                // let volume_index = volume_counter % dimensions[2] as usize;

                // Copy the 3D slice to the appropriate position
                volume.slice_mut(ndarray::s![.., .., .., timepoint])
                    .assign(&slice_array);

                volume_counter += 1;
            }

            crate::log!("[file-reader] read {} 3D volumes from 4D data across {} timepoints",
                       volume_counter, dimensions[3]);
            Nifti { volume }
        }
        n => panic!("Unsupported number of dimensions: {}", n),
    }
}
