use std::cell::Cell;
use std::mem;
use std::sync::{Arc, Mutex};

mod api;

struct Context(Mutex<State>);

enum State {
    Inactive(Box<crate::gl::Context>),
    Current { read: Surface, draw: Surface },
}

thread_local!(static CURRENT_CX: Cell<Option<Arc<Context>>> = Cell::new(None));

impl Context {
    fn new() -> Arc<Self> {
        Arc::new(Self(Mutex::new(State::Inactive(Box::new(
            crate::gl::Context::new(),
        )))))
    }

    fn get_current() -> Option<Arc<Context>> {
        CURRENT_CX.with(|current| {
            let glx = current.take();
            current.set(glx.clone());
            glx
        })
    }

    fn remove_current() -> Option<Arc<Context>> {
        CURRENT_CX.with(
            |current| match (current.take(), crate::gl::Context::leave()) {
                (Some(glx), Some(gl)) => {
                    // FIXME(eddyb) flush `gl` and/or surfaces?
                    match mem::replace(&mut *glx.0.lock().unwrap(), State::Inactive(gl)) {
                        State::Inactive(_) => unreachable!("glOOF: inactive current GLX context"),
                        State::Current { .. } => {}
                    }
                    Some(glx)
                }
                (None, None) => None,
                _ => unreachable!("glOOF: mismatch between GL and GLX contexts"),
            },
        )
    }

    fn make_current(self: &Arc<Self>, read: Surface, draw: Surface) {
        Self::remove_current();

        match mem::replace(&mut *self.0.lock().unwrap(), State::Current { read, draw }) {
            State::Inactive(gl) => {
                gl.enter();
            }
            State::Current { .. } => {
                // FIXME(eddyb) maybe `Context::make_current` should return `Result`,
                // and this can be an error reported by API functions?
                panic!("glOOF: GLX context already current on another thread");
            }
        }
        CURRENT_CX.with(|current| {
            let previous = current.replace(Some(self.clone()));
            assert!(previous.is_none());
        });
    }
}

struct Surface {
    // FIXME(eddyb) encapsulate this better.
    drawable: x11::glx::GLXDrawable,
}
