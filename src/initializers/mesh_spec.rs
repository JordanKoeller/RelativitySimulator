
pub trait GLLayout {
    fn attrib_lengths(&self) -> Vec<i32>;
    fn elem_length(&self) -> i32 {
        self.attrib_lengths().iter().sum::<i32>() as i32
    }
    fn attrib_offsets(&self) -> Vec<i32> {
        let mut acc = 0;
        let mut ret = self.attrib_lengths();
        for i in 0..ret.len() {
            let curr_value = ret[i];
            ret[i] = acc;
            acc += curr_value;
        }
        ret
    }
}

#[derive(Clone)]
pub enum AttributeTypes {
    Points,
    UVCoords,
    Normals,
    Tangents,
    Bitangents,
}

#[derive(Clone)]
pub struct ElementSpec(Vec<i32>);

impl ElementSpec {
    pub fn new(attributes: Vec<AttributeTypes>) -> ElementSpec {
        ElementSpec(
            attributes
                .iter()
                .map(|a| match a {
                    AttributeTypes::UVCoords => 2,
                    _ => 3,
                })
                .collect(),
        )
    }
}
impl GLLayout for ElementSpec {
    fn attrib_lengths(&self) -> Vec<i32> {
        self.0.clone()
    }
}

#[derive(Clone)]
pub struct GLSpec {
    pub points_buffer: Vec<f32>,
    pub inds_buffer: Vec<u32>,
    pub elem_spec: ElementSpec
}

impl GLSpec {
    pub fn new(pts: Vec<f32>, inds: Vec<u32>, elems: Vec<AttributeTypes>) -> GLSpec {
        GLSpec {
            points_buffer: pts,
            inds_buffer: inds,
            elem_spec: ElementSpec::new(elems)
        }
    }
}