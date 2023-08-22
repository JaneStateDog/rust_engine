use crate::component::*;

use std::collections::HashMap;

pub struct Entity {
    component_hm: HashMap<String, Vec<Component>>,
}

impl Entity {
    pub fn new(component_vec: Vec<Component>) -> Self {
        let mut component_hm = HashMap::new();
        for component in component_vec {
            let name = component.borrow().name();
            component_hm.entry(name)
                .or_insert(Vec::new())
                .push(component);

        }

        Self {
            component_hm: component_hm,
        }
    }

    pub fn get_components_from_name(&mut self, name: &str) -> Option<&Vec<Component>> {
        self.component_hm.get(name)
    }
}