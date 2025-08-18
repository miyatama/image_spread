use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ImageInfo {
    pub grid_width: u32,
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub cells: Vec<ImageGridCell>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ImageGridCell {
    pub cell_x: u32,
    pub cell_y: u32,
    pub image_x1: u32,
    pub image_y1: u32,
    pub image_x2: u32,
    pub image_y2: u32,
    pub has_valid_pixel: bool,
}
