use bmp::Pixel;

#[derive(Copy, Clone)]
pub struct Line {
    start: (f64, f64),
    end:   (f64, f64)
}

impl Line {
    pub fn new(start: (f64, f64), end: (f64, f64)) -> Line {
        Line {
            start,
            end,
        }
    }
    fn draw_line(&self, inp: Pixel) -> Vec<(i32, i32, Pixel)> {
        let x0 = self.start.0 as i64;
        let y0 = self.start.1 as i64;
        let x1 = self.end.0   as i64;
        let y1 = self.end.1   as i64;
        if (y1 - y0).abs() < (x1 - x0).abs() {
            if x0 > x1 {
                self.draw_line_low((x1, y1), (x0, y0), inp)
            } else {
                self.draw_line_low((x0, y0), (x1, y1), inp)
            }
        }
        else {
            if y0 > y1 {
                self.draw_line_high((x1, y1), (x0, y0), inp)
            } else {
                self.draw_line_high((x0, y0), (x1, y1), inp)
            }
        }
    }
    fn draw_line_low(&self, (x0, y0): (i64, i64), (x1, y1): (i64, i64), inp: Pixel) -> Vec<(i32, i32, Pixel)> {
        let mut ret = Vec::new();
        let dx = x1 - x0;
        let mut dy = y1 - y0;
        let mut yi = 1;
        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut der = (2 * dy) - dx;
        let mut y = y0;

        for x in x0..x1 {
            ret.push((x as i32, y as i32, inp));
            if der > 0 {
                y = y + yi;
                der += 2 * (dy - dx);
            } else {
                der += 2*dy
            }
        }
        ret
    }
    fn draw_line_high(&self, (x0, y0): (i64, i64), (x1, y1): (i64, i64), inp: Pixel) -> Vec<(i32, i32, Pixel)> {
        let mut ret = Vec::new();
        let mut dx = x1 - x0;
        let dy = y1 - y0;
        let mut xi = 1;
        if dx < 0 {
            xi = -1;
            dx = -dx;
        }
        let mut der = (2 * dx) - dy;
        let mut x = x0;

        for y in y0..y1 {
            ret.push((x as i32, y as i32, inp));
            if der > 0 {
                x += xi;
                der += 2 * (dx - dy)
            } else {
                der += 2*dx
            }
        }
        ret
    }
}

impl super::Drawable for Line {
    fn draw_galaxy_2(&self, _: Option<(f64, f64)>, color: Option<Pixel>) -> Option<Vec<(i32, i32, Pixel)>> {
        match color {
            Some(a) => Some(self.draw_line(a)),
            None    => Some(self.draw_line(bmp::px![125, 125, 125]))
        }
    }
}
