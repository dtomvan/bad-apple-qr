use std::{
    fs::File,
    io::{stdout, Stdout, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use image::{DynamicImage, GrayImage, RgbImage};
use qoi::Channels;

fn main() -> Result<()> {
    let dirvish = std::fs::read_dir("frames-small")?;
    let mut stdout = stdout();
    let decoder = bardecoder::default_decoder();
    for file in dirvish.filter_map(Result::ok) {
        if let Err(e) = process_file(&mut stdout, &decoder, file.path()) {
            eprintln!("Error on file {file:?}: {e}");
        }
    }
    Ok(())
}

fn process_file(
    stdout: &mut Stdout,
    decoder: &bardecoder::Decoder<DynamicImage, GrayImage, String>,
    path: PathBuf,
) -> Result<()> {
    let file = File::open(&path)?;
    let mut qoi_decoder = qoi::Decoder::from_stream(file)?.with_channels(Channels::Rgb);

    let header = qoi_decoder.header();
    let (width, height) = (header.width, header.height);

    let decoded = qoi_decoder.decode_to_vec()?;
    let img = DynamicImage::ImageRgb8(
        RgbImage::from_vec(width, height, decoded).ok_or_else(|| anyhow!("Expected image"))?,
    );
    for result in decoder.decode(&img).into_iter().filter_map(Result::ok) {
        let _ = writeln!(stdout, "{}:::{result}", path.display());
    }
    Ok(())
}
