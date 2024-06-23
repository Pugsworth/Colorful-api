use std::{f32::INFINITY};

use rand::Rng;
use hex_color::HexColor;
use image::{Rgb};

mod color_spaces;
use crate::palette_generator::color_spaces::LABColor;




/*
    This file deals with generating the palette images.
*/


/*
 * What layout to put the color blocks into.
 */
#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum ColorBlockLayout {
    Linear, // A single vertical or horizontal line of colors. Works best for few amount of colors.
            // TODO: Can be either Vertical or Horizontal. How do I incorporate these into the
            // enum?
    Grid,   // A grid that attempts to keep the aspect ratio as close to 1:1 for rows and columns.
    Mason   // Similar to an image gallery. Some blocks will be wider or taller than others.
            // Intended for a more aesthetic look.
            // TODO: How to determine which blocks are important for different sizes?
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ColorSortOrder {
    None,
    Hue,
    Saturation,
    Lightness,
    Luminance,
}





fn get_layout_dimensions(color_count: u8, block_size: u16, layout: &ColorBlockLayout) -> (u16, u16) {
    match layout {
        ColorBlockLayout::Linear => {
            (color_count as u16 * block_size, block_size)
        },
        ColorBlockLayout::Grid => {
            let cols = (color_count as f64).sqrt().ceil() as u16;
            let rows = (color_count as f64 / cols as f64).ceil() as u16;

            (cols * block_size, rows * block_size)
        },

        _ => {
            return (0, 0);
        }
    }
}

//.Sort colors using a color map
fn sort_colors_colormap(colors: &Vec<HexColor>, color_map: colorous::Gradient) -> Vec<&HexColor> {
    const COLOR_MAP_SIZE: usize = 100;
    // Build the color map.
    let mut color_map: [HexColor; COLOR_MAP_SIZE] = [HexColor::BLACK; COLOR_MAP_SIZE];
    for i in 0..COLOR_MAP_SIZE {
        let col = colorous::TURBO.eval_rational(i, COLOR_MAP_SIZE);
        let hcol = HexColor { r:col.r, g:col.g, b:col.b, a:0xFF };
        color_map[i] = hcol;
    }

    fn calculate_distance(col_a: HexColor, col_b: HexColor) -> f32 {
        let r = col_a.r as f32 - col_b.r as f32;
        let g = col_a.g as f32 - col_b.g as f32;
        let b = col_a.b as f32 - col_b.b as f32;

        return (r * r + g * g + b * b).sqrt();
    }


    let mut temp: Vec<(&HexColor, i32)> = Vec::new();

    // Calculate the closest index of the color from the color map colors.
    for (_i, col) in colors.iter().enumerate() {
        let mut min_dist: f32 = INFINITY;
        let mut nearest_index: i32 = -1;

        // Find the nearest Turbo color and store the index of this color and the turbo map index.
        for (j, turbo_color) in color_map.iter().enumerate() {
            let distance = calculate_distance(*col, *turbo_color);

            if distance < min_dist {
                min_dist = distance;
                nearest_index = j as i32;
            }
        }

        temp.push((col, nearest_index));
    }

    // Sort by those indices
    temp.sort_by(|a, b| {
        (&b.1).partial_cmp(&a.1).unwrap()
    });

    // Does this make sense?
    let mut sorted_colors: Vec<&HexColor> = vec![];
    for (col, _) in temp.iter() {
        sorted_colors.push(*col);
    }

    return sorted_colors;
}


fn sort_colors(colors: &Vec<HexColor>) -> &Vec<HexColor> {
    fn distance_lab(col_a: LABColor, col_b: LABColor) {
        todo!();
    }

    fn distance_hue(col_a: HexColor, col_b: HexColor) {
        todo!();
    }

    return colors;
}




/*
 * Draw a block of pixels starting from `pos`
 */
fn draw_block(img: &mut image::RgbImage, color: HexColor, size: u16, pos: (u16, u16)) {
    let (x, y) = pos;
    let (r, g, b) = (color.r, color.g, color.b);
    let col = Rgb([r, g, b]);

    for i in 0..size {
        for j in 0..size {
            let px = (x + i) as u32;
            let py = (y + j) as u32;

            img.put_pixel(px, py, col)
        }
    }
}


/*
 * Generate a palette from a list of colors.
 */
pub fn image_from_colors(colors: &Vec<HexColor>, block_size: u16, layout: ColorBlockLayout) -> image::RgbImage {
    let col_len = u8::try_from(colors.len()).ok().unwrap();
    let (img_width, img_height) = get_layout_dimensions(col_len, block_size, &layout);

    println!("#colors: {}, block_size: {}, img size: ({}, {})", colors.len(), block_size, img_width, img_height);

    let mut img = image::ImageBuffer::new(img_width.into(), img_height.into());

    let sorted_colors = sort_colors(colors);

    match layout {
        ColorBlockLayout::Linear => {
            println!("Creating Linear Palette!");

            for (i, color) in sorted_colors.iter().enumerate() {
                let x = (i as u16) * block_size;
                let y = 0;

                // println!("image_from_colors: (i)(x, y) = ({}){:?}", i, (x, y));

                draw_block(&mut img, *color, block_size, (x, y));
            }
        }
        // TODO: This might require that sorting is done using a 2D sorting algorithm instead of a
        // 1D one so that it flows.
        ColorBlockLayout::Grid => {
            println!("Creating Grid Palette!");

            let width = (col_len as f32).sqrt().ceil();
            let _height = (col_len as f32 / width).ceil();

            for (i, color) in sorted_colors.iter().enumerate() {
                let x = (i as f32 % width) * block_size as f32;
                let y = (i as f32 / width).floor() * block_size as f32;

                draw_block(&mut img, *color, block_size, (x as u16, y as u16));
            }
        }
        _ => {
            todo!();
        }
    }


    return img;
}



pub fn generate_image(width: u32, height: u32, colors: &Vec<HexColor>) -> image::RgbImage {
    let mut imgbuf = image::ImageBuffer::new(width, height);

    let mut rng = rand::thread_rng();

    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        let randi = rng.gen_range(0..colors.len());
        // let r = rng.gen::<u8>();
        // let g = rng.gen::<u8>();
        // let b = rng.gen::<u8>();
        // *pixel = image::Rgb([r, g, b])

        let col = colors[randi];
        *pixel = image::Rgb([col.r, col.g, col.b])
    }

    return imgbuf;
}

