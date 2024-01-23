// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides two clients that can be used to work with
//! transactions, storage and events. The [`OfflineClient`] works
//! entirely offline and can be passed to any function that doesn't
//! require network access. The [`OnlineClient`] requires network
//! access.

mod offline_client;
#[cfg(feature = "std")]
mod online_client;

crate::macros::cfg_unstable_light_client! {
    mod light_client;

    pub use light_client::{
        LightClient, LightClientBuilder, LightClientError, RawLightClient, RawLightClientBuilder,
    };
}

pub use offline_client::{OfflineClient, OfflineClientT};

#[cfg(feature = "std")]
pub use online_client::{
    ClientRuntimeUpdater, OnlineClient, OnlineClientT, RuntimeUpdaterStream, Update, UpgradeError,
};
