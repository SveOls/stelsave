
use fltk::{prelude::*, *};
use std::collections::HashMap;
use super::{*, World};

use bmp::{Image, Pixel};

pub fn test(world: &World) {

    let mut imagerine = WrapImage::new(world);

    // let mut maxx = 0.0f64;
    // let mut minx = 0.0f64;
    // let mut maxy = 0.0f64;
    // let mut miny = 0.0f64;
    // for i in world.systems.iter() {
    //     maxx = maxx.max(i.0.coordinate.0);
    //     minx = minx.min(i.0.coordinate.0);
    //     maxy = maxy.max(i.0.coordinate.1);
    //     miny = miny.min(i.0.coordinate.1);
    // }
    // println!("{} {}", minx, maxx);
    // println!("{} {}", miny, maxy);

    // let width  = (1.1*(maxx - minx)) as u32;
    // let height = (1.1*(maxy - miny)) as u32;

    // println!("{}", width);
    // println!("{}", height);

    // let size = (1280, 720);
    // // let mut end;
    // // let mut start;
    // let mut drawn: HashMap<usize, Vec<usize>> = HashMap::new();

    // let mut image = sq_new_image(width.max(height), bmp::px![20, 20, 20]);

    // let to_coordinate_x = |x: f64| ((maxx - x) + 0.05*(maxx - minx)) as u32;
    // let to_coordinate_y = |y: f64| ((y - miny) + 0.05*(maxy - miny)) as u32;
    // let mut coordinates;

    // for (num, i) in world.systems.iter().enumerate() {
    //     coordinates = (to_coordinate_x(i.0.coordinate.0), to_coordinate_y(i.0.coordinate.1));
    //     for x in 0..3 {
    //         for y in 0..3 {
    //             image.set_pixel(coordinates.0 + x - 1, coordinates.1 + y - 1, bmp::px!(255, 255, 255));
    //         }
    //     }
    //     image.set_pixel(coordinates.0,     coordinates.1 - 2, bmp::px!(255, 255, 255));
    //     image.set_pixel(coordinates.0,     coordinates.1 + 2, bmp::px!(255, 255, 255));
    //     image.set_pixel(coordinates.0 - 2, coordinates.1,     bmp::px!(255, 255, 255));
    //     image.set_pixel(coordinates.0 + 2, coordinates.1,     bmp::px!(255, 255, 255));

    //     if let Some(a) = drawn.get(&i.0.id) {
    //         for b in a.iter() {
    //             // println!("{}", i.0.name);
    //             // println!("{}", world.systems[*b].0.name);
    //             // println!("{:?}", i.0.coordinate);
    //             // println!("{:?}", ((coordinates.0 + 0.05*(maxx - minx)), (coordinates.1 + 0.05*(maxy - miny))));
    //             // println!("{:?}", world.systems[*b].0.coordinate);
    //             // println!("{:?}", ((world.systems[*b].0.coordinate.0 + 0.05*(maxx - minx)), (world.systems[*b].0.coordinate.1 + 0.05*(maxy - miny))));



    //             // draw_line(&mut image,
    //             //     (coordinates.0 as i64, coordinates.1 as i64),
    //             //     (to_coordinate_x(world.systems[*b].0.coordinate.0) as i64, to_coordinate_y(world.systems[*b].0.coordinate.1) as i64));
    //         }
    //     }

    //     for &it in i.1.iter() {
    //         (*drawn.entry(it).or_insert(Vec::new())).push(num);
    //     }
    // }

    imagerine.draw_stars(&world);

    imagerine.add_lanes(&world);

    imagerine.save(&world);

    // let _ = image.save(format!("images//{}.bmp", world.date));


    let mut img = fltk::image::BmpImage::load(format!("images//{}.bmp", world.date)).unwrap();


    let size = (1280, 720);
    img.scale(size.1, size.1, true, true);

    let app = app::App::default();
    let mut wind = window::Window::new(100, 100, size.0, size.1, "Hello, FLTK!");
    let mut ftm = frame::Frame::new(size.0 - size.1, 0, size.1, size.1, None);
    ftm.set_image(Some(img));
    wind.end();
    wind.show();
    app.run().unwrap();
}


pub trait Drawable {
    fn draw(&self, img: &mut WrapImage, x: f64, y: f64) {
        todo!();
    }
}


pub struct WrapImage {
    widgry: Image,
    width:  u32,
    height: u32,
    xrange: (f64, f64),
    yrange: (f64, f64),
    items:  Vec<Box::<dyn Drawable>>,
}

impl WrapImage {
    fn save(&mut self, world: &World) {
        let _ = self.widgry.save(format!("images//{}.bmp", world.date));
    }
    fn new(world: &World) -> WrapImage {    let mut maxx = 0.0f64;
        let mut minx = 0.0f64;
        let mut maxy = 0.0f64;
        let mut miny = 0.0f64;
        for i in world.systems.iter() {
            maxx = maxx.max(i.0.coordinate.0);
            minx = minx.min(i.0.coordinate.0);
            maxy = maxy.max(i.0.coordinate.1);
            miny = miny.min(i.0.coordinate.1);
        }
        println!("{} {}", minx, maxx);
        println!("{} {}", miny, maxy);

        let width  = (1.1*(maxx - minx)) as u32;
        let height = (1.1*(maxy - miny)) as u32;
        WrapImage {
            widgry: sq_new_image(width.max(height), bmp::px![20, 20, 20]),
            width,
            height,
            xrange: (minx, maxx),
            yrange: (miny, maxy),
            items:  Vec::new()
        }
    }
    fn draw_stars(&mut self, world: &World) {

        let to_coordinate_x = |x: f64| ((self.xrange.1 - x) + 0.05*(self.xrange.1 - self.xrange.0)) as u32;
        let to_coordinate_y = |y: f64| ((y - self.yrange.0) + 0.05*(self.yrange.1 - self.yrange.0)) as u32;
        let mut coordinates;

        for i in world.systems.iter() {
            coordinates = (to_coordinate_x(i.0.coordinate.0), to_coordinate_y(i.0.coordinate.1));
            for x in 0..3 {
                for y in 0..3 {
                    self.widgry.set_pixel(coordinates.0 + x - 1, coordinates.1 + y - 1, bmp::px!(255, 255, 255));
                }
            }
            self.widgry.set_pixel(coordinates.0,     coordinates.1 - 2, bmp::px!(255, 255, 255));
            self.widgry.set_pixel(coordinates.0,     coordinates.1 + 2, bmp::px!(255, 255, 255));
            self.widgry.set_pixel(coordinates.0 - 2, coordinates.1,     bmp::px!(255, 255, 255));
            self.widgry.set_pixel(coordinates.0 + 2, coordinates.1,     bmp::px!(255, 255, 255));
        }

    }
    fn add_lanes(&mut self, world: &World) {
        let mut drawn: HashMap<usize, Vec<usize>> = HashMap::new();


        let to_coordinate_x = |x: f64| ((self.xrange.1 - x) + 0.05*(self.xrange.1 - self.xrange.0));
        let to_coordinate_y = |y: f64| ((y - self.yrange.0) + 0.05*(self.yrange.1 - self.yrange.0));
        let mut coordinates;

        for (num, i) in world.systems.iter().enumerate() {
            coordinates = (to_coordinate_x(i.0.coordinate.0), to_coordinate_y(i.0.coordinate.1));
            if let Some(a) = drawn.get(&i.0.id) {
                for b in a.iter() {
                    // println!("{}", i.0.name);
                    // println!("{}", world.systems[*b].0.name);
                    // println!("{:?}", i.0.coordinate);
                    // println!("{:?}", ((coordinates.0 + 0.05*(maxx - minx)), (coordinates.1 + 0.05*(maxy - miny))));
                    // println!("{:?}", world.systems[*b].0.coordinate);
                    // println!("{:?}", ((world.systems[*b].0.coordinate.0 + 0.05*(maxx - minx)), (world.systems[*b].0.coordinate.1 + 0.05*(maxy - miny))));

                    self.items.push(Box::new(Line::new((coordinates.0, coordinates.1),
                        (to_coordinate_x(world.systems[*b].0.coordinate.0), to_coordinate_y(world.systems[*b].0.coordinate.1)))));

                    // draw_line(&mut image,
                    //     (coordinates.0 as i64, coordinates.1 as i64),
                    //     (to_coordinate_x(world.systems[*b].0.coordinate.0) as i64, to_coordinate_y(world.systems[*b].0.coordinate.1) as i64));
                }
            }
            for &it in i.1.iter() {
                (*drawn.entry(it).or_insert(Vec::new())).push(num);
            }
        }
        for i in self.items.iter() {
            i.draw();
        }
    }
    fn push(&mut self, inp: Box::<dyn Drawable>) {
        self.items.push(inp);
    }
    pub fn get_img(&mut self) -> &mut Image {
        &mut self.widgry
    }
}

#[derive(Copy, Clone)]
struct Line {
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
    fn draw_line(&self, img: &mut Image) {
        let x0 = self.start.0 as i64;
        let y0 = self.start.1 as i64;
        let x1 = self.end.0   as i64;
        let y1 = self.end.1   as i64;
        if (y1 - y0).abs() < (x1 - x0).abs() {
            if x0 > x1 {
                self.draw_line_low(img, (x1, y1), (x0, y0))
            } else {
                self.draw_line_low(img, (x0, y0), (x1, y1))
            }
        }
        else {
            if y0 > y1 {
                self.draw_line_high(img, (x1, y1), (x0, y0))
            } else {
                self.draw_line_high(img, (x0, y0), (x1, y1))
            }
        }
    }
    fn draw_line_low(&self, img: &mut Image, (x0, y0): (i64, i64), (x1, y1): (i64, i64)) {
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
            if img.get_pixel(x as u32, y as u32) != img.get_pixel(x0 as u32, y0 as u32) {
                img.set_pixel(x as u32, y as u32, bmp::px!(125, 125, 125));
            }
            if der > 0 {
                y = y + yi;
                der += 2 * (dy - dx);
            } else {
                der += 2*dy
            }
        }
    }
    fn draw_line_high(&self, img: &mut Image, (x0, y0): (i64, i64), (x1, y1): (i64, i64)) {
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
            if img.get_pixel(x as u32, y as u32) != img.get_pixel(x0 as u32, y0 as u32) {
                img.set_pixel(x as u32, y as u32, bmp::px!(125, 125, 125));
            }
            if der > 0 {
                x += xi;
                der += 2 * (dx - dy)
            } else {
                der += 2*dx
            }
        }
    }
}

impl Drawable for Line {
    fn draw(&self, img: &mut WrapImage, x: f64, y: f64) {
        self.draw_line(img.get_img());
    }
}


fn new_image(x: u32, y: u32, color: Pixel) -> Image {
    let mut img = Image::new(x, y);
    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, color);
    }
    img
}

fn sq_new_image(x: u32, color: Pixel) -> Image {
    new_image(x, x, color)
}