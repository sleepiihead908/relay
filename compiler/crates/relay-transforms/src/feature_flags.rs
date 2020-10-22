/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use interner::StringKey;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct FeatureFlags {
    #[serde(default)]
    pub enable_flight_transform: bool,

    pub enable_required_transform_for_prefix: Option<StringKey>,

    #[serde(default)]
    pub no_preloadable_metadata: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        FeatureFlags {
            enable_flight_transform: false,
            enable_required_transform_for_prefix: None,
            no_preloadable_metadata: false,
        }
    }
}
