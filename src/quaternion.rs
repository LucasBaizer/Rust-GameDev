use std::ops;
use nalgebra;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn identity() -> Quaternion {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        }
    }

    pub fn from_euler_angles(pitch: f32, roll: f32, yaw: f32) -> Quaternion {
        let cy = f32::cos(yaw * 0.5);
        let sy = f32::sin(yaw * 0.5);
        let cr = f32::cos(roll * 0.5);
        let sr = f32::sin(roll * 0.5);
        let cp = f32::cos(pitch * 0.5);
        let sp = f32::sin(pitch * 0.5);

        println!("cy: {:?}", cy);
        println!("sy: {:?}", sy);
        println!("cr: {:?}", cr);
        println!("sr: {:?}", sr);
        println!("cp: {:?}", cp);
        println!("sp: {:?}", sp);

        Quaternion {
            w: cy * cr * cp + sy * sr * sp,
            x: cy * sr * cp - sy * cr * sp,
            y: cy * cr * sp + sy * sr * cp,
            z: sy * cr * cp - cy * sr * sp
        }
    }

    pub fn from_axis_angle(x: f32, y: f32, z: f32, angle: f32) -> Quaternion {
        unimplemented!()
    }
    
    
    pub fn len(&self) -> f32 {
        f32::sqrt(f32::powi(self.w, 2) + f32::powi(self.x, 2) + f32::powi(self.y, 2) + f32::powi(self.z, 2))
    }
    
    pub fn normalize(&mut self) {
        let len = self.len();
        self.w = self.w / len;
        self.x = self.x / len;
        self.y = self.y / len;
        self.z = self.z / len;
    }

    pub fn into_matrix(self) -> nalgebra::core::Matrix4<f32> {
        unimplemented!()
    }

    pub fn slerp(&self, dst: Quaternion, t: f32) -> Quaternion {
        unimplemented!()
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z,
            z: self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x
        }
    }
}

impl ops::MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, rhs: Quaternion) {
        self.w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
        self.x = self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y;
        self.y = self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z;
        self.z = self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::f32::consts::PI;

    const EPSILON: f32 = 1.0e-6;

    // Check if two floats are approximately equal.
    fn assert_feq(a: f32, b: f32) {
        if (a-b).abs() >= EPSILON {
            panic!("Not equal: {} {}", a, b);
        }
    }

    // Check if two Quaternions are approximately equal.
    fn assert_quat_eq(a: Quaternion, b: Quaternion) {
        if (a.x-b.x).abs() >= EPSILON || (a.y-b.y).abs() >= EPSILON || (a.z- b.z).abs() >= EPSILON || (a.w-b.w).abs() >= EPSILON {
            panic!("Not equal: {:?} {:?}", a, b);
        }
    }

    // Check if two Quaternion represent approximately the same rotation.
    // Both q and -q represent the same rotation.
    fn assert_rot_eq(a: Quaternion, b: Quaternion) {
        if (a.x-b.x).abs() >= EPSILON || (a.y-b.y).abs() >= EPSILON || (a.z- b.z).abs() >= EPSILON || (a.w-b.w).abs() >= EPSILON {
            if (a.x+b.x).abs() >= EPSILON || (a.y+b.y).abs() >= EPSILON || (a.z+b.z).abs() >= EPSILON || (a.w+b.w).abs() >= EPSILON {
                panic!("Not equal: {:?} {:?}", a, b);
            }
        }
    }

    #[test]
    fn test_identity() {
        assert_eq!(Quaternion::identity(), Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 });
    }

    #[test]
    fn test_len() {
        assert_eq!(Quaternion::identity().len(), 1.0);
        assert_eq!(Quaternion { x: 2.0, y: 2.0, z: 2.0, w: 2.0 }.len(), 4.0);

        // Rotations should be unit quaternions.
        let q = Quaternion::from_euler_angles(0.1, 0.4, 123.0);
        if (q.len() - 1.0).abs() >= EPSILON {
            panic!("Incorrect length");
        }
    }

    #[test]
    fn test_normalize() {
        let mut q = Quaternion { x: 2.0, y: -2.0, z: 2.0, w: -2.0 };
        q.normalize();
        assert_feq(q.len(), 1.0);
        assert_quat_eq(q, Quaternion { x: 0.5, y: -0.5, z: 0.5, w: -0.5 });
    }

    #[test]
    fn test_euler_angles() {
        let q = Quaternion::from_euler_angles(0.0, 0.0, PI);
        assert_rot_eq(q, Quaternion { x: 0.0, y: 0.0, z: 1.0, w: 0.0 });

        let q = Quaternion::from_euler_angles(0.0, PI * 2.0, 0.0);
        assert_rot_eq(q, Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 });

        let p = Quaternion::from_euler_angles(PI / 2.0, PI / 2.0, PI / 2.0);
        let q =  Quaternion::from_euler_angles(0.0, PI / 2.0, 0.0);
        assert_rot_eq(p, q);
    }

    #[test]
    fn test_axis_angle() {
        let q = Quaternion::from_axis_angle(1.0, 0.0, 0.0, PI * 3.0);
        assert_rot_eq(q, Quaternion { x: 1.0, y: 0.0, z: 0.0, w: 0.0 });

        let sqrt2_2 = 0.707107;
        let q = Quaternion::from_axis_angle(0.0, sqrt2_2, -sqrt2_2, PI / 2.0);
        assert_rot_eq(q, Quaternion { x: 0.0, y: 0.5, z: -0.5, w: sqrt2_2 });

        // Rotating 360 around any axis should be equivalent.
        let p = Quaternion::from_axis_angle(1.0, 0.0, 0.0, PI * 2.0);
        let q = Quaternion::from_axis_angle(sqrt2_2, sqrt2_2, 0.0, -PI * 4.0);
        assert_rot_eq(p, q);
    }

    #[test]
    fn test_mul() {
        let id = Quaternion::identity();
        let zero = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        assert_quat_eq(id*zero, zero);
        assert_quat_eq(id*id, id);

        let p = Quaternion { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
        let q = Quaternion { x: -1.0, y: 0.0, z: 0.0, w: 0.0 };
        assert_quat_eq(q*id, q);
        assert_quat_eq(p*q, id);

        let mut q = Quaternion::identity();
        let p = Quaternion::from_axis_angle(1.0, 0.0, 0.0, PI);
        q *= p;
        assert_rot_eq(q, p);
        q *= p;
        assert_rot_eq(q, id);

        let q = Quaternion::from_axis_angle(0.0, -1.0, 0.0, 0.2);
        let p = Quaternion::from_euler_angles(0.5, 0.1, 0.9);
        assert_rot_eq(p*q, Quaternion { x: 0.17043722, y: 0.3202073, z: 0.185343, w: 0.9132724 });
    }

    #[test]
    fn test_into_matrix() {
        fn assert_matrix_eq(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) {
            for i in 0..4 {
                for j in 0..4 {
                    assert_feq(a[i][j], b[i][j]);
                }
            }
        }

        let q = Quaternion::identity();
        let m: [[f32; 4]; 4] = q.into_matrix().into();
        assert_matrix_eq(m, [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]);

        // Upside down rotation
        let q = Quaternion::from_axis_angle(0.0, 1.0, 0.0, PI/2.0);
        let m: [[f32; 4]; 4] = q.into_matrix().into();
        assert_matrix_eq(m, [[0.0, 0.0, -1.0, 0.0], [0.0, 1.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0]]);
    }

    #[test]
    fn test_slerp() {
        let p = Quaternion::identity();
        let q = Quaternion::from_euler_angles(0.0, 0.0, 5.0 * PI);
        let r = Quaternion::from_axis_angle(0.0, 1.0, 0.0, PI/2.0);
        assert_rot_eq(p.slerp(q, 0.5), r);

        let p = Quaternion::from_euler_angles(0.2, 0.4, 3.1 * PI);
        let p = Quaternion::from_euler_angles(0.9, 0.2, -120.0);
        assert_rot_eq(p.slerp(q, 0.9), Quaternion { x: 0.0060857176, y: 0.9921862, z: -0.05777763, w: -0.11041405 });
    }
}