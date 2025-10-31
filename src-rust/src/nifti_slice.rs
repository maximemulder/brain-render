use ndarray::Array2;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

pub struct Nifti2DSlice {
    pub width:  u16,
    pub height: u16,
    pub data:   Array2<f32>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct DisplayWindow {
    pub level: f32,
    pub width: f32,
    pub polarity: DisplayPolarity,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum DisplayPolarity {
    Positive,
    Negative,
}

impl DisplayWindow {
    /// Get the minimum value of this display window.
    pub fn min(&self) -> f32 {
        self.level - self.width / 2.0
    }

    /// Get the maximum value of this display window.
    pub fn max(&self) -> f32 {
        self.level + self.width / 2.0
    }

    /// Get the GPU [min, max] vector of this display window.
    pub fn vec(&self) -> [f32; 2] {
        match self.polarity {
            DisplayPolarity::Positive => {
                [self.min(), self.max()]
            },
            DisplayPolarity::Negative => {
                [self.max(), self.min()]
            },
        }
    }
}

impl Nifti2DSlice {
    pub fn to_js(&self) -> JsValue {
        let obj = js_sys::Object::new();

        js_sys::Reflect::set(
            &obj,
            &"width".into(),
            &JsValue::from(self.width),
        ).unwrap();

        js_sys::Reflect::set(
            &obj,
            &"height".into(),
            &JsValue::from(self.height)
        ).unwrap();

        let (vec, _) = self.data.clone().into_raw_vec_and_offset();

        let array = js_sys::Uint8Array::from(unsafe {std::slice::from_raw_parts(
            vec.as_ptr() as *const u8,
            vec.len() * std::mem::size_of::<f32>(),
        )});

        js_sys::Reflect::set(
            &obj,
            &"data".into(),
            &array,
        ).unwrap();

        obj.into()
    }

    pub fn from_js(js_value: &JsValue) -> Result<Nifti2DSlice, JsValue> {
        let width = js_sys::Reflect::get(js_value, &"width".into())
            .map_err(|_| "Missing 'width' property")?
            .as_f64()
            .ok_or("'width' should be a number")? as u16;

        let height = js_sys::Reflect::get(js_value, &"height".into())
            .map_err(|_| "Missing 'height' property")?
            .as_f64()
            .ok_or("'height' should be a number")? as u16;

        let data_array = js_sys::Reflect::get(js_value, &"data".into())
            .map_err(|_| "Missing 'data' property")?;

        let uint8_array = js_sys::Uint8Array::new(&data_array);

        // Convert Uint8Array back to f32 array
        let byte_len = uint8_array.length() as usize;
        if byte_len % std::mem::size_of::<f32>() != 0 {
            return Err("Data length must be multiple of f32 size".into());
        }

        let f32_len = byte_len / std::mem::size_of::<f32>();
        let mut buffer = vec![0f32; f32_len];

        unsafe {
            uint8_array.raw_copy_to_ptr(
                buffer.as_mut_ptr() as *mut u8,
            );
        }

        // Create ndarray from the buffer
        let data = Array2::from_shape_vec((height as usize, width as usize), buffer)
            .map_err(|e| JsValue::from_str(&format!("Invalid data shape: {}", e)))?;

        Ok(Nifti2DSlice {
            width,
            height,
            data,
        })
    }
}
