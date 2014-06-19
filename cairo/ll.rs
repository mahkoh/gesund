#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub type cairo_bool_t = ::libc::c_int;
pub enum Struct__cairo { }
pub type cairo_t = Struct__cairo;
pub enum Struct__cairo_surface { }
pub type cairo_surface_t = Struct__cairo_surface;
pub enum Struct__cairo_device { }
pub type cairo_device_t = Struct__cairo_device;
pub struct Struct__cairo_matrix {
    pub xx: ::libc::c_double,
    pub yx: ::libc::c_double,
    pub xy: ::libc::c_double,
    pub yy: ::libc::c_double,
    pub x0: ::libc::c_double,
    pub y0: ::libc::c_double,
}
pub type cairo_matrix_t = Struct__cairo_matrix;
pub enum Struct__cairo_pattern { }
pub type cairo_pattern_t = Struct__cairo_pattern;
pub type cairo_destroy_func_t =
    ::std::option::Option<extern "C" fn(arg1: *mut ::libc::c_void)>;
pub struct Struct__cairo_user_data_key {
    pub unused: ::libc::c_int,
}
pub type cairo_user_data_key_t = Struct__cairo_user_data_key;
pub type Enum__cairo_status = ::libc::c_uint;
pub static CAIRO_STATUS_SUCCESS: ::libc::c_uint = 0;
pub static CAIRO_STATUS_NO_MEMORY: ::libc::c_uint = 1;
pub static CAIRO_STATUS_INVALID_RESTORE: ::libc::c_uint = 2;
pub static CAIRO_STATUS_INVALID_POP_GROUP: ::libc::c_uint = 3;
pub static CAIRO_STATUS_NO_CURRENT_POINT: ::libc::c_uint = 4;
pub static CAIRO_STATUS_INVALID_MATRIX: ::libc::c_uint = 5;
pub static CAIRO_STATUS_INVALID_STATUS: ::libc::c_uint = 6;
pub static CAIRO_STATUS_NULL_POINTER: ::libc::c_uint = 7;
pub static CAIRO_STATUS_INVALID_STRING: ::libc::c_uint = 8;
pub static CAIRO_STATUS_INVALID_PATH_DATA: ::libc::c_uint = 9;
pub static CAIRO_STATUS_READ_ERROR: ::libc::c_uint = 10;
pub static CAIRO_STATUS_WRITE_ERROR: ::libc::c_uint = 11;
pub static CAIRO_STATUS_SURFACE_FINISHED: ::libc::c_uint = 12;
pub static CAIRO_STATUS_SURFACE_TYPE_MISMATCH: ::libc::c_uint = 13;
pub static CAIRO_STATUS_PATTERN_TYPE_MISMATCH: ::libc::c_uint = 14;
pub static CAIRO_STATUS_INVALID_CONTENT: ::libc::c_uint = 15;
pub static CAIRO_STATUS_INVALID_FORMAT: ::libc::c_uint = 16;
pub static CAIRO_STATUS_INVALID_VISUAL: ::libc::c_uint = 17;
pub static CAIRO_STATUS_FILE_NOT_FOUND: ::libc::c_uint = 18;
pub static CAIRO_STATUS_INVALID_DASH: ::libc::c_uint = 19;
pub static CAIRO_STATUS_INVALID_DSC_COMMENT: ::libc::c_uint = 20;
pub static CAIRO_STATUS_INVALID_INDEX: ::libc::c_uint = 21;
pub static CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: ::libc::c_uint = 22;
pub static CAIRO_STATUS_TEMP_FILE_ERROR: ::libc::c_uint = 23;
pub static CAIRO_STATUS_INVALID_STRIDE: ::libc::c_uint = 24;
pub static CAIRO_STATUS_FONT_TYPE_MISMATCH: ::libc::c_uint = 25;
pub static CAIRO_STATUS_USER_FONT_IMMUTABLE: ::libc::c_uint = 26;
pub static CAIRO_STATUS_USER_FONT_ERROR: ::libc::c_uint = 27;
pub static CAIRO_STATUS_NEGATIVE_COUNT: ::libc::c_uint = 28;
pub static CAIRO_STATUS_INVALID_CLUSTERS: ::libc::c_uint = 29;
pub static CAIRO_STATUS_INVALID_SLANT: ::libc::c_uint = 30;
pub static CAIRO_STATUS_INVALID_WEIGHT: ::libc::c_uint = 31;
pub static CAIRO_STATUS_INVALID_SIZE: ::libc::c_uint = 32;
pub static CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: ::libc::c_uint = 33;
pub static CAIRO_STATUS_DEVICE_TYPE_MISMATCH: ::libc::c_uint = 34;
pub static CAIRO_STATUS_DEVICE_ERROR: ::libc::c_uint = 35;
pub static CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: ::libc::c_uint = 36;
pub static CAIRO_STATUS_DEVICE_FINISHED: ::libc::c_uint = 37;
pub static CAIRO_STATUS_LAST_STATUS: ::libc::c_uint = 38;
pub type cairo_status_t = Enum__cairo_status;
pub type Enum__cairo_content = ::libc::c_uint;
pub static CAIRO_CONTENT_COLOR: ::libc::c_uint = 4096;
pub static CAIRO_CONTENT_ALPHA: ::libc::c_uint = 8192;
pub static CAIRO_CONTENT_COLOR_ALPHA: ::libc::c_uint = 12288;
pub type cairo_content_t = Enum__cairo_content;
pub type Enum__cairo_format = ::libc::c_int;
pub static CAIRO_FORMAT_INVALID: ::libc::c_int = -1;
pub static CAIRO_FORMAT_ARGB32: ::libc::c_int = 0;
pub static CAIRO_FORMAT_RGB24: ::libc::c_int = 1;
pub static CAIRO_FORMAT_A8: ::libc::c_int = 2;
pub static CAIRO_FORMAT_A1: ::libc::c_int = 3;
pub static CAIRO_FORMAT_RGB16_565: ::libc::c_int = 4;
pub static CAIRO_FORMAT_RGB30: ::libc::c_int = 5;
pub type cairo_format_t = Enum__cairo_format;
pub type cairo_write_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut ::libc::c_void,
                               arg2: *::libc::c_uchar, arg3: ::libc::c_uint)
                              -> cairo_status_t>;
pub type cairo_read_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut ::libc::c_void,
                               arg2: *mut ::libc::c_uchar,
                               arg3: ::libc::c_uint) -> cairo_status_t>;
pub struct Struct__cairo_rectangle_int {
    pub x: ::libc::c_int,
    pub y: ::libc::c_int,
    pub width: ::libc::c_int,
    pub height: ::libc::c_int,
}
pub type cairo_rectangle_int_t = Struct__cairo_rectangle_int;
pub type Enum__cairo_operator = ::libc::c_uint;
pub static CAIRO_OPERATOR_CLEAR: ::libc::c_uint = 0;
pub static CAIRO_OPERATOR_SOURCE: ::libc::c_uint = 1;
pub static CAIRO_OPERATOR_OVER: ::libc::c_uint = 2;
pub static CAIRO_OPERATOR_IN: ::libc::c_uint = 3;
pub static CAIRO_OPERATOR_OUT: ::libc::c_uint = 4;
pub static CAIRO_OPERATOR_ATOP: ::libc::c_uint = 5;
pub static CAIRO_OPERATOR_DEST: ::libc::c_uint = 6;
pub static CAIRO_OPERATOR_DEST_OVER: ::libc::c_uint = 7;
pub static CAIRO_OPERATOR_DEST_IN: ::libc::c_uint = 8;
pub static CAIRO_OPERATOR_DEST_OUT: ::libc::c_uint = 9;
pub static CAIRO_OPERATOR_DEST_ATOP: ::libc::c_uint = 10;
pub static CAIRO_OPERATOR_XOR: ::libc::c_uint = 11;
pub static CAIRO_OPERATOR_ADD: ::libc::c_uint = 12;
pub static CAIRO_OPERATOR_SATURATE: ::libc::c_uint = 13;
pub static CAIRO_OPERATOR_MULTIPLY: ::libc::c_uint = 14;
pub static CAIRO_OPERATOR_SCREEN: ::libc::c_uint = 15;
pub static CAIRO_OPERATOR_OVERLAY: ::libc::c_uint = 16;
pub static CAIRO_OPERATOR_DARKEN: ::libc::c_uint = 17;
pub static CAIRO_OPERATOR_LIGHTEN: ::libc::c_uint = 18;
pub static CAIRO_OPERATOR_COLOR_DODGE: ::libc::c_uint = 19;
pub static CAIRO_OPERATOR_COLOR_BURN: ::libc::c_uint = 20;
pub static CAIRO_OPERATOR_HARD_LIGHT: ::libc::c_uint = 21;
pub static CAIRO_OPERATOR_SOFT_LIGHT: ::libc::c_uint = 22;
pub static CAIRO_OPERATOR_DIFFERENCE: ::libc::c_uint = 23;
pub static CAIRO_OPERATOR_EXCLUSION: ::libc::c_uint = 24;
pub static CAIRO_OPERATOR_HSL_HUE: ::libc::c_uint = 25;
pub static CAIRO_OPERATOR_HSL_SATURATION: ::libc::c_uint = 26;
pub static CAIRO_OPERATOR_HSL_COLOR: ::libc::c_uint = 27;
pub static CAIRO_OPERATOR_HSL_LUMINOSITY: ::libc::c_uint = 28;
pub type cairo_operator_t = Enum__cairo_operator;
pub type Enum__cairo_antialias = ::libc::c_uint;
pub static CAIRO_ANTIALIAS_DEFAULT: ::libc::c_uint = 0;
pub static CAIRO_ANTIALIAS_NONE: ::libc::c_uint = 1;
pub static CAIRO_ANTIALIAS_GRAY: ::libc::c_uint = 2;
pub static CAIRO_ANTIALIAS_SUBPIXEL: ::libc::c_uint = 3;
pub static CAIRO_ANTIALIAS_FAST: ::libc::c_uint = 4;
pub static CAIRO_ANTIALIAS_GOOD: ::libc::c_uint = 5;
pub static CAIRO_ANTIALIAS_BEST: ::libc::c_uint = 6;
pub type cairo_antialias_t = Enum__cairo_antialias;
pub type Enum__cairo_fill_rule = ::libc::c_uint;
pub static CAIRO_FILL_RULE_WINDING: ::libc::c_uint = 0;
pub static CAIRO_FILL_RULE_EVEN_ODD: ::libc::c_uint = 1;
pub type cairo_fill_rule_t = Enum__cairo_fill_rule;
pub type Enum__cairo_line_cap = ::libc::c_uint;
pub static CAIRO_LINE_CAP_BUTT: ::libc::c_uint = 0;
pub static CAIRO_LINE_CAP_ROUND: ::libc::c_uint = 1;
pub static CAIRO_LINE_CAP_SQUARE: ::libc::c_uint = 2;
pub type cairo_line_cap_t = Enum__cairo_line_cap;
pub type Enum__cairo_line_join = ::libc::c_uint;
pub static CAIRO_LINE_JOIN_MITER: ::libc::c_uint = 0;
pub static CAIRO_LINE_JOIN_ROUND: ::libc::c_uint = 1;
pub static CAIRO_LINE_JOIN_BEVEL: ::libc::c_uint = 2;
pub type cairo_line_join_t = Enum__cairo_line_join;
pub struct Struct__cairo_rectangle {
    pub x: ::libc::c_double,
    pub y: ::libc::c_double,
    pub width: ::libc::c_double,
    pub height: ::libc::c_double,
}
pub type cairo_rectangle_t = Struct__cairo_rectangle;
pub struct Struct__cairo_rectangle_list {
    pub status: cairo_status_t,
    pub rectangles: *mut cairo_rectangle_t,
    pub num_rectangles: ::libc::c_int,
}
pub type cairo_rectangle_list_t = Struct__cairo_rectangle_list;
pub enum Struct__cairo_scaled_font { }
pub type cairo_scaled_font_t = Struct__cairo_scaled_font;
pub enum Struct__cairo_font_face { }
pub type cairo_font_face_t = Struct__cairo_font_face;
pub struct cairo_glyph_t {
    pub index: ::libc::c_ulong,
    pub x: ::libc::c_double,
    pub y: ::libc::c_double,
}
pub struct cairo_text_cluster_t {
    pub num_bytes: ::libc::c_int,
    pub num_glyphs: ::libc::c_int,
}
pub type Enum__cairo_text_cluster_flags = ::libc::c_uint;
pub static CAIRO_TEXT_CLUSTER_FLAG_BACKWARD: ::libc::c_uint = 1;
pub type cairo_text_cluster_flags_t = Enum__cairo_text_cluster_flags;
pub struct cairo_text_extents_t {
    pub x_bearing: ::libc::c_double,
    pub y_bearing: ::libc::c_double,
    pub width: ::libc::c_double,
    pub height: ::libc::c_double,
    pub x_advance: ::libc::c_double,
    pub y_advance: ::libc::c_double,
}
pub struct cairo_font_extents_t {
    pub ascent: ::libc::c_double,
    pub descent: ::libc::c_double,
    pub height: ::libc::c_double,
    pub max_x_advance: ::libc::c_double,
    pub max_y_advance: ::libc::c_double,
}
pub type Enum__cairo_font_slant = ::libc::c_uint;
pub static CAIRO_FONT_SLANT_NORMAL: ::libc::c_uint = 0;
pub static CAIRO_FONT_SLANT_ITALIC: ::libc::c_uint = 1;
pub static CAIRO_FONT_SLANT_OBLIQUE: ::libc::c_uint = 2;
pub type cairo_font_slant_t = Enum__cairo_font_slant;
pub type Enum__cairo_font_weight = ::libc::c_uint;
pub static CAIRO_FONT_WEIGHT_NORMAL: ::libc::c_uint = 0;
pub static CAIRO_FONT_WEIGHT_BOLD: ::libc::c_uint = 1;
pub type cairo_font_weight_t = Enum__cairo_font_weight;
pub type Enum__cairo_subpixel_order = ::libc::c_uint;
pub static CAIRO_SUBPIXEL_ORDER_DEFAULT: ::libc::c_uint = 0;
pub static CAIRO_SUBPIXEL_ORDER_RGB: ::libc::c_uint = 1;
pub static CAIRO_SUBPIXEL_ORDER_BGR: ::libc::c_uint = 2;
pub static CAIRO_SUBPIXEL_ORDER_VRGB: ::libc::c_uint = 3;
pub static CAIRO_SUBPIXEL_ORDER_VBGR: ::libc::c_uint = 4;
pub type cairo_subpixel_order_t = Enum__cairo_subpixel_order;
pub type Enum__cairo_hint_style = ::libc::c_uint;
pub static CAIRO_HINT_STYLE_DEFAULT: ::libc::c_uint = 0;
pub static CAIRO_HINT_STYLE_NONE: ::libc::c_uint = 1;
pub static CAIRO_HINT_STYLE_SLIGHT: ::libc::c_uint = 2;
pub static CAIRO_HINT_STYLE_MEDIUM: ::libc::c_uint = 3;
pub static CAIRO_HINT_STYLE_FULL: ::libc::c_uint = 4;
pub type cairo_hint_style_t = Enum__cairo_hint_style;
pub type Enum__cairo_hint_metrics = ::libc::c_uint;
pub static CAIRO_HINT_METRICS_DEFAULT: ::libc::c_uint = 0;
pub static CAIRO_HINT_METRICS_OFF: ::libc::c_uint = 1;
pub static CAIRO_HINT_METRICS_ON: ::libc::c_uint = 2;
pub type cairo_hint_metrics_t = Enum__cairo_hint_metrics;
pub enum Struct__cairo_font_options { }
pub type cairo_font_options_t = Struct__cairo_font_options;
pub type Enum__cairo_font_type = ::libc::c_uint;
pub static CAIRO_FONT_TYPE_TOY: ::libc::c_uint = 0;
pub static CAIRO_FONT_TYPE_FT: ::libc::c_uint = 1;
pub static CAIRO_FONT_TYPE_WIN32: ::libc::c_uint = 2;
pub static CAIRO_FONT_TYPE_QUARTZ: ::libc::c_uint = 3;
pub static CAIRO_FONT_TYPE_USER: ::libc::c_uint = 4;
pub type cairo_font_type_t = Enum__cairo_font_type;
pub type cairo_user_scaled_font_init_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut cairo_scaled_font_t,
                               arg2: *mut cairo_t,
                               arg3: *mut cairo_font_extents_t)
                              -> cairo_status_t>;
pub type cairo_user_scaled_font_render_glyph_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut cairo_scaled_font_t,
                               arg2: ::libc::c_ulong, arg3: *mut cairo_t,
                               arg4: *mut cairo_text_extents_t)
                              -> cairo_status_t>;
pub type cairo_user_scaled_font_text_to_glyphs_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut cairo_scaled_font_t,
                               arg2: *::libc::c_char, arg3: ::libc::c_int,
                               arg4: *mut *mut cairo_glyph_t,
                               arg5: *mut ::libc::c_int,
                               arg6: *mut *mut cairo_text_cluster_t,
                               arg7: *mut ::libc::c_int,
                               arg8: *mut cairo_text_cluster_flags_t)
                              -> cairo_status_t>;
pub type cairo_user_scaled_font_unicode_to_glyph_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut cairo_scaled_font_t,
                               arg2: ::libc::c_ulong,
                               arg3: *mut ::libc::c_ulong) -> cairo_status_t>;
pub type Enum__cairo_path_data_type = ::libc::c_uint;
pub static CAIRO_PATH_MOVE_TO: ::libc::c_uint = 0;
pub static CAIRO_PATH_LINE_TO: ::libc::c_uint = 1;
pub static CAIRO_PATH_CURVE_TO: ::libc::c_uint = 2;
pub static CAIRO_PATH_CLOSE_PATH: ::libc::c_uint = 3;
pub type cairo_path_data_type_t = Enum__cairo_path_data_type;
pub type cairo_path_data_t = Union__cairo_path_data_t;
pub struct Union__cairo_path_data_t {
    pub data: [u64, ..2u],
}
impl Union__cairo_path_data_t {
    pub fn header(&mut self) -> *mut Struct_Unnamed1 {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn point(&mut self) -> *mut Struct_Unnamed2 {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub struct Struct_Unnamed1 {
    pub _type: cairo_path_data_type_t,
    pub length: ::libc::c_int,
}
pub struct Struct_Unnamed2 {
    pub x: ::libc::c_double,
    pub y: ::libc::c_double,
}
pub struct Struct_cairo_path {
    pub status: cairo_status_t,
    pub data: *mut cairo_path_data_t,
    pub num_data: ::libc::c_int,
}
pub type cairo_path_t = Struct_cairo_path;
pub type Enum__cairo_device_type = ::libc::c_int;
pub static CAIRO_DEVICE_TYPE_DRM: ::libc::c_int = 0;
pub static CAIRO_DEVICE_TYPE_GL: ::libc::c_int = 1;
pub static CAIRO_DEVICE_TYPE_SCRIPT: ::libc::c_int = 2;
pub static CAIRO_DEVICE_TYPE_XCB: ::libc::c_int = 3;
pub static CAIRO_DEVICE_TYPE_XLIB: ::libc::c_int = 4;
pub static CAIRO_DEVICE_TYPE_XML: ::libc::c_int = 5;
pub static CAIRO_DEVICE_TYPE_COGL: ::libc::c_int = 6;
pub static CAIRO_DEVICE_TYPE_WIN32: ::libc::c_int = 7;
pub static CAIRO_DEVICE_TYPE_INVALID: ::libc::c_int = -1;
pub type cairo_device_type_t = Enum__cairo_device_type;
pub type cairo_surface_observer_mode_t = ::libc::c_uint;
pub static CAIRO_SURFACE_OBSERVER_NORMAL: ::libc::c_uint = 0;
pub static CAIRO_SURFACE_OBSERVER_RECORD_OPERATIONS: ::libc::c_uint = 1;
pub type cairo_surface_observer_callback_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut cairo_surface_t,
                               arg2: *mut cairo_surface_t,
                               arg3: *mut ::libc::c_void)>;
pub type Enum__cairo_surface_type = ::libc::c_uint;
pub static CAIRO_SURFACE_TYPE_IMAGE: ::libc::c_uint = 0;
pub static CAIRO_SURFACE_TYPE_PDF: ::libc::c_uint = 1;
pub static CAIRO_SURFACE_TYPE_PS: ::libc::c_uint = 2;
pub static CAIRO_SURFACE_TYPE_XLIB: ::libc::c_uint = 3;
pub static CAIRO_SURFACE_TYPE_XCB: ::libc::c_uint = 4;
pub static CAIRO_SURFACE_TYPE_GLITZ: ::libc::c_uint = 5;
pub static CAIRO_SURFACE_TYPE_QUARTZ: ::libc::c_uint = 6;
pub static CAIRO_SURFACE_TYPE_WIN32: ::libc::c_uint = 7;
pub static CAIRO_SURFACE_TYPE_BEOS: ::libc::c_uint = 8;
pub static CAIRO_SURFACE_TYPE_DIRECTFB: ::libc::c_uint = 9;
pub static CAIRO_SURFACE_TYPE_SVG: ::libc::c_uint = 10;
pub static CAIRO_SURFACE_TYPE_OS2: ::libc::c_uint = 11;
pub static CAIRO_SURFACE_TYPE_WIN32_PRINTING: ::libc::c_uint = 12;
pub static CAIRO_SURFACE_TYPE_QUARTZ_IMAGE: ::libc::c_uint = 13;
pub static CAIRO_SURFACE_TYPE_SCRIPT: ::libc::c_uint = 14;
pub static CAIRO_SURFACE_TYPE_QT: ::libc::c_uint = 15;
pub static CAIRO_SURFACE_TYPE_RECORDING: ::libc::c_uint = 16;
pub static CAIRO_SURFACE_TYPE_VG: ::libc::c_uint = 17;
pub static CAIRO_SURFACE_TYPE_GL: ::libc::c_uint = 18;
pub static CAIRO_SURFACE_TYPE_DRM: ::libc::c_uint = 19;
pub static CAIRO_SURFACE_TYPE_TEE: ::libc::c_uint = 20;
pub static CAIRO_SURFACE_TYPE_XML: ::libc::c_uint = 21;
pub static CAIRO_SURFACE_TYPE_SKIA: ::libc::c_uint = 22;
pub static CAIRO_SURFACE_TYPE_SUBSURFACE: ::libc::c_uint = 23;
pub static CAIRO_SURFACE_TYPE_COGL: ::libc::c_uint = 24;
pub type cairo_surface_type_t = Enum__cairo_surface_type;
pub type cairo_raster_source_acquire_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut cairo_pattern_t,
                               arg2: *mut ::libc::c_void,
                               arg3: *mut cairo_surface_t,
                               arg4: *cairo_rectangle_int_t)
                              -> *mut cairo_surface_t>;
pub type cairo_raster_source_release_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut cairo_pattern_t,
                               arg2: *mut ::libc::c_void,
                               arg3: *mut cairo_surface_t)>;
pub type cairo_raster_source_snapshot_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut cairo_pattern_t,
                               arg2: *mut ::libc::c_void) -> cairo_status_t>;
pub type cairo_raster_source_copy_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut cairo_pattern_t,
                               arg2: *mut ::libc::c_void,
                               arg3: *cairo_pattern_t) -> cairo_status_t>;
pub type cairo_raster_source_finish_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut cairo_pattern_t,
                               arg2: *mut ::libc::c_void)>;
pub type Enum__cairo_pattern_type = ::libc::c_uint;
pub static CAIRO_PATTERN_TYPE_SOLID: ::libc::c_uint = 0;
pub static CAIRO_PATTERN_TYPE_SURFACE: ::libc::c_uint = 1;
pub static CAIRO_PATTERN_TYPE_LINEAR: ::libc::c_uint = 2;
pub static CAIRO_PATTERN_TYPE_RADIAL: ::libc::c_uint = 3;
pub static CAIRO_PATTERN_TYPE_MESH: ::libc::c_uint = 4;
pub static CAIRO_PATTERN_TYPE_RASTER_SOURCE: ::libc::c_uint = 5;
pub type cairo_pattern_type_t = Enum__cairo_pattern_type;
pub type Enum__cairo_extend = ::libc::c_uint;
pub static CAIRO_EXTEND_NONE: ::libc::c_uint = 0;
pub static CAIRO_EXTEND_REPEAT: ::libc::c_uint = 1;
pub static CAIRO_EXTEND_REFLECT: ::libc::c_uint = 2;
pub static CAIRO_EXTEND_PAD: ::libc::c_uint = 3;
pub type cairo_extend_t = Enum__cairo_extend;
pub type Enum__cairo_filter = ::libc::c_uint;
pub static CAIRO_FILTER_FAST: ::libc::c_uint = 0;
pub static CAIRO_FILTER_GOOD: ::libc::c_uint = 1;
pub static CAIRO_FILTER_BEST: ::libc::c_uint = 2;
pub static CAIRO_FILTER_NEAREST: ::libc::c_uint = 3;
pub static CAIRO_FILTER_BILINEAR: ::libc::c_uint = 4;
pub static CAIRO_FILTER_GAUSSIAN: ::libc::c_uint = 5;
pub type cairo_filter_t = Enum__cairo_filter;
pub enum Struct__cairo_region { }
pub type cairo_region_t = Struct__cairo_region;
pub type Enum__cairo_region_overlap = ::libc::c_uint;
pub static CAIRO_REGION_OVERLAP_IN: ::libc::c_uint = 0;
pub static CAIRO_REGION_OVERLAP_OUT: ::libc::c_uint = 1;
pub static CAIRO_REGION_OVERLAP_PART: ::libc::c_uint = 2;
pub type cairo_region_overlap_t = Enum__cairo_region_overlap;
pub type Enum__cairo_pdf_version = ::libc::c_uint;
pub static CAIRO_PDF_VERSION_1_4: ::libc::c_uint = 0;
pub static CAIRO_PDF_VERSION_1_5: ::libc::c_uint = 1;
pub type cairo_pdf_version_t = Enum__cairo_pdf_version;
#[link(name = "cairo")]
extern "C" {
    pub fn cairo_version() -> ::libc::c_int;
    pub fn cairo_version_string() -> *::libc::c_char;
    pub fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    pub fn cairo_reference(cr: *mut cairo_t) -> *mut cairo_t;
    pub fn cairo_destroy(cr: *mut cairo_t);
    pub fn cairo_get_reference_count(cr: *mut cairo_t) -> ::libc::c_uint;
    pub fn cairo_get_user_data(cr: *mut cairo_t, key: *cairo_user_data_key_t)
     -> *mut ::libc::c_void;
    pub fn cairo_set_user_data(cr: *mut cairo_t, key: *cairo_user_data_key_t,
                               user_data: *mut ::libc::c_void,
                               destroy: cairo_destroy_func_t) ->
     cairo_status_t;
    pub fn cairo_save(cr: *mut cairo_t);
    pub fn cairo_restore(cr: *mut cairo_t);
    pub fn cairo_push_group(cr: *mut cairo_t);
    pub fn cairo_push_group_with_content(cr: *mut cairo_t,
                                         content: cairo_content_t);
    pub fn cairo_pop_group(cr: *mut cairo_t) -> *mut cairo_pattern_t;
    pub fn cairo_pop_group_to_source(cr: *mut cairo_t);
    pub fn cairo_set_operator(cr: *mut cairo_t, op: cairo_operator_t);
    pub fn cairo_set_source(cr: *mut cairo_t, source: *mut cairo_pattern_t);
    pub fn cairo_set_source_rgb(cr: *mut cairo_t, red: ::libc::c_double,
                                green: ::libc::c_double,
                                blue: ::libc::c_double);
    pub fn cairo_set_source_rgba(cr: *mut cairo_t, red: ::libc::c_double,
                                 green: ::libc::c_double,
                                 blue: ::libc::c_double,
                                 alpha: ::libc::c_double);
    pub fn cairo_set_source_surface(cr: *mut cairo_t,
                                    surface: *mut cairo_surface_t,
                                    x: ::libc::c_double, y: ::libc::c_double);
    pub fn cairo_set_tolerance(cr: *mut cairo_t, tolerance: ::libc::c_double);
    pub fn cairo_set_antialias(cr: *mut cairo_t,
                               antialias: cairo_antialias_t);
    pub fn cairo_set_fill_rule(cr: *mut cairo_t,
                               fill_rule: cairo_fill_rule_t);
    pub fn cairo_set_line_width(cr: *mut cairo_t, width: ::libc::c_double);
    pub fn cairo_set_line_cap(cr: *mut cairo_t, line_cap: cairo_line_cap_t);
    pub fn cairo_set_line_join(cr: *mut cairo_t,
                               line_join: cairo_line_join_t);
    pub fn cairo_set_dash(cr: *mut cairo_t, dashes: *::libc::c_double,
                          num_dashes: ::libc::c_int,
                          offset: ::libc::c_double);
    pub fn cairo_set_miter_limit(cr: *mut cairo_t, limit: ::libc::c_double);
    pub fn cairo_translate(cr: *mut cairo_t, tx: ::libc::c_double,
                           ty: ::libc::c_double);
    pub fn cairo_scale(cr: *mut cairo_t, sx: ::libc::c_double,
                       sy: ::libc::c_double);
    pub fn cairo_rotate(cr: *mut cairo_t, angle: ::libc::c_double);
    pub fn cairo_transform(cr: *mut cairo_t, matrix: *cairo_matrix_t);
    pub fn cairo_set_matrix(cr: *mut cairo_t, matrix: *cairo_matrix_t);
    pub fn cairo_identity_matrix(cr: *mut cairo_t);
    pub fn cairo_user_to_device(cr: *mut cairo_t, x: *mut ::libc::c_double,
                                y: *mut ::libc::c_double);
    pub fn cairo_user_to_device_distance(cr: *mut cairo_t,
                                         dx: *mut ::libc::c_double,
                                         dy: *mut ::libc::c_double);
    pub fn cairo_device_to_user(cr: *mut cairo_t, x: *mut ::libc::c_double,
                                y: *mut ::libc::c_double);
    pub fn cairo_device_to_user_distance(cr: *mut cairo_t,
                                         dx: *mut ::libc::c_double,
                                         dy: *mut ::libc::c_double);
    pub fn cairo_new_path(cr: *mut cairo_t);
    pub fn cairo_move_to(cr: *mut cairo_t, x: ::libc::c_double,
                         y: ::libc::c_double);
    pub fn cairo_new_sub_path(cr: *mut cairo_t);
    pub fn cairo_line_to(cr: *mut cairo_t, x: ::libc::c_double,
                         y: ::libc::c_double);
    pub fn cairo_curve_to(cr: *mut cairo_t, x1: ::libc::c_double,
                          y1: ::libc::c_double, x2: ::libc::c_double,
                          y2: ::libc::c_double, x3: ::libc::c_double,
                          y3: ::libc::c_double);
    pub fn cairo_arc(cr: *mut cairo_t, xc: ::libc::c_double,
                     yc: ::libc::c_double, radius: ::libc::c_double,
                     angle1: ::libc::c_double, angle2: ::libc::c_double);
    pub fn cairo_arc_negative(cr: *mut cairo_t, xc: ::libc::c_double,
                              yc: ::libc::c_double, radius: ::libc::c_double,
                              angle1: ::libc::c_double,
                              angle2: ::libc::c_double);
    pub fn cairo_rel_move_to(cr: *mut cairo_t, dx: ::libc::c_double,
                             dy: ::libc::c_double);
    pub fn cairo_rel_line_to(cr: *mut cairo_t, dx: ::libc::c_double,
                             dy: ::libc::c_double);
    pub fn cairo_rel_curve_to(cr: *mut cairo_t, dx1: ::libc::c_double,
                              dy1: ::libc::c_double, dx2: ::libc::c_double,
                              dy2: ::libc::c_double, dx3: ::libc::c_double,
                              dy3: ::libc::c_double);
    pub fn cairo_rectangle(cr: *mut cairo_t, x: ::libc::c_double,
                           y: ::libc::c_double, width: ::libc::c_double,
                           height: ::libc::c_double);
    pub fn cairo_close_path(cr: *mut cairo_t);
    pub fn cairo_path_extents(cr: *mut cairo_t, x1: *mut ::libc::c_double,
                              y1: *mut ::libc::c_double,
                              x2: *mut ::libc::c_double,
                              y2: *mut ::libc::c_double);
    pub fn cairo_paint(cr: *mut cairo_t);
    pub fn cairo_paint_with_alpha(cr: *mut cairo_t, alpha: ::libc::c_double);
    pub fn cairo_mask(cr: *mut cairo_t, pattern: *mut cairo_pattern_t);
    pub fn cairo_mask_surface(cr: *mut cairo_t, surface: *mut cairo_surface_t,
                              surface_x: ::libc::c_double,
                              surface_y: ::libc::c_double);
    pub fn cairo_stroke(cr: *mut cairo_t);
    pub fn cairo_stroke_preserve(cr: *mut cairo_t);
    pub fn cairo_fill(cr: *mut cairo_t);
    pub fn cairo_fill_preserve(cr: *mut cairo_t);
    pub fn cairo_copy_page(cr: *mut cairo_t);
    pub fn cairo_show_page(cr: *mut cairo_t);
    pub fn cairo_in_stroke(cr: *mut cairo_t, x: ::libc::c_double,
                           y: ::libc::c_double) -> cairo_bool_t;
    pub fn cairo_in_fill(cr: *mut cairo_t, x: ::libc::c_double,
                         y: ::libc::c_double) -> cairo_bool_t;
    pub fn cairo_in_clip(cr: *mut cairo_t, x: ::libc::c_double,
                         y: ::libc::c_double) -> cairo_bool_t;
    pub fn cairo_stroke_extents(cr: *mut cairo_t, x1: *mut ::libc::c_double,
                                y1: *mut ::libc::c_double,
                                x2: *mut ::libc::c_double,
                                y2: *mut ::libc::c_double);
    pub fn cairo_fill_extents(cr: *mut cairo_t, x1: *mut ::libc::c_double,
                              y1: *mut ::libc::c_double,
                              x2: *mut ::libc::c_double,
                              y2: *mut ::libc::c_double);
    pub fn cairo_reset_clip(cr: *mut cairo_t);
    pub fn cairo_clip(cr: *mut cairo_t);
    pub fn cairo_clip_preserve(cr: *mut cairo_t);
    pub fn cairo_clip_extents(cr: *mut cairo_t, x1: *mut ::libc::c_double,
                              y1: *mut ::libc::c_double,
                              x2: *mut ::libc::c_double,
                              y2: *mut ::libc::c_double);
    pub fn cairo_copy_clip_rectangle_list(cr: *mut cairo_t) ->
     *mut cairo_rectangle_list_t;
    pub fn cairo_rectangle_list_destroy(rectangle_list:
                                            *mut cairo_rectangle_list_t);
    pub fn cairo_glyph_allocate(num_glyphs: ::libc::c_int) ->
     *mut cairo_glyph_t;
    pub fn cairo_glyph_free(glyphs: *mut cairo_glyph_t);
    pub fn cairo_text_cluster_allocate(num_clusters: ::libc::c_int) ->
     *mut cairo_text_cluster_t;
    pub fn cairo_text_cluster_free(clusters: *mut cairo_text_cluster_t);
    pub fn cairo_font_options_create() -> *mut cairo_font_options_t;
    pub fn cairo_font_options_copy(original: *cairo_font_options_t) ->
     *mut cairo_font_options_t;
    pub fn cairo_font_options_destroy(options: *mut cairo_font_options_t);
    pub fn cairo_font_options_status(options: *mut cairo_font_options_t) ->
     cairo_status_t;
    pub fn cairo_font_options_merge(options: *mut cairo_font_options_t,
                                    other: *cairo_font_options_t);
    pub fn cairo_font_options_equal(options: *cairo_font_options_t,
                                    other: *cairo_font_options_t) ->
     cairo_bool_t;
    pub fn cairo_font_options_hash(options: *cairo_font_options_t) ->
     ::libc::c_ulong;
    pub fn cairo_font_options_set_antialias(options:
                                                *mut cairo_font_options_t,
                                            antialias: cairo_antialias_t);
    pub fn cairo_font_options_get_antialias(options: *cairo_font_options_t) ->
     cairo_antialias_t;
    pub fn cairo_font_options_set_subpixel_order(options:
                                                     *mut cairo_font_options_t,
                                                 subpixel_order:
                                                     cairo_subpixel_order_t);
    pub fn cairo_font_options_get_subpixel_order(options:
                                                     *cairo_font_options_t) ->
     cairo_subpixel_order_t;
    pub fn cairo_font_options_set_hint_style(options:
                                                 *mut cairo_font_options_t,
                                             hint_style: cairo_hint_style_t);
    pub fn cairo_font_options_get_hint_style(options: *cairo_font_options_t)
     -> cairo_hint_style_t;
    pub fn cairo_font_options_set_hint_metrics(options:
                                                   *mut cairo_font_options_t,
                                               hint_metrics:
                                                   cairo_hint_metrics_t);
    pub fn cairo_font_options_get_hint_metrics(options: *cairo_font_options_t)
     -> cairo_hint_metrics_t;
    pub fn cairo_select_font_face(cr: *mut cairo_t, family: *::libc::c_char,
                                  slant: cairo_font_slant_t,
                                  weight: cairo_font_weight_t);
    pub fn cairo_set_font_size(cr: *mut cairo_t, size: ::libc::c_double);
    pub fn cairo_set_font_matrix(cr: *mut cairo_t, matrix: *cairo_matrix_t);
    pub fn cairo_get_font_matrix(cr: *mut cairo_t,
                                 matrix: *mut cairo_matrix_t);
    pub fn cairo_set_font_options(cr: *mut cairo_t,
                                  options: *cairo_font_options_t);
    pub fn cairo_get_font_options(cr: *mut cairo_t,
                                  options: *mut cairo_font_options_t);
    pub fn cairo_set_font_face(cr: *mut cairo_t,
                               font_face: *mut cairo_font_face_t);
    pub fn cairo_get_font_face(cr: *mut cairo_t) -> *mut cairo_font_face_t;
    pub fn cairo_set_scaled_font(cr: *mut cairo_t,
                                 scaled_font: *cairo_scaled_font_t);
    pub fn cairo_get_scaled_font(cr: *mut cairo_t) ->
     *mut cairo_scaled_font_t;
    pub fn cairo_show_text(cr: *mut cairo_t, utf8: *::libc::c_char);
    pub fn cairo_show_glyphs(cr: *mut cairo_t, glyphs: *cairo_glyph_t,
                             num_glyphs: ::libc::c_int);
    pub fn cairo_show_text_glyphs(cr: *mut cairo_t, utf8: *::libc::c_char,
                                  utf8_len: ::libc::c_int,
                                  glyphs: *cairo_glyph_t,
                                  num_glyphs: ::libc::c_int,
                                  clusters: *cairo_text_cluster_t,
                                  num_clusters: ::libc::c_int,
                                  cluster_flags: cairo_text_cluster_flags_t);
    pub fn cairo_text_path(cr: *mut cairo_t, utf8: *::libc::c_char);
    pub fn cairo_glyph_path(cr: *mut cairo_t, glyphs: *cairo_glyph_t,
                            num_glyphs: ::libc::c_int);
    pub fn cairo_text_extents(cr: *mut cairo_t, utf8: *::libc::c_char,
                              extents: *mut cairo_text_extents_t);
    pub fn cairo_glyph_extents(cr: *mut cairo_t, glyphs: *cairo_glyph_t,
                               num_glyphs: ::libc::c_int,
                               extents: *mut cairo_text_extents_t);
    pub fn cairo_font_extents(cr: *mut cairo_t,
                              extents: *mut cairo_font_extents_t);
    pub fn cairo_font_face_reference(font_face: *mut cairo_font_face_t) ->
     *mut cairo_font_face_t;
    pub fn cairo_font_face_destroy(font_face: *mut cairo_font_face_t);
    pub fn cairo_font_face_get_reference_count(font_face:
                                                   *mut cairo_font_face_t) ->
     ::libc::c_uint;
    pub fn cairo_font_face_status(font_face: *mut cairo_font_face_t) ->
     cairo_status_t;
    pub fn cairo_font_face_get_type(font_face: *mut cairo_font_face_t) ->
     cairo_font_type_t;
    pub fn cairo_font_face_get_user_data(font_face: *mut cairo_font_face_t,
                                         key: *cairo_user_data_key_t) ->
     *mut ::libc::c_void;
    pub fn cairo_font_face_set_user_data(font_face: *mut cairo_font_face_t,
                                         key: *cairo_user_data_key_t,
                                         user_data: *mut ::libc::c_void,
                                         destroy: cairo_destroy_func_t) ->
     cairo_status_t;
    pub fn cairo_scaled_font_create(font_face: *mut cairo_font_face_t,
                                    font_matrix: *cairo_matrix_t,
                                    ctm: *cairo_matrix_t,
                                    options: *cairo_font_options_t) ->
     *mut cairo_scaled_font_t;
    pub fn cairo_scaled_font_reference(scaled_font: *mut cairo_scaled_font_t)
     -> *mut cairo_scaled_font_t;
    pub fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    pub fn cairo_scaled_font_get_reference_count(scaled_font:
                                                     *mut cairo_scaled_font_t)
     -> ::libc::c_uint;
    pub fn cairo_scaled_font_status(scaled_font: *mut cairo_scaled_font_t) ->
     cairo_status_t;
    pub fn cairo_scaled_font_get_type(scaled_font: *mut cairo_scaled_font_t)
     -> cairo_font_type_t;
    pub fn cairo_scaled_font_get_user_data(scaled_font:
                                               *mut cairo_scaled_font_t,
                                           key: *cairo_user_data_key_t) ->
     *mut ::libc::c_void;
    pub fn cairo_scaled_font_set_user_data(scaled_font:
                                               *mut cairo_scaled_font_t,
                                           key: *cairo_user_data_key_t,
                                           user_data: *mut ::libc::c_void,
                                           destroy: cairo_destroy_func_t) ->
     cairo_status_t;
    pub fn cairo_scaled_font_extents(scaled_font: *mut cairo_scaled_font_t,
                                     extents: *mut cairo_font_extents_t);
    pub fn cairo_scaled_font_text_extents(scaled_font:
                                              *mut cairo_scaled_font_t,
                                          utf8: *::libc::c_char,
                                          extents: *mut cairo_text_extents_t);
    pub fn cairo_scaled_font_glyph_extents(scaled_font:
                                               *mut cairo_scaled_font_t,
                                           glyphs: *cairo_glyph_t,
                                           num_glyphs: ::libc::c_int,
                                           extents:
                                               *mut cairo_text_extents_t);
    pub fn cairo_scaled_font_text_to_glyphs(scaled_font:
                                                *mut cairo_scaled_font_t,
                                            x: ::libc::c_double,
                                            y: ::libc::c_double,
                                            utf8: *::libc::c_char,
                                            utf8_len: ::libc::c_int,
                                            glyphs: *mut *mut cairo_glyph_t,
                                            num_glyphs: *mut ::libc::c_int,
                                            clusters:
                                                *mut *mut cairo_text_cluster_t,
                                            num_clusters: *mut ::libc::c_int,
                                            cluster_flags:
                                                *mut cairo_text_cluster_flags_t)
     -> cairo_status_t;
    pub fn cairo_scaled_font_get_font_face(scaled_font:
                                               *mut cairo_scaled_font_t) ->
     *mut cairo_font_face_t;
    pub fn cairo_scaled_font_get_font_matrix(scaled_font:
                                                 *mut cairo_scaled_font_t,
                                             font_matrix:
                                                 *mut cairo_matrix_t);
    pub fn cairo_scaled_font_get_ctm(scaled_font: *mut cairo_scaled_font_t,
                                     ctm: *mut cairo_matrix_t);
    pub fn cairo_scaled_font_get_scale_matrix(scaled_font:
                                                  *mut cairo_scaled_font_t,
                                              scale_matrix:
                                                  *mut cairo_matrix_t);
    pub fn cairo_scaled_font_get_font_options(scaled_font:
                                                  *mut cairo_scaled_font_t,
                                              options:
                                                  *mut cairo_font_options_t);
    pub fn cairo_toy_font_face_create(family: *::libc::c_char,
                                      slant: cairo_font_slant_t,
                                      weight: cairo_font_weight_t) ->
     *mut cairo_font_face_t;
    pub fn cairo_toy_font_face_get_family(font_face: *mut cairo_font_face_t)
     -> *::libc::c_char;
    pub fn cairo_toy_font_face_get_slant(font_face: *mut cairo_font_face_t) ->
     cairo_font_slant_t;
    pub fn cairo_toy_font_face_get_weight(font_face: *mut cairo_font_face_t)
     -> cairo_font_weight_t;
    pub fn cairo_user_font_face_create() -> *mut cairo_font_face_t;
    pub fn cairo_user_font_face_set_init_func(font_face:
                                                  *mut cairo_font_face_t,
                                              init_func:
                                                  cairo_user_scaled_font_init_func_t);
    pub fn cairo_user_font_face_set_render_glyph_func(font_face:
                                                          *mut cairo_font_face_t,
                                                      render_glyph_func:
                                                          cairo_user_scaled_font_render_glyph_func_t);
    pub fn cairo_user_font_face_set_text_to_glyphs_func(font_face:
                                                            *mut cairo_font_face_t,
                                                        text_to_glyphs_func:
                                                            cairo_user_scaled_font_text_to_glyphs_func_t);
    pub fn cairo_user_font_face_set_unicode_to_glyph_func(font_face:
                                                              *mut cairo_font_face_t,
                                                          unicode_to_glyph_func:
                                                              cairo_user_scaled_font_unicode_to_glyph_func_t);
    pub fn cairo_user_font_face_get_init_func(font_face:
                                                  *mut cairo_font_face_t) ->
     cairo_user_scaled_font_init_func_t;
    pub fn cairo_user_font_face_get_render_glyph_func(font_face:
                                                          *mut cairo_font_face_t)
     -> cairo_user_scaled_font_render_glyph_func_t;
    pub fn cairo_user_font_face_get_text_to_glyphs_func(font_face:
                                                            *mut cairo_font_face_t)
     -> cairo_user_scaled_font_text_to_glyphs_func_t;
    pub fn cairo_user_font_face_get_unicode_to_glyph_func(font_face:
                                                              *mut cairo_font_face_t)
     -> cairo_user_scaled_font_unicode_to_glyph_func_t;
    pub fn cairo_get_operator(cr: *mut cairo_t) -> cairo_operator_t;
    pub fn cairo_get_source(cr: *mut cairo_t) -> *mut cairo_pattern_t;
    pub fn cairo_get_tolerance(cr: *mut cairo_t) -> ::libc::c_double;
    pub fn cairo_get_antialias(cr: *mut cairo_t) -> cairo_antialias_t;
    pub fn cairo_has_current_point(cr: *mut cairo_t) -> cairo_bool_t;
    pub fn cairo_get_current_point(cr: *mut cairo_t, x: *mut ::libc::c_double,
                                   y: *mut ::libc::c_double);
    pub fn cairo_get_fill_rule(cr: *mut cairo_t) -> cairo_fill_rule_t;
    pub fn cairo_get_line_width(cr: *mut cairo_t) -> ::libc::c_double;
    pub fn cairo_get_line_cap(cr: *mut cairo_t) -> cairo_line_cap_t;
    pub fn cairo_get_line_join(cr: *mut cairo_t) -> cairo_line_join_t;
    pub fn cairo_get_miter_limit(cr: *mut cairo_t) -> ::libc::c_double;
    pub fn cairo_get_dash_count(cr: *mut cairo_t) -> ::libc::c_int;
    pub fn cairo_get_dash(cr: *mut cairo_t, dashes: *mut ::libc::c_double,
                          offset: *mut ::libc::c_double);
    pub fn cairo_get_matrix(cr: *mut cairo_t, matrix: *mut cairo_matrix_t);
    pub fn cairo_get_target(cr: *mut cairo_t) -> *mut cairo_surface_t;
    pub fn cairo_get_group_target(cr: *mut cairo_t) -> *mut cairo_surface_t;
    pub fn cairo_copy_path(cr: *mut cairo_t) -> *mut cairo_path_t;
    pub fn cairo_copy_path_flat(cr: *mut cairo_t) -> *mut cairo_path_t;
    pub fn cairo_append_path(cr: *mut cairo_t, path: *cairo_path_t);
    pub fn cairo_path_destroy(path: *mut cairo_path_t);
    pub fn cairo_status(cr: *mut cairo_t) -> cairo_status_t;
    pub fn cairo_status_to_string(status: cairo_status_t) -> *::libc::c_char;
    pub fn cairo_device_reference(device: *mut cairo_device_t) ->
     *mut cairo_device_t;
    pub fn cairo_device_get_type(device: *mut cairo_device_t) ->
     cairo_device_type_t;
    pub fn cairo_device_status(device: *mut cairo_device_t) -> cairo_status_t;
    pub fn cairo_device_acquire(device: *mut cairo_device_t) ->
     cairo_status_t;
    pub fn cairo_device_release(device: *mut cairo_device_t);
    pub fn cairo_device_flush(device: *mut cairo_device_t);
    pub fn cairo_device_finish(device: *mut cairo_device_t);
    pub fn cairo_device_destroy(device: *mut cairo_device_t);
    pub fn cairo_device_get_reference_count(device: *mut cairo_device_t) ->
     ::libc::c_uint;
    pub fn cairo_device_get_user_data(device: *mut cairo_device_t,
                                      key: *cairo_user_data_key_t) ->
     *mut ::libc::c_void;
    pub fn cairo_device_set_user_data(device: *mut cairo_device_t,
                                      key: *cairo_user_data_key_t,
                                      user_data: *mut ::libc::c_void,
                                      destroy: cairo_destroy_func_t) ->
     cairo_status_t;
    pub fn cairo_surface_create_similar(other: *mut cairo_surface_t,
                                        content: cairo_content_t,
                                        width: ::libc::c_int,
                                        height: ::libc::c_int) ->
     *mut cairo_surface_t;
    pub fn cairo_surface_create_similar_image(other: *mut cairo_surface_t,
                                              format: cairo_format_t,
                                              width: ::libc::c_int,
                                              height: ::libc::c_int) ->
     *mut cairo_surface_t;
    pub fn cairo_surface_map_to_image(surface: *mut cairo_surface_t,
                                      extents: *cairo_rectangle_int_t) ->
     *mut cairo_surface_t;
    pub fn cairo_surface_unmap_image(surface: *mut cairo_surface_t,
                                     image: *mut cairo_surface_t);
    pub fn cairo_surface_create_for_rectangle(target: *mut cairo_surface_t,
                                              x: ::libc::c_double,
                                              y: ::libc::c_double,
                                              width: ::libc::c_double,
                                              height: ::libc::c_double) ->
     *mut cairo_surface_t;
    pub fn cairo_surface_create_observer(target: *mut cairo_surface_t,
                                         mode: cairo_surface_observer_mode_t)
     -> *mut cairo_surface_t;
    pub fn cairo_surface_observer_add_paint_callback(abstract_surface:
                                                         *mut cairo_surface_t,
                                                     func:
                                                         cairo_surface_observer_callback_t,
                                                     data:
                                                         *mut ::libc::c_void)
     -> cairo_status_t;
    pub fn cairo_surface_observer_add_mask_callback(abstract_surface:
                                                        *mut cairo_surface_t,
                                                    func:
                                                        cairo_surface_observer_callback_t,
                                                    data: *mut ::libc::c_void)
     -> cairo_status_t;
    pub fn cairo_surface_observer_add_fill_callback(abstract_surface:
                                                        *mut cairo_surface_t,
                                                    func:
                                                        cairo_surface_observer_callback_t,
                                                    data: *mut ::libc::c_void)
     -> cairo_status_t;
    pub fn cairo_surface_observer_add_stroke_callback(abstract_surface:
                                                          *mut cairo_surface_t,
                                                      func:
                                                          cairo_surface_observer_callback_t,
                                                      data:
                                                          *mut ::libc::c_void)
     -> cairo_status_t;
    pub fn cairo_surface_observer_add_glyphs_callback(abstract_surface:
                                                          *mut cairo_surface_t,
                                                      func:
                                                          cairo_surface_observer_callback_t,
                                                      data:
                                                          *mut ::libc::c_void)
     -> cairo_status_t;
    pub fn cairo_surface_observer_add_flush_callback(abstract_surface:
                                                         *mut cairo_surface_t,
                                                     func:
                                                         cairo_surface_observer_callback_t,
                                                     data:
                                                         *mut ::libc::c_void)
     -> cairo_status_t;
    pub fn cairo_surface_observer_add_finish_callback(abstract_surface:
                                                          *mut cairo_surface_t,
                                                      func:
                                                          cairo_surface_observer_callback_t,
                                                      data:
                                                          *mut ::libc::c_void)
     -> cairo_status_t;
    pub fn cairo_surface_observer_print(surface: *mut cairo_surface_t,
                                        write_func: cairo_write_func_t,
                                        closure: *mut ::libc::c_void) ->
     cairo_status_t;
    pub fn cairo_surface_observer_elapsed(surface: *mut cairo_surface_t) ->
     ::libc::c_double;
    pub fn cairo_device_observer_print(device: *mut cairo_device_t,
                                       write_func: cairo_write_func_t,
                                       closure: *mut ::libc::c_void) ->
     cairo_status_t;
    pub fn cairo_device_observer_elapsed(device: *mut cairo_device_t) ->
     ::libc::c_double;
    pub fn cairo_device_observer_paint_elapsed(device: *mut cairo_device_t) ->
     ::libc::c_double;
    pub fn cairo_device_observer_mask_elapsed(device: *mut cairo_device_t) ->
     ::libc::c_double;
    pub fn cairo_device_observer_fill_elapsed(device: *mut cairo_device_t) ->
     ::libc::c_double;
    pub fn cairo_device_observer_stroke_elapsed(device: *mut cairo_device_t)
     -> ::libc::c_double;
    pub fn cairo_device_observer_glyphs_elapsed(device: *mut cairo_device_t)
     -> ::libc::c_double;
    pub fn cairo_surface_reference(surface: *mut cairo_surface_t) ->
     *mut cairo_surface_t;
    pub fn cairo_surface_finish(surface: *mut cairo_surface_t);
    pub fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    pub fn cairo_surface_get_device(surface: *mut cairo_surface_t) ->
     *mut cairo_device_t;
    pub fn cairo_surface_get_reference_count(surface: *mut cairo_surface_t) ->
     ::libc::c_uint;
    pub fn cairo_surface_status(surface: *mut cairo_surface_t) ->
     cairo_status_t;
    pub fn cairo_surface_get_type(surface: *mut cairo_surface_t) ->
     cairo_surface_type_t;
    pub fn cairo_surface_get_content(surface: *mut cairo_surface_t) ->
     cairo_content_t;
    pub fn cairo_surface_write_to_png(surface: *mut cairo_surface_t,
                                      filename: *::libc::c_char) ->
     cairo_status_t;
    pub fn cairo_surface_write_to_png_stream(surface: *mut cairo_surface_t,
                                             write_func: cairo_write_func_t,
                                             closure: *mut ::libc::c_void) ->
     cairo_status_t;
    pub fn cairo_surface_get_user_data(surface: *mut cairo_surface_t,
                                       key: *cairo_user_data_key_t) ->
     *mut ::libc::c_void;
    pub fn cairo_surface_set_user_data(surface: *mut cairo_surface_t,
                                       key: *cairo_user_data_key_t,
                                       user_data: *mut ::libc::c_void,
                                       destroy: cairo_destroy_func_t) ->
     cairo_status_t;
    pub fn cairo_surface_get_mime_data(surface: *mut cairo_surface_t,
                                       mime_type: *::libc::c_char,
                                       data: *mut *::libc::c_uchar,
                                       length: *mut ::libc::c_ulong);
    pub fn cairo_surface_set_mime_data(surface: *mut cairo_surface_t,
                                       mime_type: *::libc::c_char,
                                       data: *::libc::c_uchar,
                                       length: ::libc::c_ulong,
                                       destroy: cairo_destroy_func_t,
                                       closure: *mut ::libc::c_void) ->
     cairo_status_t;
    pub fn cairo_surface_supports_mime_type(surface: *mut cairo_surface_t,
                                            mime_type: *::libc::c_char) ->
     cairo_bool_t;
    pub fn cairo_surface_get_font_options(surface: *mut cairo_surface_t,
                                          options: *mut cairo_font_options_t);
    pub fn cairo_surface_flush(surface: *mut cairo_surface_t);
    pub fn cairo_surface_mark_dirty(surface: *mut cairo_surface_t);
    pub fn cairo_surface_mark_dirty_rectangle(surface: *mut cairo_surface_t,
                                              x: ::libc::c_int,
                                              y: ::libc::c_int,
                                              width: ::libc::c_int,
                                              height: ::libc::c_int);
    pub fn cairo_surface_set_device_offset(surface: *mut cairo_surface_t,
                                           x_offset: ::libc::c_double,
                                           y_offset: ::libc::c_double);
    pub fn cairo_surface_get_device_offset(surface: *mut cairo_surface_t,
                                           x_offset: *mut ::libc::c_double,
                                           y_offset: *mut ::libc::c_double);
    pub fn cairo_surface_set_fallback_resolution(surface:
                                                     *mut cairo_surface_t,
                                                 x_pixels_per_inch:
                                                     ::libc::c_double,
                                                 y_pixels_per_inch:
                                                     ::libc::c_double);
    pub fn cairo_surface_get_fallback_resolution(surface:
                                                     *mut cairo_surface_t,
                                                 x_pixels_per_inch:
                                                     *mut ::libc::c_double,
                                                 y_pixels_per_inch:
                                                     *mut ::libc::c_double);
    pub fn cairo_surface_copy_page(surface: *mut cairo_surface_t);
    pub fn cairo_surface_show_page(surface: *mut cairo_surface_t);
    pub fn cairo_surface_has_show_text_glyphs(surface: *mut cairo_surface_t)
     -> cairo_bool_t;
    pub fn cairo_image_surface_create(format: cairo_format_t,
                                      width: ::libc::c_int,
                                      height: ::libc::c_int) ->
     *mut cairo_surface_t;
    pub fn cairo_format_stride_for_width(format: cairo_format_t,
                                         width: ::libc::c_int) ->
     ::libc::c_int;
    pub fn cairo_image_surface_create_for_data(data: *mut ::libc::c_uchar,
                                               format: cairo_format_t,
                                               width: ::libc::c_int,
                                               height: ::libc::c_int,
                                               stride: ::libc::c_int) ->
     *mut cairo_surface_t;
    pub fn cairo_image_surface_get_data(surface: *mut cairo_surface_t) ->
     *mut ::libc::c_uchar;
    pub fn cairo_image_surface_get_format(surface: *mut cairo_surface_t) ->
     cairo_format_t;
    pub fn cairo_image_surface_get_width(surface: *mut cairo_surface_t) ->
     ::libc::c_int;
    pub fn cairo_image_surface_get_height(surface: *mut cairo_surface_t) ->
     ::libc::c_int;
    pub fn cairo_image_surface_get_stride(surface: *mut cairo_surface_t) ->
     ::libc::c_int;
    pub fn cairo_image_surface_create_from_png(filename: *::libc::c_char) ->
     *mut cairo_surface_t;
    pub fn cairo_image_surface_create_from_png_stream(read_func:
                                                          cairo_read_func_t,
                                                      closure:
                                                          *mut ::libc::c_void)
     -> *mut cairo_surface_t;
    pub fn cairo_recording_surface_create(content: cairo_content_t,
                                          extents: *cairo_rectangle_t) ->
     *mut cairo_surface_t;
    pub fn cairo_recording_surface_ink_extents(surface: *mut cairo_surface_t,
                                               x0: *mut ::libc::c_double,
                                               y0: *mut ::libc::c_double,
                                               width: *mut ::libc::c_double,
                                               height: *mut ::libc::c_double);
    pub fn cairo_recording_surface_get_extents(surface: *mut cairo_surface_t,
                                               extents:
                                                   *mut cairo_rectangle_t) ->
     cairo_bool_t;
    pub fn cairo_pattern_create_raster_source(user_data: *mut ::libc::c_void,
                                              content: cairo_content_t,
                                              width: ::libc::c_int,
                                              height: ::libc::c_int) ->
     *mut cairo_pattern_t;
    pub fn cairo_raster_source_pattern_set_callback_data(pattern:
                                                             *mut cairo_pattern_t,
                                                         data:
                                                             *mut ::libc::c_void);
    pub fn cairo_raster_source_pattern_get_callback_data(pattern:
                                                             *mut cairo_pattern_t)
     -> *mut ::libc::c_void;
    pub fn cairo_raster_source_pattern_set_acquire(pattern:
                                                       *mut cairo_pattern_t,
                                                   acquire:
                                                       cairo_raster_source_acquire_func_t,
                                                   release:
                                                       cairo_raster_source_release_func_t);
    pub fn cairo_raster_source_pattern_get_acquire(pattern:
                                                       *mut cairo_pattern_t,
                                                   acquire:
                                                       *mut cairo_raster_source_acquire_func_t,
                                                   release:
                                                       *mut cairo_raster_source_release_func_t);
    pub fn cairo_raster_source_pattern_set_snapshot(pattern:
                                                        *mut cairo_pattern_t,
                                                    snapshot:
                                                        cairo_raster_source_snapshot_func_t);
    pub fn cairo_raster_source_pattern_get_snapshot(pattern:
                                                        *mut cairo_pattern_t)
     -> cairo_raster_source_snapshot_func_t;
    pub fn cairo_raster_source_pattern_set_copy(pattern: *mut cairo_pattern_t,
                                                copy:
                                                    cairo_raster_source_copy_func_t);
    pub fn cairo_raster_source_pattern_get_copy(pattern: *mut cairo_pattern_t)
     -> cairo_raster_source_copy_func_t;
    pub fn cairo_raster_source_pattern_set_finish(pattern:
                                                      *mut cairo_pattern_t,
                                                  finish:
                                                      cairo_raster_source_finish_func_t);
    pub fn cairo_raster_source_pattern_get_finish(pattern:
                                                      *mut cairo_pattern_t) ->
     cairo_raster_source_finish_func_t;
    pub fn cairo_pattern_create_rgb(red: ::libc::c_double,
                                    green: ::libc::c_double,
                                    blue: ::libc::c_double) ->
     *mut cairo_pattern_t;
    pub fn cairo_pattern_create_rgba(red: ::libc::c_double,
                                     green: ::libc::c_double,
                                     blue: ::libc::c_double,
                                     alpha: ::libc::c_double) ->
     *mut cairo_pattern_t;
    pub fn cairo_pattern_create_for_surface(surface: *mut cairo_surface_t) ->
     *mut cairo_pattern_t;
    pub fn cairo_pattern_create_linear(x0: ::libc::c_double,
                                       y0: ::libc::c_double,
                                       x1: ::libc::c_double,
                                       y1: ::libc::c_double) ->
     *mut cairo_pattern_t;
    pub fn cairo_pattern_create_radial(cx0: ::libc::c_double,
                                       cy0: ::libc::c_double,
                                       radius0: ::libc::c_double,
                                       cx1: ::libc::c_double,
                                       cy1: ::libc::c_double,
                                       radius1: ::libc::c_double) ->
     *mut cairo_pattern_t;
    pub fn cairo_pattern_create_mesh() -> *mut cairo_pattern_t;
    pub fn cairo_pattern_reference(pattern: *mut cairo_pattern_t) ->
     *mut cairo_pattern_t;
    pub fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
    pub fn cairo_pattern_get_reference_count(pattern: *mut cairo_pattern_t) ->
     ::libc::c_uint;
    pub fn cairo_pattern_status(pattern: *mut cairo_pattern_t) ->
     cairo_status_t;
    pub fn cairo_pattern_get_user_data(pattern: *mut cairo_pattern_t,
                                       key: *cairo_user_data_key_t) ->
     *mut ::libc::c_void;
    pub fn cairo_pattern_set_user_data(pattern: *mut cairo_pattern_t,
                                       key: *cairo_user_data_key_t,
                                       user_data: *mut ::libc::c_void,
                                       destroy: cairo_destroy_func_t) ->
     cairo_status_t;
    pub fn cairo_pattern_get_type(pattern: *mut cairo_pattern_t) ->
     cairo_pattern_type_t;
    pub fn cairo_pattern_add_color_stop_rgb(pattern: *mut cairo_pattern_t,
                                            offset: ::libc::c_double,
                                            red: ::libc::c_double,
                                            green: ::libc::c_double,
                                            blue: ::libc::c_double);
    pub fn cairo_pattern_add_color_stop_rgba(pattern: *mut cairo_pattern_t,
                                             offset: ::libc::c_double,
                                             red: ::libc::c_double,
                                             green: ::libc::c_double,
                                             blue: ::libc::c_double,
                                             alpha: ::libc::c_double);
    pub fn cairo_mesh_pattern_begin_patch(pattern: *mut cairo_pattern_t);
    pub fn cairo_mesh_pattern_end_patch(pattern: *mut cairo_pattern_t);
    pub fn cairo_mesh_pattern_curve_to(pattern: *mut cairo_pattern_t,
                                       x1: ::libc::c_double,
                                       y1: ::libc::c_double,
                                       x2: ::libc::c_double,
                                       y2: ::libc::c_double,
                                       x3: ::libc::c_double,
                                       y3: ::libc::c_double);
    pub fn cairo_mesh_pattern_line_to(pattern: *mut cairo_pattern_t,
                                      x: ::libc::c_double,
                                      y: ::libc::c_double);
    pub fn cairo_mesh_pattern_move_to(pattern: *mut cairo_pattern_t,
                                      x: ::libc::c_double,
                                      y: ::libc::c_double);
    pub fn cairo_mesh_pattern_set_control_point(pattern: *mut cairo_pattern_t,
                                                point_num: ::libc::c_uint,
                                                x: ::libc::c_double,
                                                y: ::libc::c_double);
    pub fn cairo_mesh_pattern_set_corner_color_rgb(pattern:
                                                       *mut cairo_pattern_t,
                                                   corner_num: ::libc::c_uint,
                                                   red: ::libc::c_double,
                                                   green: ::libc::c_double,
                                                   blue: ::libc::c_double);
    pub fn cairo_mesh_pattern_set_corner_color_rgba(pattern:
                                                        *mut cairo_pattern_t,
                                                    corner_num:
                                                        ::libc::c_uint,
                                                    red: ::libc::c_double,
                                                    green: ::libc::c_double,
                                                    blue: ::libc::c_double,
                                                    alpha: ::libc::c_double);
    pub fn cairo_pattern_set_matrix(pattern: *mut cairo_pattern_t,
                                    matrix: *cairo_matrix_t);
    pub fn cairo_pattern_get_matrix(pattern: *mut cairo_pattern_t,
                                    matrix: *mut cairo_matrix_t);
    pub fn cairo_pattern_set_extend(pattern: *mut cairo_pattern_t,
                                    extend: cairo_extend_t);
    pub fn cairo_pattern_get_extend(pattern: *mut cairo_pattern_t) ->
     cairo_extend_t;
    pub fn cairo_pattern_set_filter(pattern: *mut cairo_pattern_t,
                                    filter: cairo_filter_t);
    pub fn cairo_pattern_get_filter(pattern: *mut cairo_pattern_t) ->
     cairo_filter_t;
    pub fn cairo_pattern_get_rgba(pattern: *mut cairo_pattern_t,
                                  red: *mut ::libc::c_double,
                                  green: *mut ::libc::c_double,
                                  blue: *mut ::libc::c_double,
                                  alpha: *mut ::libc::c_double) ->
     cairo_status_t;
    pub fn cairo_pattern_get_surface(pattern: *mut cairo_pattern_t,
                                     surface: *mut *mut cairo_surface_t) ->
     cairo_status_t;
    pub fn cairo_pattern_get_color_stop_rgba(pattern: *mut cairo_pattern_t,
                                             index: ::libc::c_int,
                                             offset: *mut ::libc::c_double,
                                             red: *mut ::libc::c_double,
                                             green: *mut ::libc::c_double,
                                             blue: *mut ::libc::c_double,
                                             alpha: *mut ::libc::c_double) ->
     cairo_status_t;
    pub fn cairo_pattern_get_color_stop_count(pattern: *mut cairo_pattern_t,
                                              count: *mut ::libc::c_int) ->
     cairo_status_t;
    pub fn cairo_pattern_get_linear_points(pattern: *mut cairo_pattern_t,
                                           x0: *mut ::libc::c_double,
                                           y0: *mut ::libc::c_double,
                                           x1: *mut ::libc::c_double,
                                           y1: *mut ::libc::c_double) ->
     cairo_status_t;
    pub fn cairo_pattern_get_radial_circles(pattern: *mut cairo_pattern_t,
                                            x0: *mut ::libc::c_double,
                                            y0: *mut ::libc::c_double,
                                            r0: *mut ::libc::c_double,
                                            x1: *mut ::libc::c_double,
                                            y1: *mut ::libc::c_double,
                                            r1: *mut ::libc::c_double) ->
     cairo_status_t;
    pub fn cairo_mesh_pattern_get_patch_count(pattern: *mut cairo_pattern_t,
                                              count: *mut ::libc::c_uint) ->
     cairo_status_t;
    pub fn cairo_mesh_pattern_get_path(pattern: *mut cairo_pattern_t,
                                       patch_num: ::libc::c_uint) ->
     *mut cairo_path_t;
    pub fn cairo_mesh_pattern_get_corner_color_rgba(pattern:
                                                        *mut cairo_pattern_t,
                                                    patch_num: ::libc::c_uint,
                                                    corner_num:
                                                        ::libc::c_uint,
                                                    red:
                                                        *mut ::libc::c_double,
                                                    green:
                                                        *mut ::libc::c_double,
                                                    blue:
                                                        *mut ::libc::c_double,
                                                    alpha:
                                                        *mut ::libc::c_double)
     -> cairo_status_t;
    pub fn cairo_mesh_pattern_get_control_point(pattern: *mut cairo_pattern_t,
                                                patch_num: ::libc::c_uint,
                                                point_num: ::libc::c_uint,
                                                x: *mut ::libc::c_double,
                                                y: *mut ::libc::c_double) ->
     cairo_status_t;
    pub fn cairo_matrix_init(matrix: *mut cairo_matrix_t,
                             xx: ::libc::c_double, yx: ::libc::c_double,
                             xy: ::libc::c_double, yy: ::libc::c_double,
                             x0: ::libc::c_double, y0: ::libc::c_double);
    pub fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
    pub fn cairo_matrix_init_translate(matrix: *mut cairo_matrix_t,
                                       tx: ::libc::c_double,
                                       ty: ::libc::c_double);
    pub fn cairo_matrix_init_scale(matrix: *mut cairo_matrix_t,
                                   sx: ::libc::c_double,
                                   sy: ::libc::c_double);
    pub fn cairo_matrix_init_rotate(matrix: *mut cairo_matrix_t,
                                    radians: ::libc::c_double);
    pub fn cairo_matrix_translate(matrix: *mut cairo_matrix_t,
                                  tx: ::libc::c_double, ty: ::libc::c_double);
    pub fn cairo_matrix_scale(matrix: *mut cairo_matrix_t,
                              sx: ::libc::c_double, sy: ::libc::c_double);
    pub fn cairo_matrix_rotate(matrix: *mut cairo_matrix_t,
                               radians: ::libc::c_double);
    pub fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    pub fn cairo_matrix_multiply(result: *mut cairo_matrix_t,
                                 a: *cairo_matrix_t, b: *cairo_matrix_t);
    pub fn cairo_matrix_transform_distance(matrix: *cairo_matrix_t,
                                           dx: *mut ::libc::c_double,
                                           dy: *mut ::libc::c_double);
    pub fn cairo_matrix_transform_point(matrix: *cairo_matrix_t,
                                        x: *mut ::libc::c_double,
                                        y: *mut ::libc::c_double);
    pub fn cairo_region_create() -> *mut cairo_region_t;
    pub fn cairo_region_create_rectangle(rectangle: *cairo_rectangle_int_t) ->
     *mut cairo_region_t;
    pub fn cairo_region_create_rectangles(rects: *cairo_rectangle_int_t,
                                          count: ::libc::c_int) ->
     *mut cairo_region_t;
    pub fn cairo_region_copy(original: *cairo_region_t) ->
     *mut cairo_region_t;
    pub fn cairo_region_reference(region: *mut cairo_region_t) ->
     *mut cairo_region_t;
    pub fn cairo_region_destroy(region: *mut cairo_region_t);
    pub fn cairo_region_equal(a: *cairo_region_t, b: *cairo_region_t) ->
     cairo_bool_t;
    pub fn cairo_region_status(region: *cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_get_extents(region: *cairo_region_t,
                                    extents: *mut cairo_rectangle_int_t);
    pub fn cairo_region_num_rectangles(region: *cairo_region_t) ->
     ::libc::c_int;
    pub fn cairo_region_get_rectangle(region: *cairo_region_t,
                                      nth: ::libc::c_int,
                                      rectangle: *mut cairo_rectangle_int_t);
    pub fn cairo_region_is_empty(region: *cairo_region_t) -> cairo_bool_t;
    pub fn cairo_region_contains_rectangle(region: *cairo_region_t,
                                           rectangle: *cairo_rectangle_int_t)
     -> cairo_region_overlap_t;
    pub fn cairo_region_contains_point(region: *cairo_region_t,
                                       x: ::libc::c_int, y: ::libc::c_int) ->
     cairo_bool_t;
    pub fn cairo_region_translate(region: *mut cairo_region_t,
                                  dx: ::libc::c_int, dy: ::libc::c_int);
    pub fn cairo_region_subtract(dst: *mut cairo_region_t,
                                 other: *cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_subtract_rectangle(dst: *mut cairo_region_t,
                                           rectangle: *cairo_rectangle_int_t)
     -> cairo_status_t;
    pub fn cairo_region_intersect(dst: *mut cairo_region_t,
                                  other: *cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_intersect_rectangle(dst: *mut cairo_region_t,
                                            rectangle: *cairo_rectangle_int_t)
     -> cairo_status_t;
    pub fn cairo_region_union(dst: *mut cairo_region_t,
                              other: *cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_union_rectangle(dst: *mut cairo_region_t,
                                        rectangle: *cairo_rectangle_int_t) ->
     cairo_status_t;
    pub fn cairo_region_xor(dst: *mut cairo_region_t, other: *cairo_region_t)
     -> cairo_status_t;
    pub fn cairo_region_xor_rectangle(dst: *mut cairo_region_t,
                                      rectangle: *cairo_rectangle_int_t) ->
     cairo_status_t;
    pub fn cairo_debug_reset_static_data();
    pub fn cairo_pdf_surface_create(filename: *::libc::c_char,
                                    width_in_points: ::libc::c_double,
                                    height_in_points: ::libc::c_double) ->
     *mut cairo_surface_t;
    pub fn cairo_pdf_surface_create_for_stream(write_func: cairo_write_func_t,
                                               closure: *mut ::libc::c_void,
                                               width_in_points:
                                                   ::libc::c_double,
                                               height_in_points:
                                                   ::libc::c_double) ->
     *mut cairo_surface_t;
    pub fn cairo_pdf_surface_restrict_to_version(surface:
                                                     *mut cairo_surface_t,
                                                 version:
                                                     cairo_pdf_version_t);
    pub fn cairo_pdf_get_versions(versions: *mut *cairo_pdf_version_t,
                                  num_versions: *mut ::libc::c_int);
    pub fn cairo_pdf_version_to_string(version: cairo_pdf_version_t) ->
     *::libc::c_char;
    pub fn cairo_pdf_surface_set_size(surface: *mut cairo_surface_t,
                                      width_in_points: ::libc::c_double,
                                      height_in_points: ::libc::c_double);
}
