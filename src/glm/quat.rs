use std::fmt::{Display, Formatter, Error};

use super::{NEAR_ZERO, Mat4, Vec3, Mat3};

// note: layout is [ w, x, y, z]
//              or [ w, i, j, k]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quat (pub(crate) [f32; 4]);


impl Quat {

    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Quat { Quat ( [w, x, y, z] ) }
    pub fn identity() -> Quat { Quat ( [1.0, 0.0, 0.0, 0.0] ) }
    pub fn axis_of_rotation(axis: Vec3, angle: f32) -> Quat {
        Quat([
            (angle/2.0).cos(),
            (angle/2.0).sin() * axis.x(),
            (angle/2.0).sin() * axis.y(),
            (angle/2.0).sin() * axis.z(),
        ])
    }
    pub fn w(&self) -> f32 { self.0[0] }
    pub fn x(&self) -> f32 { self.0[1] }
    pub fn y(&self) -> f32 { self.0[2] }
    pub fn z(&self) -> f32 { self.0[3] }
    pub fn xyz(&self) -> Vec3 { Vec3([self.0[1], self.0[2], self.0[3]]) }
    pub fn equals(&self, other: &Quat) -> bool {
        (self.x() - other.x()).abs() < NEAR_ZERO
        && (self.y() - other.y()).abs() < NEAR_ZERO
        && (self.z() - other.z()).abs() < NEAR_ZERO
        && (self.w() - other.w()).abs() < NEAR_ZERO
    }
    pub fn mag(&self) -> f32 { ( self[0].powi(2) + self[1].powi(2) + self[2].powi(2) + self[3].powi(2) ).sqrt() }
    pub fn length(&self) -> f32 { self.mag() }
    pub fn normalize(&self) -> Quat {
        let mag = self.mag();
        if mag != 0.0 {
            *self / mag
        }
        else {
            *self
        }
    }
    pub fn conjugate(&self) -> Quat {
        Quat ( [
             self.w(), // w
            -self.x(), // x
            -self.y(), // y
            -self.z(), // z
        ] )
    }
    pub fn inverse(&self) -> Quat {
        let inv_norm = 1.0 / (
            self.w() * self.w() +
            self.x() * self.x() +
            self.y() * self.y() +
            self.z() * self.z() );
        self.conjugate() * inv_norm
    }
    pub fn mat3(&self) -> Mat3 {
        // calculate coefficients
        let x2 = self.x() + self.x();
        let y2 = self.y() + self.y();
        let z2 = self.z() + self.z();
        let xx = self.x() * x2;
        let xy = self.x() * y2;
        let xz = self.x() * z2;
        let yy = self.y() * y2;
        let yz = self.y() * z2;
        let zz = self.z() * z2;
        let wx = self.w() * x2;
        let wy = self.w() * y2;
        let wz = self.w() * z2;
        Mat3([
            1.0 - yy - zz,       xy + wz,       xz - wy,
                  xy - wz, 1.0 - xx - zz,       yz + wx,
                  xz + wy,       yz - wx, 1.0 - xx - yy,
        ])
    }
    pub fn mat4(&self) -> Mat4 {
        // calculate coefficients
        let x2 = self.x() + self.x();
        let y2 = self.y() + self.y();
        let z2 = self.z() + self.z();
        let xx = self.x() * x2;
        let xy = self.x() * y2;
        let xz = self.x() * z2;
        let yy = self.y() * y2;
        let yz = self.y() * z2;
        let zz = self.z() * z2;
        let wx = self.w() * x2;
        let wy = self.w() * y2;
        let wz = self.w() * z2;
        Mat4([
            1.0 - yy - zz,       xy + wz,       xz - wy, 0.0,
                  xy - wz, 1.0 - xx - zz,       yz + wx, 0.0,
                  xz + wy,       yz - wx, 1.0 - xx - yy, 0.0,
                      0.0,           0.0,           0.0, 1.0,
        ])

//        m[0][0] = 1.0 - (yy + zz);
//        m[1][0] = xy - wz;
//        m[2][0] = xz + wy;
//        m[3][0] = 0.0;
//
//        m[0][1] = xy + wz;
//        m[1][1] = 1.0 - (xx + zz);
//        m[2][1] = yz - wx;
//        m[3][1] = 0.0;
//
//
//        m[0][2] = xz - wy;
//        m[1][2] = yz + wx;
//        m[2][2] = 1.0 - (xx + yy);
//        m[3][2] = 0.0;
//
//
//        m[0][3] = 0;
//        m[1][3] = 0;
//        m[2][3] = 0;
//        m[3][3] = 1;

    }

    pub fn rotate(&self, rotation: Quat) -> Quat {
        *self * rotation
    }

    pub fn slerp(&self, mut to: Quat, t: f32) -> Quat {
        let scale0: f32;
        let scale1: f32;

        // calc dot
        let mut dot = self.x() * to.x() + self.y() * to.y() + self.z() * to.z() + self.w() * to.w();

        // adjust signs (if necessary)
        if dot < 0.0 {
            dot = -dot;

            to = to * -1.0;
        }

        // calculate coefficients
        if 1.0 - dot > NEAR_ZERO {
            // standard case (slerp)
            let omega = dot.acos();
            let sinom = omega.sin();
            scale0 = ((1.0 - t) * omega).sin() / sinom;
            scale1 = (t * omega).sin() / sinom;
        } else {
            // "self" and "to" quaternions are very close
            //  ... so we can do a linear interpolation
            scale0 = 1.0 - t;
            scale1 = t;
        }

        // calculate to values
        Quat::new(
            scale0 * self.w() + scale1 * to[0],
            scale0 * self.x() + scale1 * to[1],
            scale0 * self.y() + scale1 * to[2],
            scale0 * self.z() + scale1 * to[3],
        )
    }

    pub fn lerp(&self, mut to: Quat, t: f32) -> Quat {

        // calc cosine
        let cosom = self.x() * to.x() + self.y() * to.y() + self.z() * to.z() + self.w() * to.w();

        // adjust signs (if necessary)
        if cosom < 0.0 {
            to = to * -1.0;
        }

        // linear interpolation
        let scale0 = 1.0 - t;
        let scale1 = t;

        // calculate to values
        Quat::new(
            scale0 * self.w() + scale1 * to[0],
            scale0 * self.x() + scale1 * to[1],
            scale0 * self.y() + scale1 * to[2],
            scale0 * self.z() + scale1 * to[3],
        )
    }

    // create a rotation from one axis to another. These must be unit vectors
    pub fn from_two_axis(from: Vec3, to: Vec3) -> Quat {
        let mut tx: f32;
        let mut ty: f32;
        let mut tz: f32;
        let dist: f32;

        // get dot product of two vectors
        let cost = from.x() * to.x() + from.y() * to.y() + from.z() * to.z();

        // check if parallel
        if cost > 0.99999 {
            return Quat::new(1.0, 0.0, 0.0, 0.0)
        }
        else if cost < -0.99999 {     // check if opposite
            // check if we can use cross product of from vector with [1, 0, 0]
            tx = 0.0;
            ty = from.x();
            tz = -from.y();

            let len = (ty*ty + tz*tz).sqrt();

            if len < NEAR_ZERO {
                // nope! we need cross product of from vector with [0, 1, 0]
                tx = -from.z();
                ty = 0.0;
                tz = from.x();
            }

            // normalize
            dist = 1.0 / (tx*tx + ty*ty + tz*tz).sqrt();

            tx *= dist;
            ty *= dist;
            tz *= dist;

            return Quat::new(0.0, tx, ty, tz)
        }

        // ... else we can just cross two vectors
        tx = from.y() * to.z() - from.z() * to.y();
        ty = from.z() * to.x() - from.x() * to.z();
        tz = from.x() * to.y() - from.y() * to.x();

        dist = 1.0 / (tx*tx + ty*ty + tz*tz).sqrt();

        tx *= dist;
        ty *= dist;
        tz *= dist;

        // we have to use half-angle formulae (sin^2 t = ( 1 - cos (2t) ) /2)
        let ss = (0.5 * (1.0 - cost)).sqrt();

        tx *= ss;
        ty *= ss;
        tz *= ss;

        // scale the axis to get the normalized quaternion
        // cos^2 t = ( 1 + cos (2t) ) / 2
        // w part is cosine of half the rotation angle
        Quat::new((0.5 * (1.0 + cost)).sqrt(), tx, ty, tz)
    }

    pub fn from_euler_ypr(yaw: f32, pitch: f32, roll: f32) -> Quat {
        let cr = (roll/2.0).cos();
        let cp = (pitch/2.0).cos();
        let cy = (yaw/2.0).cos();
        let sr = (roll/2.0).sin();
        let sp = (pitch/2.0).sin();
        let sy = (yaw/2.0).sin();
        let cpcy = cp * cy;
        let spsy = sp * sy;
        let spcy = sp * cy;
        let cpsy = cp * sy;
        Quat ([
            sr * cpcy - cr * spsy,
            cr * spcy + sr * cpsy,
            cr * cpsy - sr * spcy,
            cr * cpcy + sr * spsy,
        ])
    }

    /// vec must be normalized
    pub fn from_angle_axis(angle: f32, vec: Vec3) -> Quat {
        // scalar
        let scale = (angle / 2.0).sin();

        Quat ([
            (angle / 2.0).cos(),
            vec[0] * scale,
            vec[1] * scale,
            vec[2] * scale,
        ])
    }

    pub fn to_euler_ypr(&self) -> Vec3 {
        let t0 = 2.0 * (self.w() * self.x() + self.y() * self.z());
        let t1 = 1.0 - 2.0 * (self.x() * self.x() + self.y() * self.y());
        let roll = t0.atan2(t1);
        let mut t2 = 2.0 * (self.w() * self.y() - self.z() * self.x());
        t2 = if t2 > 1.0 { 1.0 } else { t2 };
        t2 = if t2 < -1.0 { -1.0 } else { t2 };
        let pitch = t2.asin();
        let t3 = 2.0 * (self.w() * self.z() + self.x() * self.y());
        let t4 = 1.0 - 2.0 * (self.y() * self.y() + self.z() * self.z());
        let yaw = t3.atan2(t4);
        Vec3([yaw, pitch, roll])
    }

    pub fn to_angle_axis(&self) -> (f32, Vec3) {

        let angle = 2.0 * self.w().acos();
        let scale = (angle / 2.0).sin();

        // if it's not pretty much zero
        if scale > NEAR_ZERO
        {
            ( angle, Vec3::new(self.x() / scale, self.y() / scale, self.z() / scale) )
        }
        else {
            ( 0.0, Vec3::new(0.0, 0.0, 1.0) )
        }
    }

    pub fn scale_angle(&self, s: f32) -> Quat {
        let (angle, vec) = self.to_angle_axis();
        Self::from_angle_axis(angle * s, vec)
    }
}

//------------------------------------------------------------------------------------------------//
// OPERATORS                                                                                      //
//------------------------------------------------------------------------------------------------//
impl std::ops::Mul<Quat> for Quat {
    type Output = Quat;

    fn mul(self, rhs: Quat) -> Quat {
        Quat ( [
            self.w() * rhs.w() - self.x() * rhs.x() - self.y() * rhs.y() - self.z() * rhs.z(), // w
            self.x() * rhs.w() + self.w() * rhs.x() + self.y() * rhs.z() - self.z() * rhs.y(), // x
            self.y() * rhs.w() + self.w() * rhs.y() + self.z() * rhs.x() - self.x() * rhs.z(), // y
            self.z() * rhs.w() + self.w() * rhs.z() + self.x() * rhs.y() - self.y() * rhs.x(), // z
        ] ).normalize()
    }
}

impl std::ops::MulAssign<Quat> for Quat {

    fn mul_assign(&mut self, rhs: Quat) {
        *self = *self * rhs;
    }
}

// Quat * Vec3 = Vec3 rotated by Quat
impl std::ops::Mul<Vec3> for Quat {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let q_xyz: Vec3 = self.xyz();
        let t: Vec3 = q_xyz.cross(&rhs) * 2.0;
        let u: Vec3 = q_xyz.cross(&t);
        rhs + (t * self.w()) + u
    }
}

// // What is this??
// impl std::ops::Mul<Vec3> for Quat {
//     type Output = Quat;
//
//     fn mul(self, rhs: Vec3) -> Quat {
//         Quat ( [
//            -self.x() * rhs.x() - self.y() * rhs.y() - self.z() * rhs.z(), // w
//             self.w() * rhs.x() + self.y() * rhs.z() - self.z() * rhs.y(), // x
//             self.w() * rhs.y() + self.z() * rhs.x() - self.x() * rhs.z(), // y
//             self.w() * rhs.z() + self.x() * rhs.y() - self.y() * rhs.x(), // z
//         ] )
//     }
// }

impl std::ops::Mul<f32> for Quat {
    type Output = Quat;

    fn mul(self, rhs: f32) -> Self::Output {
        // may not be a unit quaternion after this
        Quat::new(self.w() * rhs, self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl std::ops::MulAssign<f32> for Quat {
    fn mul_assign(&mut self, rhs: f32) {
        // may not be a unit quaternion after this
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
        self[3] *= rhs;
    }
}

impl std::ops::Div<f32> for Quat {
    type Output = Quat;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 { panic!("Cannot divide by zero. (Quat / 0.0)"); }
        Quat ( [
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
            self[3] / rhs,
        ] )
    }
}

impl std::ops::DivAssign<f32> for Quat {
    fn div_assign(&mut self, rhs: f32) {
        // may not be a unit quaternion after this
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
        self[3] /= rhs;
    }
}

impl std::ops::Index<usize> for Quat {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Quat {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.0[index]
    }
}

//------------------------------------------------------------------------------------------------//
// FROM                                                                                           //
//------------------------------------------------------------------------------------------------//
impl From<Mat4> for Quat {
    fn from(m: Mat4) -> Self {
        let tr: f32 = m[0] + m[5] + m[10];

        // check the diagonal
        if tr > 0.0 {
            let s: f32 = (tr + 1.0).sqrt();
            let si: f32 = 0.5 / s;
            Quat([
                s / 2.0,
                (m[6] - m[9]) * si,
                (m[8] - m[2]) * si,
                (m[1] - m[4]) * si,
            ])
        }
        // diagonal is negative
        else {
            let mut q: [f32; 4] = [0.0; 4];
            let mut i: usize;
            let j: usize;
            let k: usize;
            let nxt: [usize; 3] = [1, 2, 0];

            i = 0;
            if m[5] > m[0] { i = 1; }
            if m[10] > m[(i,i)] { i = 2; }
            j = nxt[i];
            k = nxt[j];
            let mut s: f32 = ((m[(i,i)] - (m[(j,j)] + m[(k,k)])) + 1.0).sqrt();
            q[i] = s * 0.5;
            if s != 0.0 { s = 0.5 / s; }
            q[3] = (m[(j,k)] - m[(k,j)]) * s;
            q[j] = (m[(i,j)] + m[(j,i)]) * s;
            q[k] = (m[(i,k)] + m[(k,i)]) * s;
            Quat([ q[3], q[0], q[1], q[2] ])
        }
    }
}

impl From<Vec3> for Quat {
    fn from(f: Vec3) -> Self {
        Quat::from_euler_ypr(f.x(), f.y(), f.z())
    }
}

//------------------------------------------------------------------------------------------------//
// OTHER                                                                                          //
//------------------------------------------------------------------------------------------------//
impl Default for Quat {
    fn default() -> Self {
        Quat([ 1.0, 0.0, 0.0, 0.0 ])
    }
}

impl Display for Quat {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({} + {}i + {}j + {}k)", self.w(), self.x(), self.y(), self.z())
    }
}