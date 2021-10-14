use bmp::{Image, Pixel};
use fltk::{prelude::*, *};
use super::super::World;
use std::collections::HashMap;

mod line;
mod star;






pub struct BmpWrap {
    // imagedata
    image:  Image,
    xrange: (f64, f64),
    yrange: (f64, f64),
    data:   World,
    size:   i32,

    // drawing, fltk
    items:   Vec<Box<dyn Drawable>>,
    content: Option<image::BmpImage>,
    closest: Option<HashMap<(u32, u32), Option<usize>>>
}


impl BmpWrap {
    pub fn _get_height(&self) -> u32 {
        self.image.get_height()
    }
    pub fn _get_width(&self) -> u32 {
        self.image.get_width()
    }
    pub fn new(data: World, size: i32) -> BmpWrap {
        let mut ret = BmpWrap {
            image: Image::new(0, 0),
            items: Vec::new(),
            content: None,
            xrange: (0.0, 0.0),
            yrange: (0.0, 0.0),
            size,
            closest: None,
            data
        };
        for i in ret.data.systems.iter() {
            ret.xrange.1 = ret.xrange.1.max(i.0.coordinate.0);
            ret.xrange.0 = ret.xrange.0.min(i.0.coordinate.0);
            ret.yrange.1 = ret.yrange.1.max(i.0.coordinate.1);
            ret.yrange.0 = ret.yrange.0.min(i.0.coordinate.1);
        }
        let width  = (1.1*(ret.xrange.1 - ret.xrange.0)) as u32;
        let height = (1.1*(ret.yrange.1 - ret.yrange.0)) as u32;
        ret.image = Image::new(width.max(height), height.max(width));
        for (x, y) in ret.image.coordinates() {
            ret.image.set_pixel(x, y, bmp::px![20, 20, 30]);
        }
        let mut drawn: HashMap<usize, Vec<usize>> = HashMap::new();
        for (num, (star, lanes_to)) in ret.data.systems.iter().enumerate() {
            ret.items.push(Box::new(star::Star::new(star.id, star.coordinate)));
            if let Some(a) = drawn.get(&star.id) {
                for b in a.iter() {
                    ret.items.push(Box::new(line::Line::new(star.coordinate, ret.data.systems[*b].0.coordinate)));
                }

            }
            for &it in lanes_to.iter() {
                (*drawn.entry(it).or_insert(Vec::new())).push(num);
            }
        }

        ret
    }
    pub fn read_to(&mut self, name: Option<&str>) {

        let _ = self.save(&format!("images//{}.bmp", name.unwrap_or("temp")));
        self.content = Some(fltk::image::BmpImage::load(&format!("images//{}.bmp", name.unwrap_or("temp"))).unwrap());
        // if let Some(a) = name {
        //     let _ = self.save(&format!("images//{}.bmp", a));
        //     self.content = Some(fltk::image::BmpImage::load(&format!("images//{}.bmp", a)).unwrap());
        // } else {
        //     let _ = self.save(&format!("images//temp.bmp"));
        //     self.content = Some(fltk::image::BmpImage::load(&format!("images//temp.bmp")).unwrap());
        // }
        if let Some(a) = &mut self.content {
            a.scale(self.size, self.size, true, true);
        }
    }
    pub fn _get_pixel(&self, _x: i32, _y: i32) -> Pixel {
        self.image.get_pixel(0, 0)
    }
    pub fn set_pixel(&mut self, x: i32, y: i32, z: Pixel) {

        self.image.set_pixel(self.offset_x(x), self.offset_y(y), z)
    }
    pub fn offset_x(&self, x: i32) -> u32 {
        ((self.xrange.1 as i32 - x) + ((0.05 - 0.5)*(self.xrange.1 - self.xrange.0) + 0.5*((self.xrange.1 - self.xrange.0).max(self.yrange.1 - self.yrange.0))) as i32) as u32
    }
    pub fn offset_y(&self, y: i32) -> u32 {
        ((y - self.yrange.0 as i32) + ((0.05 - 0.5)*(self.yrange.1 - self.yrange.0) + 0.5*((self.xrange.1 - self.xrange.0).max(self.yrange.1 - self.yrange.0))) as i32) as u32
    }
    pub fn save(&mut self, name: &str) {
        let _ = self.image.save(name);
    }
    pub fn get_content(&mut self) -> Option<image::BmpImage> {
        if self.content.is_none() {
            self.read_to(None);
        }
        let mut ret = None;
        std::mem::swap(&mut ret, &mut self.content);
        ret
    }
    /// Draws galaxy view. Optional input is hyperlane colors.
    pub fn draw_g(&mut self, col: Option<Pixel>) {
        let mut temp = Vec::new();
        for i in self.items.iter() {
            if let Some(mut a) = i.draw_galaxy_2(None, col) {
                temp.append(&mut a);
            }
        }
        for i in self.items.iter() {
            if let Some(mut a) = i.draw_galaxy_1(None, None) {
                temp.append(&mut a);
            }
        }
        for &(x, y, pxl) in temp.iter() {
            self.set_pixel(x, y, pxl)
        }
    }
    /// Updates the HashMap of (coordinate, nearest_star). Optional input is radius.
    pub fn make_dis_map(&mut self, inp: Option<u32>) {
        if self.closest.is_none() {
            let mut temp = Vec::new();
            for i in self.items.iter() {
                if let Some(a) = i.draw_closest_to(None, None) {
                    temp.push(a);
                }
            }
            let temp: Vec<(u32, u32, usize)> = temp.iter().map(|(x, y, z)| (self.offset_x(*x), self.offset_y(*y), *z)).collect();

            let radius = {
                match inp {
                    Some(a) => a,
                    None    => 1200
                }
            };

            let mut tempdistance: (u32, usize);
            let mut dy;
            let mut dx;
            let mut ret = HashMap::new();
            for (x, y) in self.image.coordinates() {
                tempdistance = (u32::MAX, 0);
                for &(i, j, id) in temp.iter() {
                    dy = y.max(j) - y.min(j);
                    dx = x.max(i) - x.min(i);
                    if tempdistance.0 > ( dy * dy + dx * dx ) {
                        tempdistance = (dy * dy + dx * dx, id);
                    }
                }
                if tempdistance.0 > radius {
                    ret.insert((x, y), None);
                } else {
                    ret.insert((x, y), Some(tempdistance.1));
                }
            }
            self.closest = Some(ret);
        }
    }
    /// Draws mosaic
    pub fn dis_map(&mut self, inp: Option<u32>) {
        let mut visited: HashMap<usize, u32> = HashMap::new();
        if self.closest.is_none() || inp.is_some() {
            self.make_dis_map(inp);
        }
        let mut color;
        let mut i = 0;
        if let Some(a) = &self.closest {
            for (key, val) in a.iter().filter(|(_, y)| y.is_some()).map(|(x, y)| (x, y.unwrap())) {
                i += 10000;
                color = visited.entry(val).or_insert(i);
                self.image.set_pixel(key.0, key.1, bmp::px![(*color%256),((*color/256)%256),(*color/256)/256])
            }
        }

        // adds stars for testing
        let mut temp = Vec::new();
        for i in self.items.iter() {
            if let Some(mut a) = i.draw_galaxy_1(None, None) {
                temp.append(&mut a);
            }
        }
        for &(x, y, pxl) in temp.iter() {
            self.set_pixel(x, y, pxl)
        }
    }
}


pub trait Drawable {
    /// draws last layer
    fn draw_galaxy_1(&self, _: Option<(f64, f64)>, _: Option<Pixel>) ->     Option<Vec<(i32, i32, Pixel)>> {
        None
    }
    /// draws second to last layer
    fn draw_galaxy_2(&self, _: Option<(f64, f64)>, _: Option<Pixel>) ->     Option<Vec<(i32, i32, Pixel)>> {
        None
    }
    fn draw_closest_to(&self, _: Option<(f64, f64)>, _: Option<Pixel>) ->   Option<(i32, i32, usize)> {
        None
    }
    fn draw_system(&self, _: Option<(f64, f64)>, _: Option<Pixel>) ->       Option<Vec<(i32, i32, Pixel)>> {
        None
    }
}


