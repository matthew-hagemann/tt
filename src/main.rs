// // Include the generated bindings

// #![allow(improper_ctypes)]
// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]
// include!("bindings/bindings.rs");

// use std::ptr;
// mod x;

// fn main() {
//     unsafe {
//         // Open display
//         let display = XOpenDisplay(ptr::null());
//         if display.is_null() {
//             eprintln!("Unable to open display");
//             return;
//         }

//         // Create a simple window
//         let screen = XDefaultScreen(display);
//         let root_window = XRootWindow(display, screen);
//         let black_pixel = XBlackPixel(display, screen);
//         let white_pixel = XWhitePixel(display, screen);

//         let window = XCreateSimpleWindow(
//             display,
//             root_window,
//             0,     // x position
//             0,     // y position
//             500,   // width
//             500,   // height
//             1,     // border width
//             black_pixel,
//             white_pixel,
//         );

//         // Map the window
//         XMapWindow(display, window);

//         // Create a graphics context
//         let gc = XCreateGC(display, window, 0, ptr::null_mut());

//         // Draw a rectangle
//         XSetForeground(display, gc, black_pixel);
//         XDrawRectangle(display, window, gc, 20, 20, 160, 60);
//         XFlush(display); // Make sure the rectangle is displayed

//         // Sleep to keep the window open briefly
//         std::thread::sleep(std::time::Duration::from_secs(2));

//         // Clean up
//         XFreeGC(display, gc);
//         XDestroyWindow(display, window);
//         XCloseDisplay(display);
//     }
// }
//
fn main() {}

