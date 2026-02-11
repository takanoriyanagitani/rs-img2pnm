use std::io;

use io::Cursor;
use io::Seek;
use io::Write;

use io::Read;

use image::DynamicImage;
use image::ImageFormat;

pub fn bytes2image(dat: &[u8]) -> Result<DynamicImage, io::Error> {
    image::load_from_memory(dat).map_err(io::Error::other)
}

pub fn img2wtr<W>(img: &DynamicImage, wtr: W) -> Result<(), io::Error>
where
    W: Write + Seek,
{
    img.write_to(wtr, ImageFormat::Pnm)
        .map_err(io::Error::other)
}

pub fn reader2bytes<R>(rdr: R, limit: u64) -> Result<Vec<u8>, io::Error>
where
    R: Read,
{
    let mut taken = rdr.take(limit);
    let mut buf = vec![];
    taken.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn stdin2bytes(limit: u64) -> Result<Vec<u8>, io::Error> {
    reader2bytes(io::stdin().lock(), limit)
}

pub fn stdin2img(limit: u64) -> Result<DynamicImage, io::Error> {
    let imgdat: Vec<u8> = stdin2bytes(limit)?;
    bytes2image(&imgdat)
}

pub fn img2stdout(img: &DynamicImage) -> Result<(), io::Error> {
    let buf: Vec<u8> = vec![];
    let mut cur = Cursor::new(buf);
    img2wtr(img, &mut cur)?;
    let wrote: Vec<u8> = cur.into_inner();
    let o = io::stdout();
    let mut ol = o.lock();
    ol.write_all(&wrote)?;
    ol.flush()
}

pub fn stdin2img2pnm(limit: u64) -> Result<(), io::Error> {
    let img: DynamicImage = stdin2img(limit)?;
    img2stdout(&img)
}
