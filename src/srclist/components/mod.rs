// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Sky-model component types.
//! Code from mwa_hyperdrive !!!!

use marlu::{pos::xyz::xyzs_to_cross_uvws, AzEl, Jones, LmnRime, RADec, XyzGeodetic, UVW};
use ndarray::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceComponent {
    /// Coordinates struct associated with the component.
    #[serde(flatten)]
    pub radec: RADec,

    /// The type of component.
    pub comp_type: ComponentType,

    /// The flux densities associated with this component.
    pub flux_type: FluxDensityType,
}
