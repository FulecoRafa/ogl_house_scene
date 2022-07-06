#[macro_export]
macro_rules! load_tex {
    ($display: expr, $path: expr, png) => {
        {
            let image = image::load(std::io::Cursor::new(&include_bytes!($path)),
                image::ImageFormat::Png).unwrap().to_rgba8();
            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
            let texture = glium::texture::SrgbTexture2d::new($display, image).unwrap();
            texture
        }
    };

    ($display: expr, $path: expr, jpg) => {
        {
            let image = image::load(std::io::Cursor::new(&include_bytes!($path)),
                image::ImageFormat::Jpeg).unwrap().to_rgba8();
            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
            let texture = glium::texture::SrgbTexture2d::new($display, image).unwrap();
            texture
        }
    }
}