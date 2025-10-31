use crate::{nifti_file_worker::AnatomicalAxis, display_window::DisplayWindow};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FragmentParams {
    pub volume_dims: [f32; 3],
    pub polarity: u32,
    pub window: [f32; 2],
    pub axis: u32,
    pub slice_index: f32,
}

impl FragmentParams {
    pub fn new(
        volume_dimensions: [usize; 3],
        axis: AnatomicalAxis,
        slice_index: usize,
        window: DisplayWindow,
    ) -> Self {
        let dimension_length = match axis {
            AnatomicalAxis::Axial    => volume_dimensions[2],
            AnatomicalAxis::Coronal  => volume_dimensions[1],
            AnatomicalAxis::Sagittal => volume_dimensions[0],
        };

        let normalized_slice_index = slice_index as f32 / (dimension_length - 1) as f32;

        Self {
            volume_dims: [
                volume_dimensions[0] as f32,
                volume_dimensions[1] as f32,
                volume_dimensions[2] as f32,
            ],
            polarity: window.polarity as u32,
            window: [
                window.min(),
                window.max(),
            ],
            axis: axis as u32,
            slice_index: normalized_slice_index,
        }
    }
}
