use derive_try_from_primitive::TryFromPrimitive;
use smallvec::SmallVec;
use std::convert::{TryFrom, TryInto};
use std::rc::Rc;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum Enum {
    POINTS = 0x0000,
    LINES = 0x0001,
    LINE_LOOP = 0x0002,
    LINE_STRIP = 0x0003,
    TRIANGLES = 0x0004,
    TRIANGLE_STRIP = 0x0005,
    TRIANGLE_FAN = 0x0006,
    QUADS = 0x0007,
    QUAD_STRIP = 0x0008,
    POLYGON = 0x0009,

    ACCUM = 0x0100,
    LOAD = 0x0101,
    RETURN = 0x0102,
    MULT = 0x0103,
    ADD = 0x0104,

    NEVER = 0x0200,
    LESS = 0x0201,
    EQUAL = 0x0202,
    LEQUAL = 0x0203,
    GREATER = 0x0204,
    NOTEQUAL = 0x0205,
    GEQUAL = 0x0206,
    ALWAYS = 0x0207,

    SRC_COLOR = 0x0300,
    ONE_MINUS_SRC_COLOR = 0x0301,
    SRC_ALPHA = 0x0302,
    ONE_MINUS_SRC_ALPHA = 0x0303,
    DST_ALPHA = 0x0304,
    ONE_MINUS_DST_ALPHA = 0x0305,
    DST_COLOR = 0x0306,
    ONE_MINUS_DST_COLOR = 0x0307,
    SRC_ALPHA_SATURATE = 0x0308,

    FRONT_LEFT = 0x0400,
    FRONT_RIGHT = 0x0401,
    BACK_LEFT = 0x0402,
    BACK_RIGHT = 0x0403,
    FRONT = 0x0404,
    BACK = 0x0405,
    LEFT = 0x0406,
    RIGHT = 0x0407,
    FRONT_AND_BACK = 0x0408,
    AUX0 = 0x0409,
    AUX1 = 0x040A,
    AUX2 = 0x040B,
    AUX3 = 0x040C,

    INVALID_ENUM = 0x0500,
    INVALID_VALUE = 0x0501,
    INVALID_OPERATION = 0x0502,
    STACK_OVERFLOW = 0x0503,
    STACK_UNDERFLOW = 0x0504,
    OUT_OF_MEMORY = 0x0505,

    _2D = 0x0600,
    _3D = 0x0601,
    _3D_COLOR = 0x0602,
    _3D_COLOR_TEXTURE = 0x0603,
    _4D_COLOR_TEXTURE = 0x0604,

    PASS_THROUGH_TOKEN = 0x0700,
    POINT_TOKEN = 0x0701,
    LINE_TOKEN = 0x0702,
    POLYGON_TOKEN = 0x0703,
    BITMAP_TOKEN = 0x0704,
    DRAW_PIXEL_TOKEN = 0x0705,
    COPY_PIXEL_TOKEN = 0x0706,
    LINE_RESET_TOKEN = 0x0707,

    EXP = 0x0800,
    EXP2 = 0x0801,

    CW = 0x0900,
    CCW = 0x0901,

    COEFF = 0x0A00,
    ORDER = 0x0A01,
    DOMAIN = 0x0A02,

    CURRENT_COLOR = 0x0B00,
    CURRENT_INDEX = 0x0B01,
    CURRENT_NORMAL = 0x0B02,
    CURRENT_TEXTURE_COORDS = 0x0B03,
    CURRENT_RASTER_COLOR = 0x0B04,
    CURRENT_RASTER_INDEX = 0x0B05,
    CURRENT_RASTER_TEXTURE_COORDS = 0x0B06,
    CURRENT_RASTER_POSITION = 0x0B07,
    CURRENT_RASTER_POSITION_VALID = 0x0B08,
    CURRENT_RASTER_DISTANCE = 0x0B09,

    POINT_SMOOTH = 0x0B10,
    POINT_SIZE = 0x0B11,
    POINT_SIZE_RANGE = 0x0B12,
    POINT_SIZE_GRANULARITY = 0x0B13,

    LINE_SMOOTH = 0x0B20,
    LINE_WIDTH = 0x0B21,
    LINE_WIDTH_RANGE = 0x0B22,
    LINE_WIDTH_GRANULARITY = 0x0B23,
    LINE_STIPPLE = 0x0B24,
    LINE_STIPPLE_PATTERN = 0x0B25,
    LINE_STIPPLE_REPEAT = 0x0B26,

    LIST_MODE = 0x0B30,
    MAX_LIST_NESTING = 0x0B31,
    LIST_BASE = 0x0B32,
    LIST_INDEX = 0x0B33,

    POLYGON_MODE = 0x0B40,
    POLYGON_SMOOTH = 0x0B41,
    POLYGON_STIPPLE = 0x0B42,
    EDGE_FLAG = 0x0B43,
    CULL_FACE = 0x0B44,
    CULL_FACE_MODE = 0x0B45,
    FRONT_FACE = 0x0B46,

    LIGHTING = 0x0B50,
    LIGHT_MODEL_LOCAL_VIEWER = 0x0B51,
    LIGHT_MODEL_TWO_SIDE = 0x0B52,
    LIGHT_MODEL_AMBIENT = 0x0B53,
    SHADE_MODEL = 0x0B54,
    COLOR_MATERIAL_FACE = 0x0B55,
    COLOR_MATERIAL_PARAMETER = 0x0B56,
    COLOR_MATERIAL = 0x0B57,

    FOG = 0x0B60,
    FOG_INDEX = 0x0B61,
    FOG_DENSITY = 0x0B62,
    FOG_START = 0x0B63,
    FOG_END = 0x0B64,
    FOG_MODE = 0x0B65,
    FOG_COLOR = 0x0B66,

    DEPTH_RANGE = 0x0B70,
    DEPTH_TEST = 0x0B71,
    DEPTH_WRITEMASK = 0x0B72,
    DEPTH_CLEAR_VALUE = 0x0B73,
    DEPTH_FUNC = 0x0B74,

    ACCUM_CLEAR_VALUE = 0x0B80,

    STENCIL_TEST = 0x0B90,
    STENCIL_CLEAR_VALUE = 0x0B91,
    STENCIL_FUNC = 0x0B92,
    STENCIL_VALUE_MASK = 0x0B93,
    STENCIL_FAIL = 0x0B94,
    STENCIL_PASS_DEPTH_FAIL = 0x0B95,
    STENCIL_PASS_DEPTH_PASS = 0x0B96,
    STENCIL_REF = 0x0B97,
    STENCIL_WRITEMASK = 0x0B98,

    MATRIX_MODE = 0x0BA0,
    NORMALIZE = 0x0BA1,
    VIEWPORT = 0x0BA2,
    MODELVIEW_STACK_DEPTH = 0x0BA3,
    PROJECTION_STACK_DEPTH = 0x0BA4,
    TEXTURE_STACK_DEPTH = 0x0BA5,
    MODELVIEW_MATRIX = 0x0BA6,
    PROJECTION_MATRIX = 0x0BA7,
    TEXTURE_MATRIX = 0x0BA8,

    ATTRIB_STACK_DEPTH = 0x0BB0,

    ALPHA_TEST = 0x0BC0,
    ALPHA_TEST_FUNC = 0x0BC1,
    ALPHA_TEST_REF = 0x0BC2,

    DITHER = 0x0BD0,

    BLEND_DST = 0x0BE0,
    BLEND_SRC = 0x0BE1,
    BLEND = 0x0BE2,

    LOGIC_OP_MODE = 0x0BF0,
    LOGIC_OP = 0x0BF1,

    AUX_BUFFERS = 0x0C00,
    DRAW_BUFFER = 0x0C01,
    READ_BUFFER = 0x0C02,

    SCISSOR_BOX = 0x0C10,
    SCISSOR_TEST = 0x0C11,

    INDEX_CLEAR_VALUE = 0x0C20,
    INDEX_WRITEMASK = 0x0C21,
    COLOR_CLEAR_VALUE = 0x0C22,
    COLOR_WRITEMASK = 0x0C23,

    INDEX_MODE = 0x0C30,
    RGBA_MODE = 0x0C31,
    DOUBLEBUFFER = 0x0C32,
    STEREO = 0x0C33,

    RENDER_MODE = 0x0C40,

    PERSPECTIVE_CORRECTION_HINT = 0x0C50,
    POINT_SMOOTH_HINT = 0x0C51,
    LINE_SMOOTH_HINT = 0x0C52,
    POLYGON_SMOOTH_HINT = 0x0C53,
    FOG_HINT = 0x0C54,

    TEXTURE_GEN_S = 0x0C60,
    TEXTURE_GEN_T = 0x0C61,
    TEXTURE_GEN_R = 0x0C62,
    TEXTURE_GEN_Q = 0x0C63,

    PIXEL_MAP_I_TO_I = 0x0C70,
    PIXEL_MAP_S_TO_S = 0x0C71,
    PIXEL_MAP_I_TO_R = 0x0C72,
    PIXEL_MAP_I_TO_G = 0x0C73,
    PIXEL_MAP_I_TO_B = 0x0C74,
    PIXEL_MAP_I_TO_A = 0x0C75,
    PIXEL_MAP_R_TO_R = 0x0C76,
    PIXEL_MAP_G_TO_G = 0x0C77,
    PIXEL_MAP_B_TO_B = 0x0C78,
    PIXEL_MAP_A_TO_A = 0x0C79,

    PIXEL_MAP_I_TO_I_SIZE = 0x0CB0,
    PIXEL_MAP_S_TO_S_SIZE = 0x0CB1,
    PIXEL_MAP_I_TO_R_SIZE = 0x0CB2,
    PIXEL_MAP_I_TO_G_SIZE = 0x0CB3,
    PIXEL_MAP_I_TO_B_SIZE = 0x0CB4,
    PIXEL_MAP_I_TO_A_SIZE = 0x0CB5,
    PIXEL_MAP_R_TO_R_SIZE = 0x0CB6,
    PIXEL_MAP_G_TO_G_SIZE = 0x0CB7,
    PIXEL_MAP_B_TO_B_SIZE = 0x0CB8,
    PIXEL_MAP_A_TO_A_SIZE = 0x0CB9,

    UNPACK_SWAP_BYTES = 0x0CF0,
    UNPACK_LSB_FIRST = 0x0CF1,
    UNPACK_ROW_LENGTH = 0x0CF2,
    UNPACK_SKIP_ROWS = 0x0CF3,
    UNPACK_SKIP_PIXELS = 0x0CF4,
    UNPACK_ALIGNMENT = 0x0CF5,

    PACK_SWAP_BYTES = 0x0D00,
    PACK_LSB_FIRST = 0x0D01,
    PACK_ROW_LENGTH = 0x0D02,
    PACK_SKIP_ROWS = 0x0D03,
    PACK_SKIP_PIXELS = 0x0D04,
    PACK_ALIGNMENT = 0x0D05,

    MAP_COLOR = 0x0D10,
    MAP_STENCIL = 0x0D11,
    INDEX_SHIFT = 0x0D12,
    INDEX_OFFSET = 0x0D13,
    RED_SCALE = 0x0D14,
    RED_BIAS = 0x0D15,
    ZOOM_X = 0x0D16,
    ZOOM_Y = 0x0D17,
    GREEN_SCALE = 0x0D18,
    GREEN_BIAS = 0x0D19,
    BLUE_SCALE = 0x0D1A,
    BLUE_BIAS = 0x0D1B,
    ALPHA_SCALE = 0x0D1C,
    ALPHA_BIAS = 0x0D1D,
    DEPTH_SCALE = 0x0D1E,
    DEPTH_BIAS = 0x0D1F,

    MAX_EVAL_ORDER = 0x0D30,
    MAX_LIGHTS = 0x0D31,
    MAX_CLIP_PLANES = 0x0D32,
    MAX_TEXTURE_SIZE = 0x0D33,
    MAX_PIXEL_MAP_TABLE = 0x0D34,
    MAX_ATTRIB_STACK_DEPTH = 0x0D35,
    MAX_MODELVIEW_STACK_DEPTH = 0x0D36,
    MAX_NAME_STACK_DEPTH = 0x0D37,
    MAX_PROJECTION_STACK_DEPTH = 0x0D38,
    MAX_TEXTURE_STACK_DEPTH = 0x0D39,
    MAX_VIEWPORT_DIMS = 0x0D3A,

    SUBPIXEL_BITS = 0x0D50,
    INDEX_BITS = 0x0D51,
    RED_BITS = 0x0D52,
    GREEN_BITS = 0x0D53,
    BLUE_BITS = 0x0D54,
    ALPHA_BITS = 0x0D55,
    DEPTH_BITS = 0x0D56,
    STENCIL_BITS = 0x0D57,
    ACCUM_RED_BITS = 0x0D58,
    ACCUM_GREEN_BITS = 0x0D59,
    ACCUM_BLUE_BITS = 0x0D5A,
    ACCUM_ALPHA_BITS = 0x0D5B,

    NAME_STACK_DEPTH = 0x0D70,

    AUTO_NORMAL = 0x0D80,

    MAP1_COLOR_4 = 0x0D90,
    MAP1_INDEX = 0x0D91,
    MAP1_NORMAL = 0x0D92,
    MAP1_TEXTURE_COORD_1 = 0x0D93,
    MAP1_TEXTURE_COORD_2 = 0x0D94,
    MAP1_TEXTURE_COORD_3 = 0x0D95,
    MAP1_TEXTURE_COORD_4 = 0x0D96,
    MAP1_VERTEX_3 = 0x0D97,
    MAP1_VERTEX_4 = 0x0D98,

    MAP2_COLOR_4 = 0x0DB0,
    MAP2_INDEX = 0x0DB1,
    MAP2_NORMAL = 0x0DB2,
    MAP2_TEXTURE_COORD_1 = 0x0DB3,
    MAP2_TEXTURE_COORD_2 = 0x0DB4,
    MAP2_TEXTURE_COORD_3 = 0x0DB5,
    MAP2_TEXTURE_COORD_4 = 0x0DB6,
    MAP2_VERTEX_3 = 0x0DB7,
    MAP2_VERTEX_4 = 0x0DB8,

    MAP1_GRID_DOMAIN = 0x0DD0,
    MAP1_GRID_SEGMENTS = 0x0DD1,
    MAP2_GRID_DOMAIN = 0x0DD2,
    MAP2_GRID_SEGMENTS = 0x0DD3,

    TEXTURE_1D = 0x0DE0,
    TEXTURE_2D = 0x0DE1,

    TEXTURE_WIDTH = 0x1000,
    TEXTURE_HEIGHT = 0x1001,
    TEXTURE_COMPONENTS = 0x1003,
    TEXTURE_BORDER_COLOR = 0x1004,
    TEXTURE_BORDER = 0x1005,

    DONT_CARE = 0x1100,
    FASTEST = 0x1101,
    NICEST = 0x1102,

    AMBIENT = 0x1200,
    DIFFUSE = 0x1201,
    SPECULAR = 0x1202,
    POSITION = 0x1203,
    SPOT_DIRECTION = 0x1204,
    SPOT_EXPONENT = 0x1205,
    SPOT_CUTOFF = 0x1206,
    CONSTANT_ATTENUATION = 0x1207,
    LINEAR_ATTENUATION = 0x1208,
    QUADRATIC_ATTENUATION = 0x1209,

    COMPILE = 0x1300,
    COMPILE_AND_EXECUTE = 0x1301,

    BYTE = 0x1400,
    UNSIGNED_BYTE = 0x1401,
    SHORT = 0x1402,
    UNSIGNED_SHORT = 0x1403,
    INT = 0x1404,
    UNSIGNED_INT = 0x1405,
    FLOAT = 0x1406,
    _2_BYTES = 0x1407,
    _3_BYTES = 0x1408,
    _4_BYTES = 0x1409,

    CLEAR = 0x1500,
    AND = 0x1501,
    AND_REVERSE = 0x1502,
    COPY = 0x1503,
    AND_INVERTED = 0x1504,
    NOOP = 0x1505,
    XOR = 0x1506,
    OR = 0x1507,
    NOR = 0x1508,
    EQUIV = 0x1509,
    INVERT = 0x150A,
    OR_REVERSE = 0x150B,
    COPY_INVERTED = 0x150C,
    OR_INVERTED = 0x150D,
    NAND = 0x150E,
    SET = 0x150F,

    EMISSION = 0x1600,
    SHININESS = 0x1601,
    AMBIENT_AND_DIFFUSE = 0x1602,
    COLOR_INDEXES = 0x1603,

    MODELVIEW = 0x1700,
    PROJECTION = 0x1701,
    TEXTURE = 0x1702,

    COLOR = 0x1800,
    DEPTH = 0x1801,
    STENCIL = 0x1802,

    COLOR_INDEX = 0x1900,
    STENCIL_INDEX = 0x1901,
    DEPTH_COMPONENT = 0x1902,
    RED = 0x1903,
    GREEN = 0x1904,
    BLUE = 0x1905,
    ALPHA = 0x1906,
    RGB = 0x1907,
    RGBA = 0x1908,
    LUMINANCE = 0x1909,
    LUMINANCE_ALPHA = 0x190A,

    BITMAP = 0x1A00,

    POINT = 0x1B00,
    LINE = 0x1B01,
    FILL = 0x1B02,

    RENDER = 0x1C00,
    FEEDBACK = 0x1C01,
    SELECT = 0x1C02,

    FLAT = 0x1D00,
    SMOOTH = 0x1D01,

    KEEP = 0x1E00,
    REPLACE = 0x1E01,
    INCR = 0x1E02,
    DECR = 0x1E03,

    VENDOR = 0x1F00,
    RENDERER = 0x1F01,
    VERSION = 0x1F02,
    EXTENSIONS = 0x1F03,

    S = 0x2000,
    T = 0x2001,
    R = 0x2002,
    Q = 0x2003,

    MODULATE = 0x2100,
    DECAL = 0x2101,

    TEXTURE_ENV_MODE = 0x2200,
    TEXTURE_ENV_COLOR = 0x2201,

    TEXTURE_ENV = 0x2300,

    EYE_LINEAR = 0x2400,
    OBJECT_LINEAR = 0x2401,
    SPHERE_MAP = 0x2402,

    TEXTURE_GEN_MODE = 0x2500,
    OBJECT_PLANE = 0x2501,
    EYE_PLANE = 0x2502,

    NEAREST = 0x2600,
    LINEAR = 0x2601,

    NEAREST_MIPMAP_NEAREST = 0x2700,
    LINEAR_MIPMAP_NEAREST = 0x2701,
    NEAREST_MIPMAP_LINEAR = 0x2702,
    LINEAR_MIPMAP_LINEAR = 0x2703,

    TEXTURE_MAG_FILTER = 0x2800,
    TEXTURE_MIN_FILTER = 0x2801,
    TEXTURE_WRAP_S = 0x2802,
    TEXTURE_WRAP_T = 0x2803,

    CLAMP = 0x2900,
    REPEAT = 0x2901,

    CLIP_PLANE0 = 0x3000,
    CLIP_PLANE1 = 0x3001,
    CLIP_PLANE2 = 0x3002,
    CLIP_PLANE3 = 0x3003,
    CLIP_PLANE4 = 0x3004,
    CLIP_PLANE5 = 0x3005,

    LIGHT0 = 0x4000,
    LIGHT1 = 0x4001,
    LIGHT2 = 0x4002,
    LIGHT3 = 0x4003,
    LIGHT4 = 0x4004,
    LIGHT5 = 0x4005,
    LIGHT6 = 0x4006,
    LIGHT7 = 0x4007,
}

macro_rules! commands {
    (@type enum) => {Enum};
    (@ffi_type enum) => {u32};
    (@from_ffi($gl:ident) $name:ident: enum) => {$name.try_into().unwrap()};

    // FIXME(eddyb) model these properly
    (@type bitfield) => {u32};
    (@ffi_type bitfield) => {u32};
    (@from_ffi($gl:ident) $name:ident: bitfield) => {$name};

    (@type int) => {i32};
    (@ffi_type int) => {i32};
    (@from_ffi($gl:ident) $name:ident: int) => {$name};

    (@type sizei) => {u32};
    (@ffi_type sizei) => {i32};
    (@from_ffi($gl:ident) $name:ident: sizei) => {$name.try_into().unwrap()};

    (@type float) => {f32};
    (@ffi_type float) => {f32};
    (@from_ffi($gl:ident) $name:ident: float) => {$name};

    (@type double) => {f64};
    (@ffi_type double) => {f64};
    (@from_ffi($gl:ident) $name:ident: double) => {$name};

    (@type [$elem:tt; dyn $len:block]) => {SmallVec<[commands!(@type $elem); 4]>};
    (@ffi_type [$elem:tt; dyn $len:block]) => {*const commands!(@ffi_type $elem)};
    (@from_ffi($gl:ident) $name:ident: [$elem:tt; dyn $len:block]) => {
        unsafe { std::slice::from_raw_parts($name, $len) }
            .iter()
            .map(|&x| commands!(@from_ffi($gl) x: $elem))
            .collect()
    };

    (@type list) => {Rc<super::DisplayList>};
    (@ffi_type list) => {u32};
    (@from_ffi($gl:ident) $name:ident: list) => {$gl.lists[&$name].clone()};

    ($($name:ident $(($($param:ident: $ty:tt),*))?),*) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug)]
        pub enum Command {
            $($name $(($(commands!(@type $ty)),*))?),*
        }

        impl Command {
            fn submit(self, gl: &mut super::Context) {
                if let Some((_, list)) = &mut gl.compile_list {
                    if gl.execute_immediately {
                        gl.pending_cmds.push(self.clone());
                    }
                    list.cmds.push(self);
                } else {
                    assert!(gl.execute_immediately);
                    gl.pending_cmds.push(self);
                }
            }
        }

        $(#[no_mangle]
        pub extern "C" fn $name($($($param: commands!(@ffi_type $ty)),*)?) {
            super::Context::with(stringify!($name), |gl| {
                $($(let $param = commands!(@from_ffi(gl) $param: $ty);)*)?
                Command::$name $(($($param),*))?.submit(gl);
            });
        })*
    };
}

commands! {
    glClear(buf: bitfield),

    glEnable(target: enum),

    glMaterialfv(face: enum, pname: enum, params: [float; dyn {
        use Enum::*;
        match pname {
            AMBIENT | DIFFUSE | AMBIENT_AND_DIFFUSE | SPECULAR | EMISSION => 4,

            SHININESS => 1,

            COLOR_INDEXES => 3,

            // FIXME(eddyb) report error
            _ => unimplemented!("glMaterialfv(pname={:?})", pname),
        }
    }]),
    glLightfv(light: enum, pname: enum, params: [float; dyn {
        use Enum::*;
        match pname {
            AMBIENT | DIFFUSE | SPECULAR | POSITION => 4,

            SPOT_DIRECTION => 3,

            SPOT_EXPONENT
            | SPOT_CUTOFF
            | CONSTANT_ATTENUATION
            | LINEAR_ATTENUATION
            | QUADRATIC_ATTENUATION => 1,

            // FIXME(eddyb) report error
            _ => unimplemented!("glLightfv(pname={:?})", pname),
        }
    }]),
    glShadeModel(mode: enum),

    glBegin(mode: enum),
    glVertex3f(x: float, y: float, z: float),
    glNormal3f(x: float, y: float, z: float),
    glEnd,

    glViewport(x: int, y: int, w: sizei, h: sizei),

    glMatrixMode(mode: enum),
    glPushMatrix,
    glPopMatrix,
    glLoadIdentity,
    glRotatef(angle: float, x: float, y: float, z: float),
    glTranslatef(x: float, y: float, z: float),
    glFrustum(l: double, r: double, b: double, t: double, n: double, f: double),

    glCallList(n: list)
}

#[no_mangle]
pub extern "C" fn glGenLists(s: i32) -> u32 {
    super::Context::with("glGenLists", |gl| {
        let n = gl.first_unused_list;
        gl.first_unused_list += s as u32;
        n
    })
}

#[no_mangle]
pub extern "C" fn glNewList(n: u32, mode: u32) {
    let mode = Enum::try_from(mode).unwrap();
    super::Context::with("glNewList", |gl| {
        // FIXME(eddyb) report error
        assert!(gl.compile_list.is_none());
        gl.compile_list = Some((n, super::DisplayList::default()));
        gl.execute_immediately = {
            use Enum::*;
            match mode {
                COMPILE => false,
                COMPILE_AND_EXECUTE => true,
                // FIXME(eddyb) report error
                _ => unimplemented!("glNewList(mode={:?})", mode),
            }
        };
    });
}

#[no_mangle]
pub extern "C" fn glEndList() {
    super::Context::with("glEndList", |gl| {
        // FIXME(eddyb) report error
        let (n, list) = gl.compile_list.take().unwrap();
        gl.lists.insert(n, Rc::new(list));
        gl.execute_immediately = true;
    });
}
