use cgmath::prelude::*;

use shapes::BlockFace;
use utils::Vec3F;

const FLOAT_TOLERANCE: f32 = 1e-8f32;
const ERR_VEC: Vec3F = Vec3F::new(FLOAT_TOLERANCE, FLOAT_TOLERANCE, FLOAT_TOLERANCE);
/**
 * Calculates if a point intersects a cube.
 *
 * If it does, returns the location of intersection and a BlockFace, representing which side of the cube intersected.
 *
 * I use the "Algebraic method", described in this wikipedia article https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
 */

pub fn line_intersects_block(
    line_start: &Vec3F,
    line_end: &Vec3F,
    neg_corner: &Vec3F,
    pos_corner: &Vec3F,
) -> Option<(Vec3F, BlockFace)> {
    let line_direction = (line_end - line_start).normalize();
    let line_point = line_start.clone();
    let faces = [
        (neg_corner, -Vec3F::unit_x(), BlockFace::Left),
        (neg_corner, -Vec3F::unit_y(), BlockFace::Bottom),
        (neg_corner, -Vec3F::unit_z(), BlockFace::Front),
        (pos_corner, Vec3F::unit_x(), BlockFace::Right),
        (pos_corner, Vec3F::unit_y(), BlockFace::Top),
        (pos_corner, Vec3F::unit_z(), BlockFace::Back),
    ];
    let first_collision: Option<(f32, Vec3F, &BlockFace)> =
        faces.iter().fold(None, |best, (&point_on_face, surface_normal, face)| {
            let line_dot_normal = surface_normal.dot(line_direction);
            let collision_pt = if line_dot_normal.abs() > FLOAT_TOLERANCE {
                let d = (point_on_face - line_point).dot(*surface_normal) / line_dot_normal;
                let intersection_point = line_start + line_direction * d;
                // If a point is on a face of the cube, it is on the entire cube.
                // So I don't need a special bounds check for each face - I can just boundscheck the entire cube.
                if approx_within(&intersection_point, neg_corner, pos_corner)
                    && approx_within(&intersection_point, line_start, &(line_end + ERR_VEC))
                {
                    Some((d, intersection_point, face))
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(best_collider) = best {
                if let Some(new_collider) = collision_pt {
                    if new_collider.0 < best_collider.0 {
                        Some(new_collider)
                    } else {
                        Some(best_collider)
                    }
                } else {
                    Some(best_collider)
                }
            } else {
                collision_pt
            }
        });
    if let Some(collision) = first_collision {
        Some((collision.1, collision.2.clone()))
    } else {
        None
    }
}

fn approx_within(query: &Vec3F, lows: &Vec3F, highs: &Vec3F) -> bool {
    let t_vec = (query - lows).div_element_wise(highs - lows);
    approx_between(t_vec.x, 0f32, 1f32) && approx_between(t_vec.y, 0f32, 1f32) && approx_between(t_vec.z, 0f32, 1f32)
}

fn approx_between(x: f32, a: f32, b: f32) -> bool {
    x >= a - FLOAT_TOLERANCE && x <= b + FLOAT_TOLERANCE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_within_finds_easy_within() {
        let query = Vec3F::new(1.2f32, 3.6f32, 4.8f32);
        let low = Vec3F::new(1f32, 3f32, 4f32);
        let high = Vec3F::new(2f32, 4f32, 5f32);
        assert_eq!(true, approx_within(&query, &low, &high));
        assert_eq!(false, approx_within(&high, &low, &query));
        assert_eq!(false, approx_within(&low, &query, &high));
    }

    #[test]
    fn test_approx_within_checks_all_coordinates() {
        let query = Vec3F::new(2.5f32, 3.6f32, 4.8f32);
        let low = Vec3F::new(1f32, 3f32, 4f32);
        let high = Vec3F::new(2f32, 4f32, 5f32);
        assert_eq!(false, approx_within(&query, &low, &high));
    }

    #[test]
    fn test_approx_within_accepts_when_query_on_limit() {
        let query = Vec3F::new(1.8f32, 3.6f32, 4.8f32);
        let high = Vec3F::new(2f32, 4f32, 5f32);
        assert_eq!(true, approx_within(&query, &query, &high));
    }

    #[test]
    fn test_approx_within_accepts_when_within_floating_point_range() {
        let low = Vec3F::new(1.8f32, 3.6f32, 4.8f32);
        let query = Vec3F::new(1.8f32 - 1e-10f32, 3.6f32, 4.8f32);
        let high = Vec3F::new(2f32, 4f32, 5f32);
        assert_eq!(true, approx_within(&query, &low, &high));
    }

    #[test]
    fn test_line_intersects_bisects_block() {
        let line_start = Vec3F::new(-1f32, 0f32, 0f32);
        let line_end = Vec3F::new(1f32, 0f32, 0f32);
        let lows = Vec3F::new(-0.5f32, -0.5f32, -0.5f32);
        let highs = Vec3F::new(0.5f32, 0.5f32, 0.5f32);
        if let Some((pt, face)) = line_intersects_block(&line_start, &line_end, &lows, &highs) {
            assert_eq!(face, BlockFace::Left);
            assert_eq!(pt, Vec3F::new(-0.5f32, 0f32, 0f32));
        } else {
            assert_eq!(false, true);
        }
    }
}
