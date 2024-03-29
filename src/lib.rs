#[cfg(test)]
mod tests {
    use tracing::{event, instrument, span, subscriber, Level};
    use tracing_proc_macros::traced_test;
    use tracing_subscriber::FmtSubscriber;

    #[test]
    #[instrument]
    fn test1() {
        let fmt_subscriber = FmtSubscriber::new();
        subscriber::with_default(fmt_subscriber, || {
            event!(Level::INFO, "T1: INFO 1");
            let span = span!(Level::INFO, "my_span");
            let _guard = span.enter();
            event!(Level::DEBUG, "T1: DEBUG 1");
            drop(_guard);
            event!(Level::DEBUG, "T1: DEBUG 2");
            event!(Level::INFO, "T1: INFO 2");
        });
    }

    #[traced_test(FmtSubscriber)]
    fn test2() {
        event!(Level::INFO, "T2: INFO 1");
        let span = span!(Level::INFO, "my_span");
        let _guard = span.enter();
        event!(Level::DEBUG, "T2: DEBUG 1");
        drop(_guard);
        event!(Level::DEBUG, "T2: DEBUG 2");
        event!(Level::INFO, "T2: INFO 2");
    }
}
