use crate::entity::basics::{Line, Point};
use serde::{Deserialize, Serialize};

/**
 * 画像を解析した情報
 * grid_width: 画像の分割幅
 * path: 画像ファイルパス
 * width: 画像の幅
 * height: 画像の高さ
 * cells: グリッドで分割したセル
 * cells_width: セルのX方向の個数
 * cells_height: セルのX方向の個数
 * cell_blocks: 有効なセルの塊
 */
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ImageInfo {
    pub grid_width: u32,
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub cells: Vec<ImageGridCell>,
    pub cells_width: u32,
    pub cells_height: u32,
    pub cell_blocks: Vec<CellBlock>,
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CellBlock {
    pub has_cell: Vec<Point>,
    pub lines: Vec<Line>,
}
