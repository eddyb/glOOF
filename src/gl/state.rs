use crate::gl::api_1_0::{Command, Enum};
use glam::{DMat4, DVec3, DVec4, Mat4, Vec3};
use std::f32::consts::PI;

#[derive(Debug, Default)]
pub struct State {
    // Matrices.
    pub modelview: MatrixStack,
    pub projection: MatrixStack,
    matrix_mode: MatrixMode,
}

#[derive(Debug)]
enum MatrixMode {
    ModelView,
    Projection,
}

impl Default for MatrixMode {
    fn default() -> Self {
        MatrixMode::ModelView
    }
}

#[derive(Debug, Default)]
pub struct MatrixStack {
    pub mat: Mat4,
    stack: Vec<Mat4>,
}

impl MatrixStack {
    fn push(&mut self) {
        self.stack.push(self.mat);
    }
    fn pop(&mut self) {
        // FIXME(eddyb) report error
        self.mat = self.stack.pop().unwrap();
    }
}

impl State {
    fn matrix_stack(&mut self) -> &mut MatrixStack {
        match self.matrix_mode {
            MatrixMode::ModelView => &mut self.modelview,
            MatrixMode::Projection => &mut self.projection,
        }
    }

    fn matrix(&mut self) -> &mut Mat4 {
        &mut self.matrix_stack().mat
    }

    fn matrix_mul(&mut self, other: Mat4) {
        let mat = self.matrix();
        *mat = mat.mul_mat4(&other);
    }

    fn matrix_mul_double(&mut self, other: DMat4) {
        let mat = self.matrix();
        *mat = mat.as_f64().mul_mat4(&other).as_f32();
    }

    pub fn apply(&mut self, cmd: Command, unhandled: &mut impl FnMut(&mut Self, Command)) {
        use {Command::*, Enum::*};
        match cmd {
            glCallList(list) => {
                for cmd in list.cmds.iter().cloned() {
                    self.apply(cmd, unhandled);
                }
            }
            glMatrixMode(mode) => {
                self.matrix_mode = match mode {
                    MODELVIEW => MatrixMode::ModelView,
                    PROJECTION => MatrixMode::Projection,
                    // FIXME(eddyb) report error
                    _ => unimplemented!("glMatrixMode({:?})", mode),
                };
            }
            glPushMatrix => self.matrix_stack().push(),
            glPopMatrix => self.matrix_stack().pop(),
            glLoadIdentity => *self.matrix() = Mat4::identity(),
            glRotatef(angle, x, y, z) => self.matrix_mul(Mat4::from_axis_angle(
                Vec3::new(x, y, z),
                angle / 180.0 * PI,
            )),
            glTranslatef(x, y, z) => self.matrix_mul(Mat4::from_translation(Vec3::new(x, y, z))),
            glTranslated(x, y, z) => {
                self.matrix_mul_double(DMat4::from_translation(DVec3::new(x, y, z)))
            }
            glFrustum(l, r, b, t, n, f) => self.matrix_mul_double(DMat4::from_cols(
                DVec4::new(2.0 * n / (r - l), 0.0, 0.0, 0.0),
                DVec4::new(0.0, 2.0 * n / (t - b), 0.0, 0.0),
                DVec4::new(
                    (r + l) / (r - l),
                    (t + b) / (t - b),
                    -(f + n) / (f - n),
                    -1.0,
                ),
                DVec4::new(0.0, 0.0, -(2.0 * f * n) / (f - n), 0.0),
            )),
            _ => unhandled(self, cmd),
        }
    }
}
