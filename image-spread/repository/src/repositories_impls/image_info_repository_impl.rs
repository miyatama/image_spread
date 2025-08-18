use crate::repositories::ImageInfoRepository;
use image::GenericImageView;
use infra::FileSystem;
use util::AppResult;
use util::Error::{InvalidImageInfoError, SaveFileError};
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
        let image_rgb = img.clone().to_rgba8();
        let alphas = (0..(width * height))
            .map(|index| {
                let y = index / width;
                let x = index - y * width;
                let pixel = image_rgb.get_pixel(x, y);
                pixel[3]
            })
            .collect::<Vec<u8>>();
        // println!("alphas: {:?}", alphas.clone());
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
                let mut max_alpha = 0u8;
                let mut last_max_changed_index = 0;
                let mut last_max_changed_x = 0;
                let mut last_max_changed_y = 0;
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        let index = (y * grid_width + x) as usize;
                        // TODO alpha配列使うように変更したい
                        let alpha = image_rgb.get_pixel(x, y)[3];
                        if alpha > max_alpha {
                            max_alpha = alpha;
                            last_max_changed_index = index;
                            last_max_changed_x = x;
                            last_max_changed_y = y;
                        }
                    }
                }

                let has_valid_pixel = max_alpha > 0u8;
                cells.push(ImageGridCell {
                    cell_x: x,
                    cell_y: y,
                    image_x1: x1,
                    image_y1: y1,
                    image_x2: x2,
                    image_y2: y2,
                    has_valid_pixel: has_valid_pixel,
                });
            }
        }
        Ok(ImageInfo {
            path: path,
            grid_width: grid_width,
            width: width,
            height: height,
            cells: cells,
        })
    }

    fn write_grid_image(&self, image_info: ImageInfo) -> AppResult<()> {
        let width = image_info.width;
        let height = image_info.height;
        let grid_width = image_info.grid_width;
        let mut image = image::RgbImage::new(width, height);
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
        let blue = image::Rgb([0u8, 0u8, 255u8]);
        let white = image::Rgb([255u8, 255u8, 255u8]);
        imageproc::drawing::draw_filled_rect_mut(
            &mut image,
            imageproc::rect::Rect::at(0i32, 0i32).of_size(width, height),
            white,
        );
        for x in 0..x_cell_size {
            for y in 0..y_cell_size {
                if let Some(cell) = image_info
                    .cells
                    .iter()
                    .find(|cell| cell.cell_x == x && cell.cell_y == y)
                {
                    if cell.has_valid_pixel {
                        imageproc::drawing::draw_filled_rect_mut(
                            &mut image,
                            imageproc::rect::Rect::at(cell.image_x1 as i32, cell.image_y1 as i32)
                                .of_size(
                                    cell.image_x2 - cell.image_x1,
                                    cell.image_y2 - cell.image_y1,
                                ),
                            blue,
                        );
                    }
                } else {
                    return Err(InvalidImageInfoError(format!(
                        "cell not found({}, {})",
                        x, y
                    )));
                }
            }
        }
        let image: image::DynamicImage = image.into();
        match self
            .file_system
            .save_ebidence_file("grid_image.png".to_string(), image)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(SaveFileError(format!("{}", e))),
        }
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
        let result_pixels = vec![255u8; width as usize * height as usize * 4usize];
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
            width: width,
            height: height,
            cells: vec![
                generate_image_grid_cell(0, 0, 0, 0, 4, 4, true),
                generate_image_grid_cell(1, 0, 5, 0, 9, 4, true),
                generate_image_grid_cell(0, 1, 0, 5, 4, 9, true),
                generate_image_grid_cell(1, 1, 5, 5, 9, 9, true),
            ],
        };
        assert_eq!(result.path, expect.path);
        assert_eq!(result.grid_width, expect.grid_width);
        assert_eq!(result.cells.len(), expect.cells.len());
        for cell in result.cells {
            if let Some(expect_cell) = expect.cells.iter().find(|expect_cell| {
                expect_cell.cell_x == cell.cell_x && expect_cell.cell_y == cell.cell_y
            }) {
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
        let result_pixels = vec![255u8; width as usize * height as usize * 4usize];
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
            width: width,
            height: height,
            cells: vec![
                generate_image_grid_cell(0, 0, 0, 0, 4, 4, true),
                generate_image_grid_cell(1, 0, 5, 0, 9, 4, true),
                generate_image_grid_cell(2, 0, 10, 0, 11, 4, true),
                generate_image_grid_cell(0, 1, 0, 5, 4, 9, true),
                generate_image_grid_cell(1, 1, 5, 5, 9, 9, true),
                generate_image_grid_cell(2, 1, 10, 5, 11, 9, true),
                generate_image_grid_cell(0, 2, 0, 10, 4, 11, true),
                generate_image_grid_cell(1, 2, 5, 10, 9, 11, true),
                generate_image_grid_cell(2, 2, 10, 10, 11, 11, true),
            ],
        };
        assert_eq!(result.path, expect.path);
        assert_eq!(result.grid_width, expect.grid_width);
        assert_eq!(result.cells.len(), expect.cells.len());
        for cell in result.cells {
            if let Some(expect_cell) = expect.cells.iter().find(|expect_cell| {
                expect_cell.cell_x == cell.cell_x && expect_cell.cell_y == cell.cell_y
            }) {
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
        let result_pixels = vec![0u8; width as usize * height as usize * 4usize];
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
            width: width,
            height: height,
            cells: vec![
                generate_image_grid_cell(0, 0, 0, 0, 4, 4, false),
                generate_image_grid_cell(1, 0, 5, 0, 9, 4, false),
                generate_image_grid_cell(2, 0, 10, 0, 10, 4, false),
                generate_image_grid_cell(0, 1, 0, 5, 4, 9, false),
                generate_image_grid_cell(1, 1, 5, 5, 9, 9, false),
                generate_image_grid_cell(2, 1, 10, 5, 10, 9, false),
                generate_image_grid_cell(0, 2, 0, 10, 4, 10, false),
                generate_image_grid_cell(1, 2, 5, 10, 9, 10, false),
                generate_image_grid_cell(2, 2, 10, 10, 10, 10, false),
            ],
        };
        assert_eq!(result.path, expect.path);
        assert_eq!(result.grid_width, expect.grid_width);
        assert_eq!(result.cells.len(), expect.cells.len());
        for cell in result.cells {
            if let Some(expect_cell) = expect.cells.iter().find(|expect_cell| {
                expect_cell.cell_x == cell.cell_x && expect_cell.cell_y == cell.cell_y
            }) {
                assert_eq!(cell, expect_cell.clone());
            } else {
                panic!("invalid actual cell ({}, {})", cell.cell_x, cell.cell_y);
            }
        }
    }

    #[test]
    fn test_split_to_grid_valid_pixel() {
        // 3x3ピク中央のみピクセルあり
        let width = 3u32;
        let height = 3u32;
        let result_pixels = vec![
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, // 1行目
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, // 2行目
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, // 3行目
        ];
        let result_img = image::ImageBuffer::from_raw(width, height, result_pixels).unwrap();
        let mock_image_file = DynamicImage::ImageRgba8(result_img);
        let mut mock_file_system = MockFileSystem::new();
        mock_file_system
            .expect_open_image_file()
            .times(1)
            .return_once_st(move |_| Ok(mock_image_file));
        let repository = ImageInfoRepositoryImpl::new(&mock_file_system);
        let result = repository.parse("path".to_string(), 1);
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        let expect = ImageInfo {
            path: "path".to_string(),
            grid_width: 1,
            width: width,
            height: height,
            cells: vec![
                generate_image_grid_cell(0, 0, 0, 0, 0, 0, false),
                generate_image_grid_cell(1, 0, 1, 0, 1, 0, false),
                generate_image_grid_cell(2, 0, 2, 0, 2, 0, false),
                generate_image_grid_cell(0, 1, 0, 1, 0, 1, false),
                generate_image_grid_cell(1, 1, 1, 1, 1, 1, true),
                generate_image_grid_cell(2, 1, 2, 1, 2, 1, false),
                generate_image_grid_cell(0, 2, 0, 2, 0, 2, false),
                generate_image_grid_cell(1, 2, 1, 2, 1, 2, false),
                generate_image_grid_cell(2, 2, 2, 2, 2, 2, false),
            ],
        };
        assert_eq!(result.path, expect.path);
        assert_eq!(result.grid_width, expect.grid_width);
        assert_eq!(result.cells.len(), expect.cells.len());
        for cell in result.cells {
            if let Some(expect_cell) = expect.cells.iter().find(|expect_cell| {
                expect_cell.cell_x == cell.cell_x && expect_cell.cell_y == cell.cell_y
            }) {
                assert_eq!(cell, expect_cell.clone());
            } else {
                panic!("invalid actual cell ({}, {})", cell.cell_x, cell.cell_y);
            }
        }
    }

    #[test]
    fn test_split_to_grid_valid_4x4_checker() {
        // 4x4のチェッカーマーク
        let width = 4u32;
        let height = 4u32;
        let result_pixels = vec![
            // 1行目
            vec![0u8, 0u8, 0u8, 0u8],
            vec![0u8, 0u8, 0u8, 0u8],
            vec![0u8, 0u8, 0u8, 1u8],
            vec![0u8, 0u8, 0u8, 1u8],
            // 2行目
            vec![0u8, 0u8, 0u8, 0u8],
            vec![0u8, 0u8, 0u8, 0u8],
            vec![0u8, 0u8, 0u8, 1u8],
            vec![0u8, 0u8, 0u8, 1u8],
            // 3行目
            vec![0u8, 0u8, 0u8, 1u8],
            vec![0u8, 0u8, 0u8, 1u8],
            vec![0u8, 0u8, 0u8, 0u8],
            vec![0u8, 0u8, 0u8, 0u8],
            // 4行目
            vec![0u8, 0u8, 0u8, 1u8],
            vec![0u8, 0u8, 0u8, 1u8],
            vec![0u8, 0u8, 0u8, 0u8],
            vec![0u8, 0u8, 0u8, 0u8],
        ];
        let result_pixels = result_pixels.into_iter().flatten().collect::<Vec<u8>>();
        let result_img = image::ImageBuffer::from_raw(width, height, result_pixels).unwrap();
        let mock_image_file = DynamicImage::ImageRgba8(result_img);
        let mut mock_file_system = MockFileSystem::new();
        mock_file_system
            .expect_open_image_file()
            .times(1)
            .return_once_st(move |_| Ok(mock_image_file));
        let repository = ImageInfoRepositoryImpl::new(&mock_file_system);
        let result = repository.parse("path".to_string(), 2);
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        let expect = ImageInfo {
            path: "path".to_string(),
            grid_width: 2,
            width: width,
            height: height,
            cells: vec![
                generate_image_grid_cell(0, 0, 0, 0, 1, 1, false),
                generate_image_grid_cell(1, 0, 2, 0, 3, 1, true),
                generate_image_grid_cell(0, 1, 0, 2, 1, 3, true),
                generate_image_grid_cell(1, 1, 2, 2, 3, 3, false),
            ],
        };
        assert_eq!(result.path, expect.path);
        assert_eq!(result.grid_width, expect.grid_width);
        assert_eq!(result.cells.len(), expect.cells.len());
        for cell in result.cells {
            if let Some(expect_cell) = expect.cells.iter().find(|expect_cell| {
                expect_cell.cell_x == cell.cell_x && expect_cell.cell_y == cell.cell_y
            }) {
                assert_eq!(cell, expect_cell.clone());
            } else {
                panic!("invalid actual cell ({}, {})", cell.cell_x, cell.cell_y);
            }
        }
    }

    fn generate_image_grid_cell(
        cell_x: u32,
        cell_y: u32,
        image_x1: u32,
        image_y1: u32,
        image_x2: u32,
        image_y2: u32,
        has_valid_pixel: bool,
    ) -> ImageGridCell {
        ImageGridCell {
            cell_x,
            cell_y,
            image_x1,
            image_y1,
            image_x2,
            image_y2,
            has_valid_pixel,
        }
    }
}
