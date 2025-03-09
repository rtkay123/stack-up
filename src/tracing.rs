use tracing_subscriber::{
    EnvFilter, Layer, Registry, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::Monitoring;

/// Telemetry handle
#[allow(missing_debug_implementations)]
pub struct Tracing {}

impl Tracing {
    /// Create a new [builder](TracingBuilder)
    /// # Examples
    /// ```
    /// # use sellershut_services::tracing::Tracing;
    /// let _tracing = Tracing::builder();
    /// ```
    pub fn builder() -> TracingBuilder {
        TracingBuilder::default()
    }
}

/// A builder for initialising [tracing] layers
#[allow(missing_debug_implementations)]
pub struct TracingBuilder {
    layer: Vec<Box<dyn Layer<Registry> + Sync + Send>>,
}

impl Default for TracingBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TracingBuilder {
    /// Create a new builder
    /// # Examples
    /// ```
    /// # use sellershut_services::tracing::TracingBuilder;
    /// let _tracing = TracingBuilder::new();
    /// ```
    pub fn new() -> Self {
        let types: Box<dyn Layer<Registry> + Sync + Send> =
            tracing_subscriber::fmt::layer().boxed();
        TracingBuilder { layer: vec![types] }
    }

    #[cfg(feature = "opentelemetry")]
    /// Adds opentelemetry
    pub fn try_with_opentelemetry(
        mut self,
        config: &crate::AppConfig,
        monitoring: &Monitoring,
    ) -> Result<Self, crate::ServiceError> {
        use opentelemetry::{KeyValue, global, trace::TracerProvider};
        use opentelemetry_otlp::WithExportConfig;
        use opentelemetry_sdk::{
            Resource,
            trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
        };
        use opentelemetry_semantic_conventions::{
            SCHEMA_URL,
            resource::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
        };
        use tracing_opentelemetry::OpenTelemetryLayer;

        global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );

        let resource = Resource::builder()
            .with_schema_url(
                [
                    KeyValue::new(SERVICE_NAME, config.name.to_owned()),
                    KeyValue::new(SERVICE_VERSION, config.version.to_owned()),
                    KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, config.env.to_string()),
                ],
                SCHEMA_URL,
            )
            .with_service_name(config.name.to_string())
            .build();

        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(monitoring.opentelemetry_endpoint.as_ref())
            .build()?;

        let provider = SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .with_resource(resource)
            .with_id_generator(RandomIdGenerator::default())
            .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                1.0,
            ))))
            .build();

        global::set_tracer_provider(provider.clone());
        let tracer = provider.tracer(config.name.to_string());

        self.layer.push(OpenTelemetryLayer::new(tracer).boxed());

        Ok(self)
    }

    /// Initialises tracing with the provided level
    /// # Examples
    /// ```
    /// # use sellershut_services::tracing::TracingBuilder;
    //  # let config = crate::Monitoring {
    //  #     log_level: "info".to_string(),
    //  #     #[cfg(feature = "opentelemetry")]
    //  #     opentelemetry_endpoint: "http://localhost:4317".into(),
    //  # };
    /// let _tracing = TracingBuilder::new().build(&config);
    /// ```
    pub fn build(self, config: &Monitoring) -> Tracing {
        tracing_subscriber::registry()
            .with(self.layer)
            .with(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| config.log_level.to_string().into()),
            )
            .try_init()
            .ok();
        Tracing {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build() {
        let builder = Tracing::builder();
        let level = Monitoring {
            log_level: "info".to_string(),
            #[cfg(feature = "opentelemetry")]
            opentelemetry_endpoint: "http://localhost:4317".into(),
        };
        builder.build(&level);
    }
}
