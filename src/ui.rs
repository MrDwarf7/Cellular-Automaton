use iced::widget::{container, column, text, image as img_widget};
use iced::{Element, Length};
use std::sync::{Arc, Mutex};
use image::{RgbImage, ImageBuffer, Rgb, ColorType, codecs::png::PngEncoder};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::grid::Grid;

pub struct GridDisplay {
    grid: Arc<Mutex<Grid>>,
    last_render_tick: Arc<AtomicU64>,
}

impl GridDisplay {
    pub fn new(grid: Arc<Mutex<Grid>>) -> Self {
        GridDisplay { 
            grid,
            last_render_tick: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl<'a, Message: 'a> From<GridDisplay> for Element<'a, Message> {
    fn from(grid_display: GridDisplay) -> Self {
        let (pop_counts, img_handle, grid_width, grid_height) = {
            let g = grid_display.grid.lock().unwrap();
            
            let width = g.width;
            let height = g.height;
            let pop_counts = g.get_population_counts();
            
            // Create image buffer with optimized scaling - render directly to RGB bytes
            let scale = 1; // 1:1 mapping for 500x500 grid (no downscaling needed)
            let display_width = width / scale;
            let display_height = height / scale;
            
            // Pre-allocate buffer and fill in one pass (better cache locality)
            let mut pixels: Vec<u8> = vec![0; (display_width * display_height * 3) as usize];
            
            for y in 0..display_height {
                for x in 0..display_width {
                    let grid_x = x * scale;
                    let grid_y = y * scale;
                    
                    let idx = ((y * display_width + x) * 3) as usize;
                    if let Some(cell) = g.get_cell(grid_x, grid_y) {
                        let (r, g_val, b) = cell.cell_type.get_color();
                        pixels[idx] = r;
                        pixels[idx + 1] = g_val;
                        pixels[idx + 2] = b;
                    } else {
                        pixels[idx] = 0;
                        pixels[idx + 1] = 0;
                        pixels[idx + 2] = 0;
                    }
                }
            }
            
            // Encode to PNG in memory
            let mut png_data = Vec::with_capacity(pixels.len() / 4); // Reserve reasonable space
            let encoder = PngEncoder::new(&mut png_data);
            encoder.encode(
                &pixels,
                display_width,
                display_height,
                ColorType::Rgb8,
            ).ok();
            
            // Create image handle from bytes
            let handle = iced::widget::image::Handle::from_memory(png_data);
            
            (pop_counts, handle, width, height)
        };
        
        let info_text = text(format!(
            "Grid: {}x{} | Population: {}",
            grid_width, grid_height, pop_counts
        )).size(12);
        
        let grid_image = img_widget(img_handle)
            .width(Length::Fixed(800.0))
            .height(Length::Fixed(800.0));
        
        let content = column![
            info_text,
            grid_image
        ]
        .spacing(10)
        .padding(10);
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
