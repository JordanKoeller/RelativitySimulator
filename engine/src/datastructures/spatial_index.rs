use crate::Vec3F;

pub trait HasPosition {
    fn position(&self) -> &Vec3F;
}

pub trait SpatialIndex<T: HasPosition> {
    fn push(&mut self, element: T);

    fn set_data(&mut self, data: Vec<T>);

    fn clear(&mut self);

    fn count(&self) -> usize;

    fn query_near(&self, position: &Vec3F, radius: f32) -> Vec<usize>;

    fn query_near_count(&self, position: &Vec3F, radius: f32) -> usize;

    fn data(&self) -> &Vec<T>;
}
