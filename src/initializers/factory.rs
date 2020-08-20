

pub trait Factory {

    type Resource;
    type Spec;

    fn new_resource(&self, spec: Self::Spec) -> Self::Resource;
}