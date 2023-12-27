use num::clamp;
use skia_safe::{Canvas, Data, IRect, Image, Matrix, Paint, Rect};
use std::path::PathBuf;

#[derive(Debug)]
pub enum BackgroundMode {
    // Tile,
    // Clamp,
    // Center,
    // Cover,
    // Contain,
    Stretch,
    Fill,
    // Fit,
}

impl Default for BackgroundMode {
    fn default() -> Self {
        BackgroundMode::Stretch
    }
}

#[derive(Debug)]
pub struct Background {
    // Display mode (Fit, Strech, Tile, etc...)
    fill_mode: BackgroundMode,
    matrix: Matrix, // Cached Model matrix
    paint: Paint,   // Paint info (for transparency)
    image_path: Option<PathBuf>,
    image: Option<Image>, // Image ()
}

impl Default for Background {
    fn default() -> Self {
        Background {
            fill_mode: BackgroundMode::default(),
            matrix: Matrix::default(),
            paint: Paint::default(),
            image: None,
            image_path: None,
        }
    }
}

impl Background {
    pub fn reload_image(&mut self) {
        if self.image_path.is_none() {
            println!("No image to reload");
            return;
        }

        let image_path = self.image_path.clone().unwrap();

        let image_data = Data::from_filename(image_path);
        if image_data.is_none() {
            todo!("Handle error when data cannot be loaded, unsupported file format")
        }
        self.image = Image::from_encoded(image_data.unwrap());
        println!("Reloaded image");
    }

    pub fn load_image(&mut self, image_file: PathBuf) {
        // TODO: resize image to the max displayable resolution to avoid unaccesary ressource
        // or instead pad the image to the closest power of two to benefit from mipmapping

        if self.image_path.is_none() {
            println!("First image initialization");
            self.image_path = Some(image_file.clone());
            self.reload_image();
        }

        let loaded_image_path = self.image_path.clone().unwrap();
        if loaded_image_path.to_string_lossy() != image_file.to_string_lossy() {
            println!("Found a new image that is different, Requesting reloading of image");
            self.reload_image();
        }
    }

    pub fn draw(&self, root_canvas: &Canvas) {
        if self.image.is_none() {
            return;
        }

        // TODO: Is this clone needed ?
        let image = self.image.clone().unwrap();

        // root_canvas.draw_image_nine(
        //     image,
        //     IRect::new(0, 0, 200, 400),
        //     Rect::new(0.0, 0.0, 1.0, 1.0),
        //     skia_safe::FilterMode::Linear,
        //     None,
        // );
        root_canvas.draw_image(image, (0, 0), Some(&self.paint));
        root_canvas.save();
        root_canvas.reset_matrix();
    }

    pub fn update_window_info(&mut self, _pos_x: u32, _pos_y: u32, _width: u32, _height: u32) {
        todo!("update_window_info")
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        let alpha = (opacity * 255.0) as u8;
        let alpha = clamp(alpha, 0, 255);

        self.paint.set_alpha(alpha);
    }
}
