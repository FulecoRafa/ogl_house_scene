extern crate image;

#[macro_export]
macro_rules! load_tex {
    ($display: ident, $path: expr, jpeg, srgb) => {
        {
            let image = image::load(std::io::Cursor::new(&include_bytes!($path)),
                            image::ImageFormat::Jpeg).unwrap().to_rgba8();

            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

            let texture = glium::texture::SrgbTexture2d::new(&$display, image).unwrap();

            texture
        }
    };
    ($display: ident, $path: expr, png, srgb) => {
        {
            let image = image::load(std::io::Cursor::new(&include_bytes!($path)),
                            image::ImageFormat::Png).unwrap().to_rgba8();

            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

            let texture = glium::texture::SrgbTexture2d::new($display, image).unwrap();

            texture
        }
    };
    ($display: ident, $path: expr, jpeg) => {
        {
            let image = image::load(std::io::Cursor::new(&include_bytes!($path)),
                            image::ImageFormat::Jpeg).unwrap().to_rgba8();

            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

            let texture = glium::texture::Texture2d::new($display, image).unwrap();

            texture
        }
    };
    ($display: ident, $path: expr, png) => {
        {
            let image = image::load(std::io::Cursor::new(&include_bytes!($path)),
                            image::ImageFormat::Png).unwrap().to_rgba8();

            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

            let texture = glium::texture::Texture2d::new($display, image).unwrap();

            texture
        }
    };
}
