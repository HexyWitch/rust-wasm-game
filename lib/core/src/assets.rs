extern crate failure;
extern crate png;

use failure::Error;

pub fn image_from_png(png_data: &[u8]) -> Result<Image, Error> {
    let decoder = png::Decoder::new(png_data);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;
    Ok(Image {
        data: buf,
        width: info.width,
        height: info.height,
    })
}

#[derive(Hash)]
pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}
