use std::io::Cursor;
use std::path::Path;
use image::{DynamicImage, GenericImageView, ImageBuffer, imageops};
use actix_web::{Error, HttpResponse};

/// Serves the static image.
pub async fn serve_static_map() -> Result<HttpResponse, Error> {
    let img_path = Path::new("./resources/images/maps/map.png");
    let img = image::open(img_path).unwrap().to_rgba8();
    let mut buffer = Vec::new();

    // Write the image to a buffer in PNG format.
    image::png::PngEncoder::new(&mut buffer)
        .encode(
            &img,
            img.width(),
            img.height(),
            image::ColorType::Rgba8,
        )
        .unwrap();

    Ok(
        HttpResponse::Ok()
            .content_type("image/png")
            .body(buffer),
    )
}

/// Generates a dynamically rendered image.
pub async fn generate_dynamic_image() -> Result<HttpResponse, Error> {
    let icon_path = Path::new("./resources/images/icons/icon.png");
    let icon = image::open(icon_path).unwrap().to_rgba8();

    // Do computation and draw the dynamic object. For this example, we'll overlay the icon onto a blank image.
    let mut image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::new(300, 300);

    imageops::overlay(&mut image, &icon, 100, 100);

    // For real-time computation, you can run your complex algorithms here and update `image` accordingly.

    let mut buffer = Vec::new();

    // Write the image to a buffer in PNG format.
    image::png::PngEncoder::new(&mut buffer)
        .encode(
            &image,
            image.width(),
            image.height(),
            image::ColorType::Rgba8,
        )
        .unwrap();

    Ok(
        HttpResponse::Ok()
            .content_type("image/png")
            .body(buffer),
    )
}

/// Function to convert ImageBuffer to bytes for WebSocket transfer.
pub fn image_to_bytes(img: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Vec<u8> {
    let mut buffer = Vec::new();
    image::png::PngEncoder::new(&mut buffer)
        .encode(
            &img,
            img.width(),
            img.height(),
            image::ColorType::Rgba8,
        )
        .unwrap();

    buffer
}
