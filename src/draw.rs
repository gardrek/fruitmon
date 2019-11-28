use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;

use image;

pub fn load_image(filepath: &str) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filepath)?;
    let buf = std::io::BufReader::new(file);
    let sprite = image::load(buf, image::ImageFormat::PNG).unwrap();
    Ok(sprite)
}

pub fn blit(screen: &mut [u8], dest: (usize, usize), image: &image::DynamicImage) {
    let image = image.as_rgba8().unwrap();

    let width = image.width() as usize;
    let height = image.height() as usize;

    let run_width = width.min(SCREEN_WIDTH - dest.0);
    let run_height = height.min(SCREEN_HEIGHT - dest.1);

    let mut pixels = image.pixels();

    for y in 0..height {
        for x in 0..width {
            let pixel = pixels.next().unwrap();
            if y < run_height && x < run_width && pixel[3] > 0 {
                let screen_x = x + dest.0;
                let screen_y = y + dest.1;
                let index = screen_x + screen_y * SCREEN_WIDTH;
                for channel in 0..3 {
                    screen[index * 4 + channel] = pixel.0[channel];
                }
            }
        }
    }
}

/*
/// Drawables can be blitted to the pixel buffer and animated.
pub trait Drawable<V> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}

impl Drawable for image::DynamicImage {
    fn width(&self) -> usize {
         self.as_rgba8().unwrap().width() as usize
    }

    fn height(&self) -> usize {
         self.as_rgba8().unwrap().height() as usize
    }

    fn pixels(&self) -> &[u8] {
         self.as_rgba8().unwrap().into_raw()
    }
}

/// Blit a drawable to the pixel buffer.
pub fn blit<S>(screen: &mut [u8], dest: (usize, usize), sprite: &S)
where
    S: Drawable,
{
    assert!(dest.0 + sprite.width() <= SCREEN_WIDTH);
    assert!(dest.1 + sprite.height() <= SCREEN_HEIGHT);

    let pixels = sprite.pixels();
    let width = sprite.width() * 4;

    let mut s = 0;
    for y in 0..sprite.height() {
        let i = dest.0 * 4 + dest.1 * SCREEN_WIDTH * 4 + y * SCREEN_WIDTH * 4;

        // Merge pixels from sprite into screen
        let zipped = screen[i..i + width].iter_mut().zip(&pixels[s..s + width]);
        for (left, &right) in zipped {
            if right > 0 {
                *left = right;
            }
        }

        s += width;
    }
}
*/
