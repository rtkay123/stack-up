#[cfg(any(feature = "nats-jetstream", feature = "nats-core"))]
pub struct NatsHeadersExtraction<'a>(pub &'a async_nats::HeaderMap);

#[cfg(any(feature = "nats-jetstream", feature = "nats-core"))]
impl opentelemetry::propagation::Extractor for NatsHeadersExtraction<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0
            .get(async_nats::header::IntoHeaderName::into_header_name(key))
            .map(|value| value.as_str())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.iter().map(|(n, _v)| n.as_ref()).collect()
    }
}

#[cfg(any(feature = "nats-jetstream", feature = "nats-core"))]
pub struct NatsHeadersInjection<'a>(pub &'a mut async_nats::HeaderMap);

#[cfg(any(feature = "nats-jetstream", feature = "nats-core"))]
impl opentelemetry::propagation::Injector for NatsHeadersInjection<'_> {
    fn set(&mut self, key: &str, value: String) {
        self.0.insert(key, value);
    }
}
