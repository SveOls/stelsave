use bmp::{Pixel};
use super::Drawable;


#[derive(Copy, Clone)]
pub struct Star {
    pos: (f64, f64),
    id: usize
}

impl Star {
    pub fn new(id: usize, pos: (f64, f64)) -> Star {
        Star {
            pos,
            id
        }
    }
}

impl Drawable for Star {
    fn draw_galaxy_1(&self, _: Option<(f64, f64)>, color: Option<Pixel>) -> Option<Vec<(i32, i32, Pixel)>> {
        let mut ret = Vec::new();
        let color = match color {
            Some(a) => a,
            None    => bmp::px![255, 255, 255]
        };
        for x in 0..3 {
            for y in 0..3 {
                ret.push((self.pos.0 as i32 + x - 1, self.pos.1 as i32 + y - 1, color));
                // img.set_pixel(self.pos.0 as i32 + x - 1, self.pos.1 as i32 + y - 1, bmp::px!(255, 255, 255));
            }
        }
        ret.push((self.pos.0 as i32    , self.pos.1 as i32 - 2, color));
        ret.push((self.pos.0 as i32    , self.pos.1 as i32 + 2, color));
        ret.push((self.pos.0 as i32 - 2, self.pos.1 as i32    , color));
        ret.push((self.pos.0 as i32 + 2, self.pos.1 as i32    , color));
        Some(ret)
    }
    fn draw_closest_to(&self, _: Option<(f64, f64)>, _: Option<Pixel>) ->   Option<(i32, i32, usize)> {
        Some((self.pos.0 as i32 + 2, self.pos.1 as i32, self.id))
    }
}
