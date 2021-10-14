use fltk::{prelude::*, *};

pub struct AppWrap {
    app: app::App,
}

impl AppWrap {
    pub fn new() -> AppWrap {
        AppWrap {
            app: app::App::default(),
        }
    }
    pub fn run(self) {
        self.app.run().unwrap();
    }
    pub fn wait(&mut self) -> bool {
        self.app.wait()
    }
}

pub struct WindWrap {
    window: window::Window,
    height: i32,
    width: i32,
}

impl WindWrap {
    pub fn new(width: i32, height: i32) -> WindWrap {
        WindWrap {
            window: window::Window::new(100, 100, width, height, "None"),
            height,
            width
        }
    }
    pub fn set_label(&mut self, inp: &str) {
        self.window.set_label(inp)
    }
    pub fn new_with_name(width: i32, height: i32, name: &'static str) -> WindWrap {
        WindWrap {
            window: window::Window::new(100, 100, width, height, name),
            height,
            width
        }
    }
    pub fn end(&mut self) {
        self.window.end();
    }
    pub fn show(&mut self) {
        self.window.show();
    }
}

pub struct FrameWrap {
    frame: frame::Frame,
}

impl FrameWrap {
    pub fn new_square(size: (i32, i32)) -> FrameWrap {
        FrameWrap {
            frame: frame::Frame::new(size.0-size.1, 0, size.1, size.1, None)
        }
    }
    pub fn set_image<I: ImageExt>(&mut self, image: Option<I>) {
        self.frame.set_image(image);
    }
    // pub fn test(&self) {
    //     self.frame.handle
    // }
}