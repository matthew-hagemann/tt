use std::os::raw::c_int;

include!("bindings/bindings.rs");

pub struct TermWindow {
    pub tty_width: c_int,
    pub tty_height: c_int,
    pub window_width: c_int,
    pub window_height: c_int,
    pub char_height: c_int,
    pub char_width: c_int,

    // FIXME: Probably should be enums:
    pub window_state: c_int,
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
