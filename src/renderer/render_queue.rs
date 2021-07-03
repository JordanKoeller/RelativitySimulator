use std::thread::ThreadId;
use std::collections::BinaryHeap;

use utils::{SyncMutRef, getSyncMutRef};


type RenderCommand = i32;

#[derive(Debug, Default)]
pub struct RenderQueue {
  queue: SyncMutRef<BinaryHeap<RenderCommand>>
}

impl RenderQueue {
  pub fn push (&mut self, cmd: RenderCommand) {
    self.queue.lock().expect("Could not unlock Render Command Queue").push(cmd);
  }

  pub fn pop(&mut self) -> Option<RenderCommand> {
    self.queue.lock().expect("Could not unlock Render Command Queue").pop()
  }
  
  pub fn consume(&mut self) -> RenderQueueConsumer<'_> {
    RenderQueueConsumer(self)
  }
}

pub struct RenderQueueConsumer<'a>(&'a mut RenderQueue);

impl <'a> Iterator for RenderQueueConsumer<'a> {
  type Item = RenderCommand;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

impl <'a> Drop for RenderQueueConsumer<'a> {
  fn drop(&mut self) {
    while self.next().is_some() {}
  }
}