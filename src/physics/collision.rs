use std::cmp::Ordering;
use std::collections::BinaryHeap;

use specs::prelude::*;
use specs::{Component, VecStorage};

use cgmath::prelude::*;

use physics::TransformComponent;
use utils::{Vec3F};


pub type CollisionQueue = BinaryHeap<CollisionSummary>;

pub trait Collision: Component {

  fn sphere_collision(&self, sphere: (&Vec3F, &f32), velocity: &Vec3F) -> Option<CollisionSummary>;

  fn distance_to(&self, pt: &Vec3F) -> f32;

  // Some helper functions for traits
  fn between(&self, left: &f32, right: &f32, center: &f32) -> bool {
    // println!("{} {} {}", left, center, right);
    left <= center && center <= right
  }

  fn approx_between(&self, left: &f32, right: &f32, center: &f32) -> bool {
    // println!("{} {} {}", left, center, right);
    let tolerance = 1e-8f32;
    left - tolerance <= *center && *center <= right + tolerance
  }

  fn get_collision(&self, surface_pt: &Vec3F, query_pt: &Vec3F, velocity: &Vec3F, surface_normal: &Vec3F) -> Option<CollisionSummary> {
    let v_dot_n = velocity.dot(*surface_normal);
    if v_dot_n.abs() < 1e-8f32 {
      None
    } else {
      let t = (surface_pt - query_pt).dot(*surface_normal) / v_dot_n;
      let intersection_pt = velocity * t + query_pt;
      Some(CollisionSummary {
        time: t,
        position: intersection_pt,
        surface_normal: surface_normal.clone()
      })
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionSummary {
  pub time: f32,
  pub position: Vec3F,
  pub surface_normal: Vec3F
}

impl PartialEq for CollisionSummary {
  fn eq(&self, rhs: &Self) -> bool {
    self.time == rhs.time && self.position == rhs.position && self.surface_normal == rhs.surface_normal
  }
}

impl Eq for CollisionSummary {}

impl Ord for CollisionSummary {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.time < other.time {
      Ordering::Less
    } else if self.time > other.time {
      Ordering::Greater
    } else {
      Ordering::Equal
    }
  }
}

impl PartialOrd for CollisionSummary {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct CanCollide {
  pub radius: f32,
}