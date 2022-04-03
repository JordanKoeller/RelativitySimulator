use std::cmp::{Ord, Ordering};

use specs::Entity;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::thread::ThreadId;

use utils::{getSyncMutRef, Mat4F, SyncMutRef};

use debug::*;
use ecs::{DrawableId, Material};
use renderer::RenderCommand;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct DrawCall {
    pub drawable: DrawableId,
    pub entity: Entity,
    pub cmd: RenderCommand,
}

#[inline]
fn reverse(o: Ordering) -> Ordering {
    match o {
        Ordering::Less => Ordering::Greater,
        Ordering::Greater => Ordering::Less,
        Ordering::Equal => Ordering::Equal,
    }
}

impl Ord for DrawCall {
    fn cmp(&self, other: &Self) -> Ordering {
        let comparisons: [Ordering; 4] = [
            self.drawable.1.cmp(&other.drawable.1), // shader compare
            self.drawable.0.cmp(&other.drawable.0), // drawable id compare
            self.entity.cmp(&other.entity),         // Entity compare
            self.cmd.cmp(&other.cmd),               // Command key compare
        ];
        for comparison in comparisons {
            if comparison != Ordering::Equal {
                return reverse(comparison);
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for DrawCall {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default)]
pub struct RenderQueue {
    queue: SyncMutRef<BinaryHeap<DrawCall>>,
}

impl RenderQueue {
    pub fn push(&self, cmd: DrawCall) {
        self.queue
            .lock()
            .expect("Could not unlock Render Command Queue")
            .push(cmd);
    }

    pub fn pop(&self) -> Option<DrawCall> {
        let ret = self.queue.lock().expect("Could not unlock Render Command Queue").pop();
        if let Some(dc) = ret {
            // step_debug!(format!("Dequeueing {:?}", dc.drawable));
            Some(dc)
        } else {
            None
        }
    }

    pub fn pop_if<F: FnOnce(&DrawCall) -> bool>(&self, cond: F) -> Option<DrawCall> {
        let mut queue_ref = self.queue.lock().expect("Could not unlock Render Command Queue");
        if queue_ref
            .peek()
            .map(|x| if cond(x) { Some(true) } else { None })
            .flatten()
            .is_some()
        {
            queue_ref.pop()
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<DrawCall> {
        if let Some(v) = self.queue.lock().expect("Could not unlock Render Command Queue").peek() {
            Some(v.clone())
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.queue.lock().expect("Could not unlock Render Command Queue").len()
    }

    pub fn consume(&mut self) -> RenderQueueConsumer<'_> {
        RenderQueueConsumer(self)
    }
}

pub struct RenderQueueConsumer<'a>(&'a mut RenderQueue);

impl<'a> Iterator for RenderQueueConsumer<'a> {
    type Item = DrawCall;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a> Drop for RenderQueueConsumer<'a> {
    fn drop(&mut self) {
        while self.next().is_some() {}
    }
}

impl<'a> RenderQueueConsumer<'a> {
    pub fn peek(&self) -> Option<DrawCall> {
        self.0.peek()
    }

    pub fn pop_if<F: FnOnce(&DrawCall) -> bool>(&mut self, f: F) -> Option<DrawCall> {
        self.0.pop_if(f)
    }

    pub fn consumed(&self) -> bool {
        self.0.len() == 0
    }
}
