#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
pub mod tracing;
