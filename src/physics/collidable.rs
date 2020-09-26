use utils::*;

pub struct AxisAlignedBoundingBox {
  pub mins: Vec3F,
  pub maxes: Vec3F,
}

impl AxisAlignedBoundingBox {
  pub fn new(mins: Vec3F, maxes: Vec3F) -> AxisAlignedBoundingBox {
    AxisAlignedBoundingBox {
      mins, maxes
    }
  }
}

pub struct CollisionDetails {
  collision_point: Vec3F,
  tangent: Vec3F
}

impl CollisionDetails {
  pub fn new(collision_point: Vec3F, tangent: Vec3F) -> CollisionDetails {
    CollisionDetails {
      collision_point,
      tangent,
    }
  }
}

pub trait Collidable {

  fn aabb(&self) -> AxisAlignedBoundingBox;

  fn collision_details(&self, other: &dyn Collidable) -> Option<CollisionDetails>;

  fn check_dimension(&self, al: &f32, ar: &f32, bl: &f32, br: &f32) -> bool {
    ar > bl || br > al
  }

  fn aabb_intersects(&self, other: &dyn Collidable) -> bool {
    let my_aabb = self.aabb();
    let ot_aabb = other.aabb();
    self.check_dimension(&my_aabb.mins.x, &my_aabb.maxes.x, &ot_aabb.mins.x, &ot_aabb.maxes.x) &&
    self.check_dimension(&my_aabb.mins.y, &my_aabb.maxes.y, &ot_aabb.mins.y, &ot_aabb.maxes.y) &&
    self.check_dimension(&my_aabb.mins.z, &my_aabb.maxes.z, &ot_aabb.mins.z, &ot_aabb.maxes.z)
  }
}