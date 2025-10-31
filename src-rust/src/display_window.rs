use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct DisplayWindow {
    pub level: f32,
    pub width: f32,
    pub polarity: DisplayPolarity,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum DisplayPolarity {
    Positive = 0,
    Negative = 1,
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
}
