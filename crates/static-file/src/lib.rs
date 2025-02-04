//! Static file producer implementation.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod event;
pub mod segments;
mod static_file_producer;

pub use event::StaticFileProducerEvent;
pub use static_file_producer::{
    StaticFileProducer, StaticFileProducerResult, StaticFileProducerWithResult, StaticFileTargets,
};
