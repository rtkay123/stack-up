#[cfg(any(feature = "nats-jetstream", feature = "nats-core"))]
pub mod nats {
    pub mod extractor {
        pub struct HeaderMap<'a>(pub &'a async_nats::HeaderMap);

        impl opentelemetry::propagation::Extractor for HeaderMap<'_> {
            fn get(&self, key: &str) -> Option<&str> {
                self.0
                    .get(async_nats::header::IntoHeaderName::into_header_name(key))
                    .map(|value| value.as_str())
            }

            fn keys(&self) -> Vec<&str> {
                self.0.iter().map(|(n, _v)| n.as_ref()).collect()
            }
        }
    }

    pub mod injector {
        pub struct HeaderMap<'a>(pub &'a mut async_nats::HeaderMap);

        impl opentelemetry::propagation::Injector for HeaderMap<'_> {
            fn set(&mut self, key: &str, value: String) {
                self.0.insert(key, value);
            }
        }
    }
}

#[cfg(feature = "tonic")]
pub mod tonic {
    pub mod extractor {
        pub struct MetadataMap<'a>(&'a tonic::metadata::MetadataMap);
        impl opentelemetry::propagation::Extractor for MetadataMap<'_> {
            fn get(&self, key: &str) -> Option<&str> {
                self.0.get(key).and_then(|metadata| metadata.to_str().ok())
            }

            /// Collect all the keys from the MetadataMap.
            fn keys(&self) -> Vec<&str> {
                self.0
                    .keys()
                    .map(|key| match key {
                        tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                        tonic::metadata::KeyRef::Binary(v) => v.as_str(),
                    })
                    .collect::<Vec<_>>()
            }
        }
    }

    pub mod injector {
        pub struct MetadataMap<'a>(&'a mut tonic::metadata::MetadataMap);

        impl opentelemetry::propagation::Injector for MetadataMap<'_> {
            /// Set a key and value in the MetadataMap.  Does nothing if the key or value are not valid inputs
            fn set(&mut self, key: &str, value: String) {
                if let Ok(key) = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()) {
                    if let Ok(val) = tonic::metadata::MetadataValue::try_from(&value) {
                        self.0.insert(key, val);
                    }
                }
            }
        }
    }
}
