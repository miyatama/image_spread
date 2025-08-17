use crate::repositories::ImageInfoRepository;
use image::GenericImageView;
use infra::FileSystem;
use util::AppResult;
use util::{ImageGridCell, ImageInfo};

pub struct ImageInfoRepositoryImpl<'r, T: FileSystem> {
    file_system: &'r T,
}

impl<'r, T: FileSystem> ImageInfoRepositoryImpl<'r, T> {
    pub fn new(file_system: &'r T) -> Self {
        Self {
            file_system: file_system,
        }
    }
}

impl<'r, T: FileSystem> ImageInfoRepository for ImageInfoRepositoryImpl<'r, T> {
    fn parse(&self, path: String, grid_width: u32) -> AppResult<ImageInfo> {
        let img = self.file_system.open_image_file(path.clone()).unwrap();
        let (width, height) = img.dimensions();
        let x_cell_size = width / grid_width;
        let y_cell_size = height / grid_width;
        let x_cell_size = if (x_cell_size * grid_width) < width {
            x_cell_size + 1
        } else {
            x_cell_size
        };
        let y_cell_size = if (y_cell_size * grid_width) < height {
            y_cell_size + 1
        } else {
            y_cell_size
        };
        let mut cells = vec![];
        for x in 0..x_cell_size {
            for y in 0..y_cell_size {
                let x1 = x * grid_width;
                let y1 = y * grid_width;
                let x2 = if (x1 + grid_width) > width {
                    width - 1
                } else {
                    x1 + grid_width - 1
                };
                let y2 = if (y1 + grid_width) > height {
                    height - 1
                } else {
                    y1 + grid_width - 1
                };
                cells.push(ImageGridCell {
                    cell_x: x,
                    cell_y: y,
                    image_x1: x1,
                    image_y1: y1,
                    image_x2: x2,
                    image_y2: y2,
                });
            }
        }
        Ok(ImageInfo {
            path: path,
            grid_width: grid_width,
            cells: cells,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::DynamicImage;
    use infra::MockFileSystem;

    #[test]
    fn test_split_to_grid() {
        let width = 10u32;
        let height = 10u32;
        let result_pixels = vec![100u8; width as usize * height as usize * 4usize];
        let result_img = image::ImageBuffer::from_raw(width, height, result_pixels).unwrap();
        let mock_image_file = DynamicImage::ImageRgba8(result_img);
        let mut mock_file_system = MockFileSystem::new();
        mock_file_system
            .expect_open_image_file()
            .times(1)
            .return_once_st(move |_| Ok(mock_image_file));
        let repository = ImageInfoRepositoryImpl::new(&mock_file_system);
        let result = repository.parse("path".to_string(), 5);
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        let expect = ImageInfo {
            path: "path".to_string(),
            grid_width: 5,
            cells: vec![
                ImageGridCell {
                    cell_x: 0,
                    cell_y: 0,
                    image_x1: 0,
                    image_y1: 0,
                    image_x2: 4,
                    image_y2: 4,
                },
                ImageGridCell {
                    cell_x: 1,
                    cell_y: 0,
                    image_x1: 5,
                    image_y1: 0,
                    image_x2: 9,
                    image_y2: 4,
                },
                ImageGridCell {
                    cell_x: 0,
                    cell_y: 1,
                    image_x1: 0,
                    image_y1: 5,
                    image_x2: 4,
                    image_y2: 9,
                },
                ImageGridCell {
                    cell_x: 1,
                    cell_y: 1,
                    image_x1: 5,
                    image_y1: 5,
                    image_x2: 9,
                    image_y2: 9,
                },
            ],
        };
        assert_eq!(result.path, expect.path);
        assert_eq!(result.grid_width, expect.grid_width);
        assert_eq!(result.cells.len(), expect.cells.len());
        for cell in result.cells {
            if let Some(expect_cell) = expect.cells.iter().find(|expect_cell| expect_cell.cell_x == cell.cell_x && expect_cell.cell_y == cell.cell_y) {
                assert_eq!(cell, expect_cell.clone());
            } else {
                panic!("invalid actual cell ({}, {})", cell.cell_x, cell.cell_y);
            }
        }
    }

    #[test]
    fn test_split_to_grid_oversize() {
        let width = 12u32;
        let height = 12u32;
        let result_pixels = vec![100u8; width as usize * height as usize * 4usize];
        let result_img = image::ImageBuffer::from_raw(width, height, result_pixels).unwrap();
        let mock_image_file = DynamicImage::ImageRgba8(result_img);
        let mut mock_file_system = MockFileSystem::new();
        mock_file_system
            .expect_open_image_file()
            .times(1)
            .return_once_st(move |_| Ok(mock_image_file));
        let repository = ImageInfoRepositoryImpl::new(&mock_file_system);
        let result = repository.parse("path".to_string(), 5);
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        let expect = ImageInfo {
            path: "path".to_string(),
            grid_width: 5,
            cells: vec![
                ImageGridCell {
                    cell_x: 0,
                    cell_y: 0,
                    image_x1: 0,
                    image_y1: 0,
                    image_x2: 4,
                    image_y2: 4,
                },
                ImageGridCell {
                    cell_x: 1,
                    cell_y: 0,
                    image_x1: 5,
                    image_y1: 0,
                    image_x2: 9,
                    image_y2: 4,
                },
                ImageGridCell {
                    cell_x: 2,
                    cell_y: 0,
                    image_x1: 10,
                    image_y1: 0,
                    image_x2: 11,
                    image_y2: 4,
                },
                ImageGridCell {
                    cell_x: 0,
                    cell_y: 1,
                    image_x1: 0,
                    image_y1: 5,
                    image_x2: 4,
                    image_y2: 9,
                },
                ImageGridCell {
                    cell_x: 1,
                    cell_y: 1,
                    image_x1: 5,
                    image_y1: 5,
                    image_x2: 9,
                    image_y2: 9,
                },
                ImageGridCell {
                    cell_x: 2,
                    cell_y: 1,
                    image_x1: 10,
                    image_y1: 5,
                    image_x2: 11,
                    image_y2: 9,
                },
                ImageGridCell {
                    cell_x: 0,
                    cell_y: 2,
                    image_x1: 0,
                    image_y1: 10,
                    image_x2: 4,
                    image_y2: 11,
                },
                ImageGridCell {
                    cell_x: 1,
                    cell_y: 2,
                    image_x1: 5,
                    image_y1: 10,
                    image_x2: 9,
                    image_y2: 11,
                },
                ImageGridCell {
                    cell_x: 2,
                    cell_y: 2,
                    image_x1: 10,
                    image_y1: 10,
                    image_x2: 11,
                    image_y2: 11,
                },
            ],
        };
        assert_eq!(result.path, expect.path);
        assert_eq!(result.grid_width, expect.grid_width);
        assert_eq!(result.cells.len(), expect.cells.len());
        for cell in result.cells {
            if let Some(expect_cell) = expect.cells.iter().find(|expect_cell| expect_cell.cell_x == cell.cell_x && expect_cell.cell_y == cell.cell_y) {
                assert_eq!(cell, expect_cell.clone());
            } else {
                panic!("invalid actual cell ({}, {})", cell.cell_x, cell.cell_y);
            }
        }
    }

    #[test]
    fn test_split_to_grid_oversize_1() {
        let width = 11u32;
        let height = 11u32;
        let result_pixels = vec![100u8; width as usize * height as usize * 4usize];
        let result_img = image::ImageBuffer::from_raw(width, height, result_pixels).unwrap();
        let mock_image_file = DynamicImage::ImageRgba8(result_img);
        let mut mock_file_system = MockFileSystem::new();
        mock_file_system
            .expect_open_image_file()
            .times(1)
            .return_once_st(move |_| Ok(mock_image_file));
        let repository = ImageInfoRepositoryImpl::new(&mock_file_system);
        let result = repository.parse("path".to_string(), 5);
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        let expect = ImageInfo {
            path: "path".to_string(),
            grid_width: 5,
            cells: vec![
                ImageGridCell {
                    cell_x: 0,
                    cell_y: 0,
                    image_x1: 0,
                    image_y1: 0,
                    image_x2: 4,
                    image_y2: 4,
                },
                ImageGridCell {
                    cell_x: 1,
                    cell_y: 0,
                    image_x1: 5,
                    image_y1: 0,
                    image_x2: 9,
                    image_y2: 4,
                },
                ImageGridCell {
                    cell_x: 2,
                    cell_y: 0,
                    image_x1: 10,
                    image_y1: 0,
                    image_x2: 10,
                    image_y2: 4,
                },
                ImageGridCell {
                    cell_x: 0,
                    cell_y: 1,
                    image_x1: 0,
                    image_y1: 5,
                    image_x2: 4,
                    image_y2: 9,
                },
                ImageGridCell {
                    cell_x: 1,
                    cell_y: 1,
                    image_x1: 5,
                    image_y1: 5,
                    image_x2: 9,
                    image_y2: 9,
                },
                ImageGridCell {
                    cell_x: 2,
                    cell_y: 1,
                    image_x1: 10,
                    image_y1: 5,
                    image_x2: 10,
                    image_y2: 9,
                },
                ImageGridCell {
                    cell_x: 0,
                    cell_y: 2,
                    image_x1: 0,
                    image_y1: 10,
                    image_x2: 4,
                    image_y2: 10,
                },
                ImageGridCell {
                    cell_x: 1,
                    cell_y: 2,
                    image_x1: 5,
                    image_y1: 10,
                    image_x2: 9,
                    image_y2: 10,
                },
                ImageGridCell {
                    cell_x: 2,
                    cell_y: 2,
                    image_x1: 10,
                    image_y1: 10,
                    image_x2: 10,
                    image_y2: 10,
                },
            ],
        };
        assert_eq!(result.path, expect.path);
        assert_eq!(result.grid_width, expect.grid_width);
        assert_eq!(result.cells.len(), expect.cells.len());
        for cell in result.cells {
            if let Some(expect_cell) = expect.cells.iter().find(|expect_cell| expect_cell.cell_x == cell.cell_x && expect_cell.cell_y == cell.cell_y) {
                assert_eq!(cell, expect_cell.clone());
            } else {
                panic!("invalid actual cell ({}, {})", cell.cell_x, cell.cell_y);
            }
        }
    }
}
