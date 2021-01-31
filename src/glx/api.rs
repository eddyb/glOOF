use lazy_static::lazy_static;
use libc::{c_char, c_int, c_uchar, c_ulong, free, malloc};
use std::ffi::{CStr, CString};
use std::sync::Arc;
use std::{iter, mem, ptr};
use x11_dl::glx::{
    GLXContext, GLXDrawable, GLXFBConfig, GLX_DONT_CARE, GLX_EXTENSIONS, GLX_NONE, GLX_RGBA_BIT,
    GLX_VENDOR, GLX_VERSION, GLX_WINDOW_BIT,
};
use x11_dl::xlib::{Bool, Display, False, InputOutput, Success, True, XVisualInfo, Xlib};

lazy_static! {
    static ref XLIB: Xlib = Xlib::open().unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn glXGetProcAddressARB(
    proc_name: *const c_uchar,
) -> Option<unsafe extern "C" fn()> {
    let proc_name = CStr::from_ptr(proc_name as *const c_char).to_str().unwrap();

    eprintln!("glXGetProcAddressARB({:?})", proc_name);

    macro_rules! export {
        ($($name:ident)*) => {
            match proc_name {
                $(stringify!($name) => Some(mem::transmute($name as usize)),)*
                _ => None
            }
        };
    }
    // FIXME(eddyb) DRY with the actual entry-point definitions
    export! {
        glXQueryVersion
        glXGetClientString
        glXQueryServerString
        glXQueryExtensionsString
        glXChooseVisual
        glXGetConfig
        glXGetFBConfigs
        glXChooseFBConfig
        glXGetFBConfigAttrib
        glXGetVisualFromFBConfig
        glXCreateWindow
        glXDestroyWindow
        glXCreateContext
        glXIsDirect
        glXMakeCurrent
        glXDestroyContext
        glXSwapBuffers

        // glxgears
        glXQueryDrawable

        // wine
        glXCopyContext
        glXCreateNewContext
        glXCreatePbuffer
        glXCreatePixmap
        glXDestroyPbuffer
        glXDestroyPixmap
        glXGetCurrentContext
        glXGetCurrentDrawable
        glXMakeContextCurrent
    }
}

#[no_mangle]
pub unsafe extern "C" fn glXQueryVersion(
    _dpy: *mut Display,
    major: *mut c_int,
    minor: *mut c_int,
) -> Bool {
    eprintln!("glXQueryVersion()");

    // wine needs either GLX 1.3 or some vendor extensions.
    *major = 1;
    *minor = 3;

    True
}

#[no_mangle]
pub unsafe extern "C" fn glXGetClientString(_dpy: *mut Display, name: c_int) -> *const c_char {
    eprintln!("glXGetClientString(name={})", name);

    match name {
        GLX_VENDOR => "glOOF\0",
        GLX_VERSION => concat!(version_str!(major.minor), "\0"),
        GLX_EXTENSIONS => "\0",
        _ => return ptr::null(),
    }
    .as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn glXQueryServerString(
    _dpy: *mut Display,
    screen: c_int,
    name: c_int,
) -> *const c_char {
    assert_eq!(screen, 0);

    eprintln!("glXQueryServerString(name={})", name);

    match name {
        GLX_VENDOR => "glOOF\0",
        GLX_VERSION => concat!(version_str!(major.minor), "\0"),
        GLX_EXTENSIONS => "\0",
        _ => return ptr::null(),
    }
    .as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn glXQueryExtensionsString(
    _dpy: *mut Display,
    screen: c_int,
) -> *const c_char {
    assert_eq!(screen, 0);

    eprintln!("glXQueryExtensionsString()");

    "\0".as_ptr() as *const c_char
}

macro_rules! attribs {
    (@type(Visual) bool) => {bool};
    (@type(FBConfig) bool) => {Option<bool>};
    (@type($mode:ident) int) => {i32};
    (@type(FBConfig) bitmask) => {u32};
    (@type(FBConfig) enum) => {u32};

    (@parse(Visual, $next:expr) bool) => {true};
    (@parse(FBConfig, $next:expr) bool) => {{
        #![allow(non_upper_case_globals)]
        match $next() {
            GLX_DONT_CARE => None,
            False => Some(false),
            True => Some(true),
            value => panic!("FBConfigAttrib::parse: invalid bool value {}", value),
        }
    }};
    (@parse($mode:ident, $next:expr) int) => {$next()};
    (@parse(FBConfig, $next:expr) bitmask) => {$next() as u32};
    (@parse(FBConfig, $next:expr) enum) => {$next() as u32};

    (@define($mode:ident, enum $Attrib:ident, struct $Attribs:ident) {
        $($name:ident: $ty:ident $(= $default:expr)?),* $(,)?
    }) => {
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, Debug)]
        enum $Attrib {
            $($name(attribs!(@type($mode) $ty))),*
        }
        impl $Attrib {
            fn name(&self) -> &'static str {
                match self {
                    $(Self::$name(_) => stringify!($name)),*
                }
            }

            fn parse(attrib: c_int, next: impl FnOnce() -> c_int) -> Self {
                match attrib {
                    $(x11_dl::glx::$name => Self::$name(attribs!(@parse($mode, next) $ty)),)*
                    attrib => panic!(concat!(stringify!($Attrib), "::parse: invalid attribute {}"), attrib),
                }
            }

            unsafe fn parse_list(mut list: *const c_int) -> impl Iterator<Item = Self> {
                iter::from_fn(move || {
                    if *list == 0 {
                        return None;
                    }
                    let mut next = || {
                        let attrib = *list;
                        list = list.add(1);
                        attrib
                    };
                    Some(Self::parse(next(), next))
                })
            }
        }

        #[allow(non_snake_case)]
        #[derive(Copy, Clone, Debug)]
        struct $Attribs {
            $($name: attribs!(@type($mode) $ty)),*
        }
        impl Default for $Attribs {
            fn default() -> Self {
                let mut default = Self {
                    $($name: Default::default()),*
                };
                default.extend([
                    $($($Attrib::parse(x11_dl::glx::$name, || $default),)?)*
                ].iter().copied());
                default
            }
        }
        impl Extend<$Attrib> for $Attribs {
            fn extend<I: IntoIterator<Item = $Attrib>>(&mut self, iter: I) {
                for attrib in iter {
                    match attrib {
                        $($Attrib::$name(x) => self.$name = x),*
                    }
                }
            }
        }
        impl iter::FromIterator<$Attrib> for $Attribs {
            fn from_iter<I: IntoIterator<Item = $Attrib>>(iter: I) -> Self {
                let mut attribs = Self::default();
                attribs.extend(iter);
                attribs
            }
        }
    };

    (
        Visual { $($visual_name:ident: $visual_ty:ident $(= $visual_default:expr)?),* $(,)? }
        VisualAndFBConfig { $($common_name:ident: $common_ty:ident $(= $common_default:expr)?),* $(,)? }
        FBConfig { $($fbconfig_name:ident: $fbconfig_ty:ident $(= $fbconfig_default:expr)?),* $(,)? }
    ) => {
        attribs!(@define(Visual, enum VisualAttrib, struct VisualAttribs) {
            $($visual_name: $visual_ty $( = $visual_default)?,)*
            $($common_name: $common_ty $( = $common_default)?,)*
        });
        attribs!(@define(FBConfig, enum FBConfigAttrib, struct FBConfigAttribs) {
            $($common_name: $common_ty $( = $common_default)?,)*
            $($fbconfig_name: $fbconfig_ty $( = $fbconfig_default)?,)*
        });
    };
}

// FIXME(eddyb) replace `GLX_DONT_CARE` with `Option`?
attribs! {
    Visual {
        GLX_USE_GL: bool = True,
        GLX_RGBA: bool,

        // NOTE(eddyb) common but `bool` defaults are different:
        // `false` for `Visual` vs `None::<bool>` for `FBConfig`.
        GLX_STEREO: bool,
    }

    VisualAndFBConfig {
        // NOTE(eddyb) allowed for Visual but ignored.
        GLX_FBCONFIG_ID: int = GLX_DONT_CARE,

        GLX_BUFFER_SIZE: int,
        GLX_LEVEL: int,
        GLX_DOUBLEBUFFER: bool,
        GLX_AUX_BUFFERS: int,
        GLX_RED_SIZE: int,
        GLX_GREEN_SIZE: int,
        GLX_BLUE_SIZE: int,
        GLX_ALPHA_SIZE: int,
        GLX_DEPTH_SIZE: int,
        GLX_STENCIL_SIZE: int,
        GLX_ACCUM_RED_SIZE: int,
        GLX_ACCUM_GREEN_SIZE: int,
        GLX_ACCUM_BLUE_SIZE: int,
        GLX_ACCUM_ALPHA_SIZE: int,
    }

    FBConfig {
        // NOTE(eddyb) common but `bool` defaults are different:
        // `false` for `Visual` vs `None::<bool>` for `FBConfig`.
        GLX_STEREO: bool = False,

        // FIXME(eddyb) encode the information in the comments below, into types.
        // bitmask { GLX_RGBA_BIT | GLX_COLOR_INDEX_BIT }
        GLX_RENDER_TYPE: bitmask = GLX_RGBA_BIT,
        // bitmask { GLX_WINDOW_BIT | GLX_PIXMAP_BIT | GLX_PBUFFER_BIT }
        GLX_DRAWABLE_TYPE: bitmask = GLX_WINDOW_BIT,
        GLX_X_RENDERABLE: bool = GLX_DONT_CARE,
        GLX_X_VISUAL_TYPE: int = GLX_DONT_CARE,
        // enum { GLX_NONE, GLX_SLOW_CONFIG, GLX_NON_CONFORMANT_CONFIG }
        GLX_CONFIG_CAVEAT: enum = GLX_DONT_CARE,
        // enum { GLX_NONE, GLX_TRANSPARENT_RGB, GLX_TRANSPARENT_INDEX }
        GLX_TRANSPARENT_TYPE: enum = GLX_NONE,
        GLX_TRANSPARENT_INDEX_VALUE: int = GLX_DONT_CARE,
        GLX_TRANSPARENT_RED_VALUE: int = GLX_DONT_CARE,
        GLX_TRANSPARENT_GREEN_VALUE: int = GLX_DONT_CARE,
        GLX_TRANSPARENT_BLUE_VALUE: int = GLX_DONT_CARE,
        GLX_TRANSPARENT_ALPHA_VALUE: int = GLX_DONT_CARE,
    }
}

const COLOR_DEPTH: c_int = 24;
const COLOR_CHANNEL_DEPTH: c_int = 8;

unsafe fn default_visual_info(dpy: *mut Display) -> *mut XVisualInfo {
    let visual_info = malloc(mem::size_of::<XVisualInfo>()) as *mut XVisualInfo;

    // HACK(eddyb) `XMatchVisualInfo` returns 0 (failure) for some reason.
    if false {
        if (XLIB.XMatchVisualInfo)(dpy, 0, COLOR_DEPTH, InputOutput, visual_info) == 0 {
            free(visual_info as *mut _);
            return ptr::null_mut();
        }
        eprintln!("visual_info = {:#?}", *visual_info);
    } else {
        visual_info.write(XVisualInfo {
            visual: (XLIB.XDefaultVisual)(dpy, 0),
            depth: COLOR_DEPTH,
            ..mem::zeroed()
        });
    }
    visual_info
}

#[no_mangle]
pub unsafe extern "C" fn glXChooseVisual(
    dpy: *mut Display,
    screen: c_int,
    attrib_list: *mut c_int,
) -> *mut XVisualInfo {
    assert_eq!(screen, 0);

    eprintln!("glXChooseVisual(attribList=[");
    for attrib in VisualAttrib::parse_list(attrib_list) {
        eprintln!("    {:?},", attrib);
    }
    eprintln!("])");

    eprintln!(
        "attribs = {:#?}",
        VisualAttrib::parse_list(attrib_list).collect::<VisualAttribs>()
    );

    default_visual_info(dpy)
}

#[no_mangle]
pub unsafe extern "C" fn glXGetConfig(
    _dpy: *mut Display,
    visual: *mut XVisualInfo,
    attrib: c_int,
    _value: *mut c_int,
) -> c_int {
    // FIXME(eddyb) make a separate `enum` for an attrib w/o values.
    let attrib = VisualAttrib::parse(attrib, || 0);

    unimplemented!(
        "glXGetConfig(visual={:#?}, attrib={})",
        *visual,
        attrib.name()
    );
}

#[no_mangle]
pub unsafe extern "C" fn glXGetFBConfigs(
    _dpy: *mut Display,
    screen: c_int,
    nelements: *mut c_int,
) -> *mut GLXFBConfig {
    assert_eq!(screen, 0);

    eprintln!("glXGetFBConfigs()");

    let fb_config_array = [ptr::null_mut()];

    let fb_configs = malloc(mem::size_of_val(&fb_config_array)) as *mut [GLXFBConfig; 1];
    fb_configs.write(fb_config_array);
    *nelements = fb_config_array.len() as c_int;
    fb_configs as *mut GLXFBConfig
}

#[no_mangle]
pub unsafe extern "C" fn glXChooseFBConfig(
    _dpy: *mut Display,
    screen: c_int,
    attrib_list: *const c_int,
    nelements: *mut c_int,
) -> *mut GLXFBConfig {
    assert_eq!(screen, 0);

    eprintln!("glXChooseFBConfig(attrib_list=[");
    for attrib in FBConfigAttrib::parse_list(attrib_list) {
        eprintln!("    {:?},", attrib);
    }
    eprintln!("])");

    eprintln!(
        "attribs = {:#?}",
        FBConfigAttrib::parse_list(attrib_list).collect::<FBConfigAttribs>()
    );

    let fb_config_array = [ptr::null_mut()];

    let fb_configs = malloc(mem::size_of_val(&fb_config_array)) as *mut [GLXFBConfig; 1];
    fb_configs.write(fb_config_array);
    *nelements = fb_config_array.len() as c_int;
    fb_configs as *mut GLXFBConfig
}

#[no_mangle]
pub unsafe extern "C" fn glXGetFBConfigAttrib(
    _dpy: *mut Display,
    config: GLXFBConfig,
    attribute: c_int,
    value: *mut c_int,
) -> c_int {
    assert_eq!(config, ptr::null_mut());

    // FIXME(eddyb) make a separate `enum` for an attrib w/o values.
    let attribute = FBConfigAttrib::parse(attribute, || 0);

    eprintln!("glXGetFBConfigAttrib(attribute={})", attribute.name());

    *value = match attribute {
        // HACK(eddyb) hardcode some values for our sole `GLXFBConfig`.
        FBConfigAttrib::GLX_FBCONFIG_ID(_) => 0,
        FBConfigAttrib::GLX_BUFFER_SIZE(_) => COLOR_DEPTH,
        FBConfigAttrib::GLX_DOUBLEBUFFER(_) => True,
        FBConfigAttrib::GLX_STEREO(_) => False,
        FBConfigAttrib::GLX_AUX_BUFFERS(_) => 0,
        FBConfigAttrib::GLX_RED_SIZE(_)
        | FBConfigAttrib::GLX_GREEN_SIZE(_)
        | FBConfigAttrib::GLX_BLUE_SIZE(_)
        | FBConfigAttrib::GLX_ALPHA_SIZE(_) => COLOR_CHANNEL_DEPTH,
        FBConfigAttrib::GLX_DEPTH_SIZE(_) => 16,
        FBConfigAttrib::GLX_STENCIL_SIZE(_) => 0,
        FBConfigAttrib::GLX_ACCUM_RED_SIZE(_)
        | FBConfigAttrib::GLX_ACCUM_GREEN_SIZE(_)
        | FBConfigAttrib::GLX_ACCUM_BLUE_SIZE(_)
        | FBConfigAttrib::GLX_ACCUM_ALPHA_SIZE(_) => 0,
        FBConfigAttrib::GLX_RENDER_TYPE(_) => GLX_RGBA_BIT,
        FBConfigAttrib::GLX_DRAWABLE_TYPE(_) => GLX_WINDOW_BIT,

        _ => unimplemented!(),
    };

    Success as c_int
}

#[no_mangle]
pub unsafe extern "C" fn glXGetVisualFromFBConfig(
    dpy: *mut Display,
    config: GLXFBConfig,
) -> *mut XVisualInfo {
    assert_eq!(config, ptr::null_mut());

    eprintln!("glXGetVisualFromFBConfig()");

    default_visual_info(dpy)
}

#[no_mangle]
pub unsafe extern "C" fn glXCreateWindow(
    _dpy: *mut Display,
    config: GLXFBConfig,
    win: c_ulong,
    attrib_list: *const c_int,
) -> c_ulong {
    assert_eq!(config, ptr::null_mut());
    if !attrib_list.is_null() {
        assert_eq!(*attrib_list, 0);
    }

    eprintln!("glXCreateWindow(win={:#x})", win);

    // HACK(eddyb) don't bother creating a child window inside `win`.
    win
}

#[no_mangle]
pub unsafe extern "C" fn glXDestroyWindow(_dpy: *mut Display, win: c_ulong) {
    eprintln!("glXDestroyWindow(win={:#x})", win);

    // NOTE(eddyb) don't have to do anything right now, see `glXCreateWindow`.
}

#[no_mangle]
pub unsafe extern "C" fn glXCreateContext(
    _dpy: *mut Display,
    vis: *mut XVisualInfo,
    share_list: GLXContext,
    direct: Bool,
) -> GLXContext {
    assert_eq!(share_list, ptr::null_mut());

    eprintln!(
        "glXCreateContext(vis={:#?}, direct={})",
        *vis,
        direct != False
    );

    Arc::into_raw(super::Context::new()) as GLXContext
}

#[no_mangle]
pub unsafe extern "C" fn glXIsDirect(_dpy: *mut Display, ctx: GLXContext) -> Bool {
    eprintln!("glXIsDirect(ctx={:#?})", ctx);

    assert!(!ctx.is_null());

    True
}

#[no_mangle]
pub unsafe extern "C" fn glXMakeCurrent(
    dpy: *mut Display,
    drawable: GLXDrawable,
    ctx: GLXContext,
) -> Bool {
    eprintln!("glXMakeCurrent(drawable={:#x}, ctx={:#?})", drawable, ctx);

    if ctx.is_null() {
        super::Context::remove_current();
    } else {
        mem::ManuallyDrop::new(Arc::from_raw(ctx as *mut super::Context))
            .make_current(super::Surface { drawable }, super::Surface { drawable });
    }

    let mut name = ptr::null_mut();
    (XLIB.XFetchName)(dpy, drawable, &mut name);
    if !name.is_null() {
        let new_name = CString::new(format!(
            "{} [glOOF]",
            CStr::from_ptr(name).to_str().unwrap()
        ))
        .unwrap();
        (XLIB.XStoreName)(dpy, drawable, new_name.as_ptr());
        free(name as *mut _);
    }

    True
}

#[no_mangle]
pub unsafe extern "C" fn glXDestroyContext(_dpy: *mut Display, ctx: GLXContext) {
    eprintln!("glXDestroyContext(ctx={:#?})", ctx);

    assert!(!ctx.is_null());

    // NOTE(eddyb) this will only drop one reference (the one lent to the user),
    // *not* the context itself, if it's still in use - this is according to spec:
    // > If `ctx` is still current to any thread, `ctx` is not destroyed until it
    // > is no longer current.
    drop(Arc::from_raw(ctx as *mut super::Context));
}

#[no_mangle]
pub unsafe extern "C" fn glXSwapBuffers(dpy: *mut Display, drawable: GLXDrawable) {
    use x11_dl::xlib::*;

    // eprintln!("glXSwapBuffers(drawable={:#x})", drawable);

    match &*super::Context::get_current().unwrap().0.lock().unwrap() {
        super::State::Inactive(_) => unreachable!("glOOF: inactive current GLX context"),
        super::State::Current { read, draw } => {
            assert_eq!(read.drawable, drawable);
            assert_eq!(draw.drawable, drawable);
        }
    }

    (XLIB.XSetWindowBackground)(dpy, drawable, 0);
    (XLIB.XClearWindow)(dpy, drawable);
    let gc = (XLIB.XCreateGC)(
        dpy,
        drawable,
        GCForeground as c_ulong,
        &XGCValues {
            foreground: 0xffffff,
            ..mem::zeroed()
        } as *const _ as *mut _,
    );

    crate::gl::Context::with("glXSwapBuffers", |gl| {
        let mut last_point = None;
        for cmd in gl.pending_cmds.drain(..) {
            gl.state.apply(cmd, &mut |state, cmd| {
                use crate::gl::api_1_0::Command::*;
                use glam::Vec3;
                match cmd {
                    glVertex3f(x, y, z) => {
                        let mut p = Vec3::new(x, y, z);
                        p = state.modelview.mat.transform_point3(p);
                        p = state.projection.mat.transform_point3(p);
                        match last_point {
                            None => last_point = Some(p),
                            Some(last) => {
                                (XLIB.XDrawLine)(
                                    dpy,
                                    drawable,
                                    gc,
                                    (last.x * 300.0 / 2.0 + 300.0 / 2.0) as _,
                                    (-last.y * 300.0 / 2.0 + 300.0 / 2.0) as _,
                                    (p.x * 300.0 / 2.0 + 300.0 / 2.0) as _,
                                    (-p.y * 300.0 / 2.0 + 300.0 / 2.0) as _,
                                );
                            }
                        }
                    }
                    _ => {
                        // eprintln!("{:?}", cmd);
                    }
                }
            });
        }
    });

    (XLIB.XFlush)(dpy);
}

macro_rules! unimplemented_entry_points {
    ($($name:ident)*) => {
        $(#[no_mangle]
        pub extern "C" fn $name() {
            unimplemented!(stringify!($name));
        })*
    };
}

unimplemented_entry_points! {
    // glxgears
    glXQueryDrawable

    // wine
    glXCopyContext
    glXCreateNewContext
    glXCreatePbuffer
    glXCreatePixmap
    glXDestroyPbuffer
    glXDestroyPixmap
    glXGetCurrentContext
    glXGetCurrentDrawable
    glXMakeContextCurrent
}
