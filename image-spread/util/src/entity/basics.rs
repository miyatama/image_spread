/**
 * 点や線などの基本的なデータを定義
 */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Line {
    p1: Point,
    p2: Point,
}
