use anyhow::Result;
use image::io::Reader as ImageReader;

fn main() -> Result<()> {
    let mut base = ImageReader::open("test_img/768.png")?.decode()?.to_rgba8();
    let scales = [
        ImageReader::open("test_img/256.png")?.decode()?.to_rgba8(),
        ImageReader::open("test_img/128.png")?.decode()?.to_rgba8(),
        ImageReader::open("test_img/64.png")?.decode()?.to_rgba8(),
        ImageReader::open("test_img/32.png")?.decode()?.to_rgba8(),
        ImageReader::open("test_img/16.png")?.decode()?.to_rgba8(),
        ImageReader::open("test_img/8.png")?.decode()?.to_rgba8(),
        ImageReader::open("test_img/4.png")?.decode()?.to_rgba8(),
        ImageReader::open("test_img/2.png")?.decode()?.to_rgba8(),
        ImageReader::open("test_img/1.png")?.decode()?.to_rgba8(),
    ];

    let half = base.width() / 2;
    for img in scales {
        let rat = base.width() / img.width();
        let mut cury = rat / 2;
        for y in 0..img.width() {
            let mut curx = rat / 2;
            for x in 0..img.width() {
                let p = *img.get_pixel(x, y);
                *base.get_pixel_mut(curx, cury) = p;
                curx += rat;
            }
            cury += rat;
        }
    }

    base.save("res.png")?;

    Ok(())
}
