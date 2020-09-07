use cgmath::prelude::*;
use utils::Vec3F;

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

  fn attributes(&self) -> &Vec<AttributeTypes>;

  fn has_attribute(&self, att: AttributeTypes) -> bool {
    self.attributes().iter().find(|x| matches!(x, att)).is_some()
  }

  fn offset_of(&self, att: AttributeTypes) -> Option<i32> {
    if self.has_attribute(att) {
      self
        .attrib_lengths()
        .iter()
        .zip(self.attributes().iter())
        .find_map(|(&offset, attrib)| if let att = attrib { Some(offset) } else { None })
    } else {
      None
    }
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
pub struct ElementSpec {
  att_lengths: Vec<i32>,
  attributes: Vec<AttributeTypes>,
}

impl ElementSpec {
  pub fn new(attributes: Vec<AttributeTypes>) -> ElementSpec {
    ElementSpec {
      att_lengths: attributes
        .iter()
        .map(|a| match a {
          AttributeTypes::UVCoords => 2,
          _ => 3,
        })
        .collect(),
      attributes,
    }
  }
}
impl GLLayout for ElementSpec {
  fn attrib_lengths(&self) -> Vec<i32> {
    self.att_lengths.clone()
  }

  fn attributes(&self) -> &Vec<AttributeTypes> {
    &self.attributes
  }
}

struct Triangle {
  a: Vec3F,
  b: Vec3F,
  c: Vec3F,
  ind: usize
}

#[derive(Clone)]
pub struct GLSpec {
  pub points_buffer: Vec<f32>,
  pub inds_buffer: Vec<u32>,
  pub elem_spec: ElementSpec,
}

impl GLSpec {
  pub fn new(pts: Vec<f32>, inds: Vec<u32>, elems: Vec<AttributeTypes>) -> GLSpec {
    GLSpec {
      points_buffer: pts,
      inds_buffer: inds,
      elem_spec: ElementSpec::new(elems),
    }
  }

  fn normalize_vectors(&mut self, stride: usize, offset: usize) {
    for i in (offset..self.points_buffer.len()).step_by(stride) {
      let i = i as usize;
      let v = Vec3F::new(
        self.points_buffer[i],
        self.points_buffer[i + 1],
        self.points_buffer[i + 2],
      );
      let normed = v.normalize();
      self.points_buffer[i] = normed.x;
      self.points_buffer[i + 1] = normed.y;
      self.points_buffer[i + 2] = normed.z;
    }
  }

  pub fn compute_lighting(&mut self) {
    let stride = self.elem_spec.elem_length() as usize;
    if self.elem_spec.has_attribute(AttributeTypes::Normals)
      && self.elem_spec.has_attribute(AttributeTypes::Tangents)
      && self.elem_spec.has_attribute(AttributeTypes::Bitangents)
    {
      // Already has the necessary vectors so I'm just
      // gonna normalize them and exit
      // self.elem_spec.offset_of(AttributeTypes::Normals).and_then(|x| )
      self.normalize_vectors(
        stride,
        self.elem_spec.offset_of(AttributeTypes::Normals).unwrap() as usize,
      );
      self.normalize_vectors(
        stride,
        self.elem_spec.offset_of(AttributeTypes::Tangents).unwrap() as usize,
      );
      self.normalize_vectors(
        stride,
        self.elem_spec.offset_of(AttributeTypes::Bitangents).unwrap() as usize,
      );
    } else {
      let pt_offset = self.elem_spec.offset_of(AttributeTypes::Points).unwrap() as usize;
      let points: Vec<Vec3F> = (pt_offset..self.points_buffer.len())
        .step_by(stride)
        .map(|i| {
          Vec3F::new(
            self.points_buffer[i],
            self.points_buffer[i + 1],
            self.points_buffer[i + 2],
          )
        })
        .collect();
      let mut normals_buffer: Vec<Vec3F> = Vec::with_capacity(points.len());
      // let mut tangents_buffer = Vec::with_capacity(points.len()); //TODO Support tangents
      // let mut bitangents_buffer = Vec::with_capacity(points.len()); //TODO Support Bitangents
      for tI in (0..self.inds_buffer.len()).step_by(3) {
        let v1 = points[self.inds_buffer[tI + 1] as usize] - points[self.inds_buffer[tI] as usize];
        let v2 = points[self.inds_buffer[tI + 2] as usize] - points[self.inds_buffer[tI] as usize];
        let norm = v1.cross(v2);
        normals_buffer[self.inds_buffer[tI] as usize] += norm;
        normals_buffer[self.inds_buffer[tI+1] as usize] += norm;
        normals_buffer[self.inds_buffer[tI+2] as usize] += norm;
      }
      for i in 0..normals_buffer.len() {
        normals_buffer[i] = normals_buffer[i].normalize();
      }
      let mut new_values: Vec<f32> = Vec::with_capacity(normals_buffer.len() * 3 + self.points_buffer.len());
      let mut new_attribs = self.elem_spec.attributes.clone();
      new_attribs.push(AttributeTypes::Normals);
      let new_elems = ElementSpec::new(new_attribs);
      let new_stride = new_elems.elem_length() as usize;
      for i in 0..points.len() {
        for j in 0..stride {
          new_values[i*new_stride + j] = self.points_buffer[i*stride + j];
        }
        new_values[(i+1)*new_stride] = normals_buffer[i].x;
        new_values[(i+1)*new_stride + 1] = normals_buffer[i].y;
        new_values[(i+1)*new_stride + 2] = normals_buffer[i].z;
      }
      self.elem_spec = new_elems;
      self.points_buffer = new_values;
    }
  }
}
