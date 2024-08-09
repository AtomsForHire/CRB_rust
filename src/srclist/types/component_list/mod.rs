//! An alternative to ['SourceList'] (not the hyperdrive implementation).
//! Follows the original python implementation of the CRB code a bit more.

use super::{FluxDensity, FluxDensityType, SourceComponent, SourceList};
use marlu::{RADec, LMN};

use std::ops::{Deref, DerefMut, Index, IndexMut};

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

    /// Veto sources by the minimum flux
    pub(crate) fn veto_by_flux(&mut self, noise: f64) {
        self.retain(|comp| match comp.flux_type {
            FluxDensityType::PowerLaw {
                fd: FluxDensity { i, .. },
                ..
            } => return i > noise,
            FluxDensityType::CurvedPowerLaw {
                fd: FluxDensity { i, .. },
                ..
            } => return i > noise,
            FluxDensityType::List { .. } => return false,
        });
    }

    /// Veto sources by fov
    pub(crate) fn veto_by_fov(&mut self, phase_centre: RADec, lambda: f64, D: f64) {
        println!("fov: {}", (lambda / (D * 2.0f64)).sin());
        return self.retain(|comp| {
            let fov = lambda / D;
            let lmn = comp.radec.to_lmn(phase_centre);

            if (lmn.l.powi(2) + lmn.m.powi(2)).sqrt() < (fov / 2.0f64).sin() {
                return true;
            } else {
                return false;
            }
        });
    }

    pub(crate) fn slice_to_struct(&self, range: std::ops::Range<usize>) -> Self {
        Self(self.0[range].to_vec())
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

impl Index<std::ops::Range<usize>> for ComponentList {
    type Output = [SourceComponent];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<std::ops::Range<usize>> for ComponentList {
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}
