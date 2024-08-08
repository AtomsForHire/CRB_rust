// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Struct to store flux densities from yaml which look like:
//! fd:
//!     freq: ...
//!     i: ...
//!     q: ...
//!     u: ...
//!

use serde::{Deserialize, Serialize};
// vec1 ensures there is at least one item in the vector
use vec1::Vec1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FluxDensity {
    pub freq: f64,

    pub i: f64,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub q: f64,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub u: f64,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub v: f64,
}

/// This is only used for serialisation
// https://stackoverflow.com/questions/53900612/how-do-i-avoid-generating-json-when-serializing-a-value-that-is-null-or-a-defaul
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero(num: &f64) -> bool {
    num.abs() < f64::EPSILON
}

/// Enum for determining what type of Flux Density the component is
/// Could be list of FluxDensity, power_law or curved_power_law
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FluxDensityType {
    List(Vec1<FluxDensity>),

    PowerLaw {
        /// Spectral index
        si: f64,

        /// Flux Density
        fd: FluxDensity,
    },

    CurvedPowerLaw {
        /// Spectral index
        si: f64,

        /// Flux Density
        fd: FluxDensity,

        /// Spectral curvature
        q: f64,
    },
}
