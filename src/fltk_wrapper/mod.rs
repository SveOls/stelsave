
use fltk::{prelude::*, *};
use bmp::Pixel;

mod wraps;
mod bmpwrap;
use wraps::{AppWrap, WindWrap, FrameWrap};

pub fn test(world: &super::World) {

    let dimension = (1280, 720);



    let mut app = AppWrap::new();
    let mut wind = WindWrap::new(dimension.0, dimension.1);
    // let mut frame = FrameWrap::new_square(dimension);
    let mut frame = FrameWrap::new_square(dimension);



    let mut img = bmpwrap::BmpWrap::new(world.clone(), dimension.1);
    // img.dis_map(None);
    img.draw_g(None);
    img.read_to(None);
    frame.set_image(img.get_content());

    let _but1 = button::Button::new(70, 200, 80, 40, "To Mosaic!");
    let _but2 = button::Button::new(160, 200, 80, 40, "To Galaxy!");

    // let (s1, r1) = app::channel();
    // but1.emit(s1, 0);


    wind.end();
    wind.show();

    app.run();

}

