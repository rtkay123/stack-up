use sellershut_services::tracing::TracingBuilder;

fn main() {
    let _tracing = TracingBuilder::default().build(None);

    tracing::info!("hello from tracing");
}
