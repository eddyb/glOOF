use indexmap::IndexMap;
use std::cell::Cell;
use std::rc::Rc;

pub mod api_1_0;
mod debug;
pub mod state;

#[derive(Debug, Default)]
pub struct DisplayList {
    pub cmds: Vec<api_1_0::Command>,
}

#[derive(Debug)]
pub struct Context {
    pub pending_cmds: Vec<api_1_0::Command>,

    // Display lists.
    pub first_unused_list: u32,
    pub lists: IndexMap<u32, Rc<DisplayList>>,
    pub compile_list: Option<(u32, DisplayList)>,
    pub execute_immediately: bool,

    pub state: state::State,
}

enum TlsState {
    Empty,
    Present(Box<Context>),
    InUse { blame: &'static str },
}

impl TlsState {
    fn swap_in(self) -> Self {
        thread_local!(static CURRENT_CX: Cell<TlsState> = Cell::new(TlsState::Empty));
        CURRENT_CX.with(|current| current.replace(self))
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            pending_cmds: vec![],

            first_unused_list: 1,
            lists: IndexMap::new(),
            compile_list: None,
            execute_immediately: true,

            state: state::State::default(),
        }
    }

    pub fn enter(self: Box<Self>) {
        match TlsState::Present(self).swap_in() {
            TlsState::Empty => {}
            TlsState::Present(_) => {
                unreachable!("glOOF: new GL context entered without leaving the current one first");
            }
            TlsState::InUse { blame } => unreachable!(
                "glOOF: new GL context entered while current one is in use by {}",
                blame
            ),
        }
    }

    pub fn leave() -> Option<Box<Self>> {
        match TlsState::Empty.swap_in() {
            TlsState::Empty => None,
            TlsState::Present(gl) => Some(gl),
            TlsState::InUse { blame } => {
                unreachable!("glOOF: GL context left while in use by {}", blame);
            }
        }
    }

    pub fn with<R>(blame: &'static str, f: impl FnOnce(&mut Self) -> R) -> R {
        struct Guard {
            blame: &'static str,
            gl: Option<Box<Context>>,
        }

        impl Drop for Guard {
            fn drop(&mut self) {
                match TlsState::Present(self.gl.take().unwrap()).swap_in() {
                    TlsState::Empty => unreachable!(
                        "glOOF: GL context went missing while in use by {}",
                        self.blame
                    ),
                    TlsState::Present(_) => unreachable!(
                        "glOOF: GL context was replaced while in use by {}",
                        self.blame
                    ),
                    TlsState::InUse { .. } => {}
                }
            }
        }

        match (TlsState::InUse { blame }).swap_in() {
            TlsState::Empty => {
                // FIXME(eddyb) maybe `Context::with` should return `Result`,
                // and this can be an error reported by API functions?
                panic!(
                    "glOOF: no GL context currently active (needed by {})",
                    blame
                );
            }
            TlsState::Present(gl) => {
                let mut guard = Guard {
                    blame,
                    gl: Some(gl),
                };
                f(guard.gl.as_mut().unwrap())
            }
            TlsState::InUse { blame: outer } => unreachable!(
                "glOOF: reentrance detected in GL context usage: {} called {}",
                outer, blame
            ),
        }
    }
}
