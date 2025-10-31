use serde::{Deserialize, Serialize};

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
