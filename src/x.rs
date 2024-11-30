use std::{ffi::c_void, os::raw::{c_int, c_short}};

include!("bindings/bindings.rs");

#[derive(Debug, Clone)]
pub enum Arg {
    Int(i32),
    UInt(u32),
    Float(f32),
    Ptr(*const c_void),
    Str(&'static str),
}

/// Represents a keyboard shortcut.
pub struct Shortcut<'a> {
    /// Modifiers held to execute the shortcut.
    pub modifiers: usize,

    /// Key pressed to invoke the shortcut.
    pub key_symbol: KeySym,

    /// Function to be executed.
    pub func: Option<&'a dyn Fn(&Arg)>,

    /// Arguments passed through to the function.
    pub arg: Arg,  
}

#[derive(PartialEq)]
pub enum WindowMode {
	VISIBLE,
	FOCUSED,
	APPKEYPAD,
	MOUSEBTN,
	MOUSEMOTION,
	REVERSE,
	KBDLOCK,
	HIDE,
	APPCURSOR,
	MOUSESGR,
	EIGHTBIT,
	BLINK,
	FBLINK,
	FOCUS,
	MOUSEX10,
	MOUSEMANY,
	BRCKTPASTE,
	NUMLOCK,
}

pub struct TermWindow {
    pub tty_width: c_int,
    pub tty_height: c_int,
    pub window_width: c_int,
    pub window_height: c_int,
    pub char_height: c_int,
    pub char_width: c_int,

    // FIXME: Probably should be enums:
    pub window_mode: WindowMode,
    pub cursor_style: c_int,
}

pub struct XWindow {
    pub display: *mut Display,
    pub colormap: Colormap,
    pub window: Window,
    pub drawable: Drawable,
    pub glyph_font_spec: *mut XftGlyphFontSpec,
    pub x_embed: Atom,
    pub wm_delete_win: Atom,
    pub net_wm_name: Atom,
    pub net_wm_icon_name: Atom,
    pub net_wm_pid: Atom,
    pub input_method_editor: InputMethodEditor,
    pub draw: XftDraw,
    pub visual: *mut Visual,
    pub attributes: XSetWindowAttributes,
    pub screen: c_int,

    //FIXME: Could this be a bool?
    pub fixed_geometry: c_int,
    pub left_offset: c_int,
    pub top_offset: c_int,
    pub geometry_mask: c_int,
}

pub struct InputMethodEditor {
    pub x_input_method: XIM,
    pub x_input_context: XIC,
    pub spot: XPoint,
    pub spotlist: XVaNestedList,
}

pub struct XSelection {
    pub x_target: Atom,
    pub primary: char,
    pub clipboard: char,
    pub time_click_1: timespec,
    pub time_click_2: timespec,
}

pub struct Font {
    pub height: c_int,
    pub width: c_int,
    pub ascent: c_int,
    pub descent: c_int,
    pub bad_slant: c_int,
    pub bad_weight: c_int,
    pub left_bearing: c_short,
    pub right_bearing: c_short,
    // check if these could be *const depending on use
    pub matched_font: *mut XftFont, 
    pub font_set: *mut FcFontSet, // set of fonts that match pattern
    pub font_pattern: *mut FcPattern, // pattern used to match fonts
}

pub struct DrawingContext {
    pub Color: *mut XftColor,
    pub col_length: usize,
    pub font: Font,
    pub bold_font: Font,
    pub italic_font: Font,
    pub Italic_bold_font: Font,
    pub graphics_context: GC,
}

// TODO: Event handlers

// FIXME: Move TermWindow to a struct along with other static globals
pub unsafe fn key_press(e: *mut XEvent, win: &TermWindow) {
    // Need to understand if e can ever be a null pointer, which is what makes this unsafe.
    let event:  &mut XKeyEvent = &mut (*e).xkey;
    let key_symbol = NoSymbol;
    let mut buf: [u8; 64] = [0; 64];
    let customkey: &mut [u8] = &mut buf;
    let mut length: i32 = 0;
    let mut c: char;
    let status: i32 = 0;
    let mut shortcut: *mut Shortcut = std::ptr::null_mut();
    
    if win.window_mode == WindowMode::KBDLOCK {
        return
    }
}
