pub trait GLElement {
    fn attrib_lengths(&self) -> Vec<u32>;
    fn elem_length(&self) -> u32 {
        self.attrib_lengths().len() as u32
    }
    fn attrib_offsets(&self) -> Vec<u32> {
        let mut acc = 0;
        let mut ret = self.attrib_lengths();
        for i in 0..ret.len() {
            let curr_value = ret[i];
            ret[i] = acc;
            acc += ret[i];
        }
        ret
    }
}

// pub trait DrawableImpl {
//     fn shader_id(&self) -> String;

//     // fn special_uniforms(&self) -> Iterator<(String, UniformValue)>;

//     fn draw(&self);
// }

// #[derive(Clone)]
// pub struct VAO {
//     pub vbo: u32,
//     pub vao: u32,
//     pub ebo: u32,
// }


// impl VAO {
//     pub fn new() -> VAO {
//         VAO { vao: 0, vbo: 0, ebo: 0}
//     }
// }