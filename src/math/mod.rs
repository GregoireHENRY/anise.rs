/*
 * ANISE Toolkit
 * Copyright (C) 2021-2022 Christopher Rabotin <christopher.rabotin@gmail.com> et al. (cf. AUTHORS.md)
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Documentation: https://nyxspace.com/
 */

extern crate nalgebra;

// Vector3 is nalgebra's Vector3 with a 64-bit floating point representation.
pub type Vector3 = nalgebra::Vector3<f64>;

pub mod interpolation;

/// Defines the aberration corrections to the state of the target body to account for one-way light time and stellar aberration.
/// **WARNING:** This enum is a placeholder until [https://github.com/anise-toolkit/anise.rs/issues/26] is implemented.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Aberration {
    None,
}
