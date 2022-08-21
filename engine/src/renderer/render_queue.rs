use std::cmp::{Ord, Ordering};
use std::sync::{RwLock, RwLockReadGuard};

use specs::Entity;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::thread::ThreadId;

use crate::utils::{getSyncMutRef, Mat4F, SyncMutRef};

use crate::datastructures::{AVLTree, AVLTreeIterator};
use crate::debug::*;
use crate::graphics::MeshComponent;
use crate::renderer::RenderCommand;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct DrawCall {
    pub mesh_component: MeshComponent,
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
            self.mesh_component
                .shader_id
                .get()
                .cmp(&other.mesh_component.shader_id.get()),
            self.mesh_component
                .vertex_array_id
                .get()
                .cmp(&other.mesh_component.vertex_array_id.get()),
            self.entity.cmp(&other.entity), // Entity compare
            self.cmd.cmp(&other.cmd),       // Command key compare
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
    queue: RwLock<AVLTree<DrawCall>>,
}

impl RenderQueue {
    pub fn push(&self, cmd: DrawCall) {
        let _ = self.queue.write().map(|mut tree| tree.push(cmd));
    }

    pub fn len(&self) -> usize {
        self.queue.read().expect("Could not unlock Render Command Queue").len()
    }

    pub fn iter(&mut self) -> RwLockReadGuard<'_, AVLTree<DrawCall>> {
        self.queue.read().expect("Could not acquire Renderer AVLT ReadLock")
    }

    pub fn drain(&mut self) {
        self.queue.write().map(|mut tree| tree.drain()).ok();
    }
}

pub struct RenderQueueConsumer<'a>(AVLTreeIterator<'a, DrawCall>);

impl<'a> RenderQueueConsumer<'a> {
    pub fn next(&mut self) -> Option<&DrawCall> {
        self.0.next()
    }

    pub fn peek(&self) -> Option<&DrawCall> {
        self.0.peek()
    }

    pub fn empty(&self) -> bool {
        self.0.empty()
    }
}
