// use std::any::Any;
// use std::collections::HashMap;

// pub struct Registry {
//     pub objects: Vec<Box<dyn Render>>
// }

// impl Registry {
//     pub fn new(objects: Vec<Box<dyn Render>>) -> Registry {
//         return Registry {
//             objects
//         }
//     }

//     pub fn add(&mut self, obj: Box<dyn Render>) {
//         self.objects.push(obj);
//     }

//     pub fn get(&self, key: usize) -> &Box<dyn Render> {
//         return &self.objects[key]; 
//     }

//     pub fn op<T: 'static>(&mut self, key: usize, operation: fn(&mut T) -> ()) {
//         if let Some(o) = self.objects[key].as_mut_any().downcast_mut::<T>() {
//             operation(o);
//         }
//     }
// }

// pub trait Render {
//     fn render(&self, shader: &crate::shader::Shader);
// }
