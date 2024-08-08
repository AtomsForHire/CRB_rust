//! An alternative to ['SourceList'] (not the hyperdrive implementation).
//! Follows the original python implementation of the CRB code a bit more.

use super::{SourceComponent, SourceList};

use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct ComponentList(Vec<SourceComponent>);

impl ComponentList {
    /// Create a component list from an exisiting source_list
    pub(crate) fn new(source_list: SourceList) -> ComponentList {
        let mut component_list: Vec<SourceComponent> = vec![];

        for comp in source_list
            .iter()
            .rev()
            .flat_map(|(_, src)| src.components.iter())
        {
            component_list.push(comp.clone());
        }

        return ComponentList(component_list);
    }
}

// Need these to expose the iter() functionality of Vec
impl Deref for ComponentList {
    type Target = Vec<SourceComponent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ComponentList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
