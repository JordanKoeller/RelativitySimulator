use std::cmp::{Ord, Ordering};


use specs::Entity;
use std::thread::ThreadId;
use std::collections::BinaryHeap;

use utils::{SyncMutRef, getSyncMutRef, Mat4F};

use ecs::{DrawableId, Material};

#[derive(Eq, PartialEq, PartialOrd, Debug)]
pub struct DrawCall {
  pub drawable: DrawableId,
  pub entity: Entity,
}

impl Ord for DrawCall {
  fn cmp(&self, other: &Self) -> Ordering {
    let shader_cmp = self.drawable.1.cmp(&other.drawable.1);
    match shader_cmp {
      Ordering::Equal => {
        self.drawable.0.cmp(&other.drawable.0)
      }
      _ => shader_cmp
    }
  }
}


#[derive(Debug, Default)]
pub struct RenderQueue {
  queue: SyncMutRef<BinaryHeap<DrawCall>>
}

impl RenderQueue {
  pub fn push(&mut self, cmd: DrawCall) {
    self.queue.lock().expect("Could not unlock Render Command Queue").push(cmd);
  }

  pub fn pop(&mut self) -> Option<DrawCall> {
    self.queue.lock().expect("Could not unlock Render Command Queue").pop()
  }
  
  pub fn consume(&mut self) -> RenderQueueConsumer<'_> {
    RenderQueueConsumer(self)
  }
}

pub struct RenderQueueConsumer<'a>(&'a mut RenderQueue);

impl <'a> Iterator for RenderQueueConsumer<'a> {
  type Item = DrawCall;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

impl <'a> Drop for RenderQueueConsumer<'a> {
  fn drop(&mut self) {
    while self.next().is_some() {}
  }
}