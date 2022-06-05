use specs::{Component, Join, Read, ReadStorage, System, VecStorage, WriteStorage};

use crate::physics::{Collision, CollisionSummary, TransformComponent};
use crate::utils::{swizzle_down, swizzle_up, Vec3F};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct AxisAlignedCubeCollision {
    center: Vec3F,
    dims: Vec3F,
}

impl AxisAlignedCubeCollision {
    pub fn from_transform(transform: &TransformComponent) -> Self {
        let c1 = Vec3F::new(-0.5f64, -0.5f64, -0.5f64);
        let c2 = Vec3F::new(0.5f64, 0.5f64, 0.5f64);
        let matrix = transform.matrix();
        let p1 = matrix * swizzle_up(&c1);
        let p2 = matrix * swizzle_up(&c2);
        let center = (p1 + p2) / 2f64;
        let dims = p2 - p1;
        Self {
            center: swizzle_down(&center),
            dims: swizzle_down(&dims),
        }
    }

    fn within_box(&self, pt: &Vec3F, bl: &Vec3F, tr: &Vec3F) -> bool {
        self.approx_between(&bl.x, &tr.x, &pt.x)
            && self.approx_between(&bl.y, &tr.y, &pt.y)
            && self.approx_between(&bl.z, &tr.z, &pt.z)
    }
}

impl Collision for AxisAlignedCubeCollision {
    fn distance_to(&self, _pt: &Vec3F) -> f64 {
        0f64
    }

    fn sphere_collision(&self, sphere: (&Vec3F, &f64), velocity: &Vec3F) -> Option<CollisionSummary> {
        let new_dims = self.dims + Vec3F::new(1f64, 1f64, 1f64) * *sphere.1 * 2f64;
        let c = sphere.0;

        let lows = self.center - new_dims / 2f64;
        let highs = self.center + new_dims / 2f64;

        let checks = [
            (&lows, &-Vec3F::unit_x()),
            (&lows, &-Vec3F::unit_y()),
            (&lows, &-Vec3F::unit_z()),
            (&highs, &Vec3F::unit_x()),
            (&highs, &Vec3F::unit_y()),
            (&highs, &Vec3F::unit_z()),
        ];

        checks.iter().fold(None, |acc, elem| {
            if let Some(summary) = self.get_collision(elem.0, &c, velocity, elem.1) {
                if summary.time >= 0f64 && self.within_box(&summary.position, &lows, &highs) {
                    if let Some(prev_best) = acc {
                        if summary.time < prev_best.time {
                            Some(summary)
                        } else {
                            Some(prev_best)
                        }
                    } else {
                        Some(summary)
                    }
                } else {
                    acc
                }
            } else {
                acc
            }
        })
    }
}
