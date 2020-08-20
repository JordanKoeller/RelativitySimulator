use renderer::{Asset, UniformManager, UniformValue, Shader};

// pub struct BaseRenderable {
//     asset: Asset,
//     uniform_manager: UniformManager,
// }

pub trait IRenderable {
    fn uniform_manager(&self) -> &UniformManager;
    fn uniform_manager_mut(&mut self) -> &mut UniformManager;
    fn render(&self);
    fn shader(&self) -> &Shader;

    fn update_uniforms(&mut self, unifs:Box<dyn Iterator<Item = (String, UniformValue)>>) {
        for (u_name, u_value) in unifs {
            self.uniform_manager_mut().set(&u_name, u_value);
        }
    }

}

pub trait BaseRenderable: IRenderable {
    // Requires the following still be defined:
    // fn asset(&self) -> &Asset;
    // fn uniform_manager(&self) -> &UniformManager;
    // fn uniform_manager_mut(&mut self) -> &mut UniformManager;
    fn asset(&self) -> &Asset;
    fn uniform_manager(&self) -> &UniformManager;
    fn uniform_manager_mut(&mut self) -> &mut UniformManager;
}

impl<T: BaseRenderable>  IRenderable for T {
    fn uniform_manager(&self) -> &UniformManager {
        <T as BaseRenderable>::uniform_manager(&self)
    }
    fn uniform_manager_mut(&mut self) -> &mut UniformManager {
        <T as BaseRenderable>::uniform_manager_mut(self)
    }
    fn render(&self) {
        self.asset().draw(<T as BaseRenderable>::uniform_manager(&self));
    }
    fn shader(&self) -> &Shader {
        &self.asset().shader
    }
}

