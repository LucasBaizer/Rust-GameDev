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

    pub fn from_euler_angles(roll: f32, pitch: f32, yaw: f32) -> Quaternion {
        Quaternion {
            w: f32::cos(roll / 2.0) * f32::cos(pitch / 2.0) * f32::cos(yaw / 2.0) + f32::sin(roll / 2.0) * f32::sin(pitch / 2.0) * f32::sin(yaw / 2.0),
            z: f32::sin(roll / 2.0) * f32::cos(pitch / 2.0) * f32::cos(yaw / 2.0) - f32::cos(roll / 2.0) * f32::sin(pitch / 2.0) * f32::sin(yaw / 2.0),
            x: f32::cos(roll / 2.0) * f32::sin(pitch / 2.0) * f32::cos(yaw / 2.0) + f32::sin(roll / 2.0) * f32::cos(pitch / 2.0) * f32::sin(yaw / 2.0),
            y: f32::cos(roll / 2.0) * f32::cos(pitch / 2.0) * f32::sin(yaw / 2.0) - f32::sin(roll / 2.0) * f32::sin(pitch / 2.0) * f32::cos(yaw / 2.0)
        }
    }

    pub fn from_axis_angle(x: f32, y: f32, z: f32, angle: f32) -> Quaternion {
        Quaternion {
            x: x * f32::sin(angle / 2.0),
            y: y * f32::sin(angle / 2.0),
            z: z * f32::sin(angle / 2.0),
            w: f32::cos(angle / 2.0)
        }
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
        let mut array: [[f32; 4]; 4] = [[0.0; 4]; 4];

        array[0][0] = 1.0 - 2.0 * f32::powi(self.y, 2) - 2.0 * f32::powi(self.z, 2);
        array[0][1] = 2.0 * self.x * self.y - 2.0 * self.z * self.w;
        array[0][2] = 2.0 * self.x * self.z + 2.0 * self.y * self.w;
        array[1][0] = 2.0 * self.x * self.y + 2.0 * self.z * self.w;
        array[1][1] = 1.0 - 2.0 * f32::powi(self.x, 2) - 2.0 * f32::powi(self.z, 2);
        array[2][1] = 2.0 * self.y * self.z + 2.0 * self.x * self.w;
        array[2][0] = 2.0 * self.x * self.z - 2.0 * self.y * self.w;
        array[2][1] = 2.0 * self.y * self.z + 2.0 * self.x * self.w;
        array[2][2] = 1.0 - 2.0 * f32::powi(self.x, 2) - 2.0 * f32::powi(self.y, 2);
        array[3][3] = 1.0;

        array.into()
    }

    pub fn slerp(&self, qb: Quaternion, t: f32) -> Quaternion {
        let cos_half_theta = self.w * qb.w + self.x * qb.x + self.y * qb.y + self.z * qb.z;
        // if qa=qb or qa=-qb then theta = 0 and we can return qa
        if f32::abs(cos_half_theta) >= 1.0 {
            Quaternion {
                w: self.w,
                x: self.x,
                y: self.y,
                z: self.z
            }
        } else {
            let half_theta = f32::acos(cos_half_theta);
            let sin_half_theta = f32::sqrt(1.0 - f32::powi(cos_half_theta, 2));
            // if theta = 180 degrees then result is not fully defined
            // we could rotate around any axis normal to qa or qb
            if f32::abs(sin_half_theta) < 0.001 { // fabs is floating point absolute
                Quaternion {
                    w: (self.w * 0.5 + qb.w * 0.5),
                    x: (self.x * 0.5 + qb.x * 0.5),
                    y: (self.y * 0.5 + qb.y * 0.5),
                    z: (self.z * 0.5 + qb.z * 0.5)
                }
            } else {
                let ratio_a = f32::sin((1.0 - t) * half_theta) / sin_half_theta;
                let ratio_b = f32::sin(t * half_theta) / sin_half_theta; 
                //calculate Quaternion.
                Quaternion {
                    w: (self.w * ratio_a + qb.w * ratio_b),
                    x: (self.x * ratio_a + qb.x * ratio_b),
                    y: (self.y * ratio_a + qb.y * ratio_b),
                    z: (self.z * ratio_a + qb.z * ratio_b)
                }
            }
        }
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, q2: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x * q2.w + self.y * q2.z - self.z * q2.y + self.w * q2.x,
            y: -self.x * q2.z + self.y * q2.w + self.z * q2.x + self.w * q2.y,
            z: self.x * q2.y - self.y * q2.x + self.z * q2.w + self.w * q2.z,
            w: -self.x * q2.x - self.y * q2.y - self.z * q2.z + self.w * q2.w
        }
    }
}

impl ops::MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, q2: Quaternion) {
        let x =  self.x * q2.w + self.y * q2.z - self.z * q2.y + self.w * q2.x;
        let y = -self.x * q2.z + self.y * q2.w + self.z * q2.x + self.w * q2.y;
        let z =  self.x * q2.y - self.y * q2.x + self.z * q2.w + self.w * q2.z;
        let w = -self.x * q2.x - self.y * q2.y - self.z * q2.z + self.w * q2.w;
    
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
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
        assert_rot_eq(q, Quaternion { x: 0.0, y: 1.0, z: 0.0, w: 0.0 });

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