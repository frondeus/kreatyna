use super::*;
use parking_lot::Mutex;

#[derive(Default)]
struct RecordingLogger(Mutex<Vec<LogRecord>>);

impl LoggerHandler for RecordingLogger {
    fn log(&self, record: LogRecord) {
        self.0.lock().push(record);
    }
}

fn setup(level: LogLevel) -> (Arc<RecordingLogger>, tracing::Dispatch) {
    let logger = Arc::new(RecordingLogger::default());
    let subscriber = tracing_subscriber::registry()
        .with(LoggerLayer::new(logger.clone()).with_filter(LevelFilter::from(level)));
    (logger, tracing::Dispatch::new(subscriber))
}

fn field(name: &str, value: &str) -> LogField {
    LogField {
        name: name.into(),
        value: value.into(),
    }
}

#[test]
fn preserves_message_fields_and_source() {
    let (logger, dispatch) = setup(LogLevel::Trace);
    let mut source_line = 0;
    tracing::dispatcher::with_default(&dispatch, || {
        source_line = line!() + 1;
        tracing::info!(target: "test.counter", z = true, count = 7_u64, text = "mouse 🐭\nnext", "Counter {}", "updated");
        tracing::info!(count = 8_u64);
    });
    let records = logger.0.lock();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].level, LogLevel::Info);
    assert_eq!(records[0].target, "test.counter");
    assert_eq!(records[0].message.as_deref(), Some("Counter updated"));
    assert_eq!(
        records[0].fields,
        vec![
            field("count", "7"),
            field("text", "mouse 🐭\nnext"),
            field("z", "true")
        ]
    );
    assert_eq!(records[0].file.as_deref(), Some(file!()));
    assert_eq!(records[0].line, Some(source_line));
    assert!(records[0].spans.is_empty());
    assert_eq!(records[1].message, None);
    assert_eq!(records[1].fields, vec![field("count", "8")]);
}

#[test]
fn filters_every_severity_threshold_before_formatting() {
    struct MustNotFormat;
    impl fmt::Debug for MustNotFormat {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("filtered field was formatted");
        }
    }
    let levels = [
        LogLevel::Trace,
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
    ];
    for (index, level) in levels.iter().enumerate() {
        let (logger, dispatch) = setup(level.clone());
        tracing::dispatcher::with_default(&dispatch, || {
            tracing::trace!("trace");
            tracing::debug!("debug");
            tracing::info!("info");
            tracing::warn!("warn");
            tracing::error!("error");
        });
        let actual: Vec<_> = logger
            .0
            .lock()
            .iter()
            .map(|record| record.level.clone())
            .collect();
        assert_eq!(actual, levels[index..]);
    }
    let (logger, dispatch) = setup(LogLevel::Info);
    tracing::dispatcher::with_default(&dispatch, || {
        tracing::debug!(value = ?MustNotFormat, "disabled");
    });
    assert!(logger.0.lock().is_empty());
}

#[test]
fn preserves_nested_spans_and_recorded_updates_without_collisions() {
    let (logger, dispatch) = setup(LogLevel::Trace);
    tracing::dispatcher::with_default(&dispatch, || {
        let root = tracing::info_span!(target: "test.root", "root", id = 1, rows = tracing::field::Empty, message = "span message");
        root.in_scope(|| {
            let child = tracing::info_span!("child", id = 2);
            child.in_scope(|| {
                tracing::info!(id = 3, "before");
                root.record("rows", 12_u64);
                root.record("id", 4);
                tracing::info!(id = 3, "after");
            });
        });
    });
    let records = logger.0.lock();
    assert_eq!(records.len(), 2);
    assert_eq!(
        records[0].spans[0].fields,
        vec![field("id", "1"), field("message", "span message")]
    );
    let after = &records[1];
    assert_eq!(
        after
            .spans
            .iter()
            .map(|span| span.name.as_str())
            .collect::<Vec<_>>(),
        ["root", "child"]
    );
    assert_eq!(after.spans[0].target, "test.root");
    assert_eq!(
        after.spans[0].fields,
        vec![
            field("id", "4"),
            field("message", "span message"),
            field("rows", "12")
        ]
    );
    assert_eq!(after.spans[1].fields, vec![field("id", "2")]);
    assert_eq!(after.fields, vec![field("id", "3")]);
}

#[test]
fn respects_explicit_parent_and_explicit_root_events() {
    let (logger, dispatch) = setup(LogLevel::Trace);
    tracing::dispatcher::with_default(&dispatch, || {
        let parent = tracing::info_span!("explicit");
        let unrelated = tracing::info_span!("unrelated");
        unrelated.in_scope(|| {
            tracing::info!(parent: &parent, "with parent");
            tracing::info!(parent: None, "without parent");
        });
    });
    let records = logger.0.lock();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].spans.len(), 1);
    assert_eq!(records[0].spans[0].name, "explicit");
    assert!(records[1].spans.is_empty());
}

#[test]
fn delivers_concurrently_from_multiple_threads() {
    let (logger, dispatch) = setup(LogLevel::Info);
    std::thread::scope(|scope| {
        for worker in 0..4 {
            let dispatch = dispatch.clone();
            scope.spawn(move || {
                tracing::dispatcher::with_default(&dispatch, || {
                    for sequence in 0..25 {
                        tracing::info!(worker, sequence, "work");
                    }
                })
            });
        }
    });
    let records = logger.0.lock();
    assert_eq!(records.len(), 100);
    let pairs: std::collections::BTreeSet<_> = records
        .iter()
        .map(|record| {
            (
                record.fields[0].value.clone(),
                record.fields[1].value.clone(),
            )
        })
        .collect();
    assert_eq!(pairs.len(), 100);
}

#[test]
fn preserves_span_context_across_async_yields() {
    use tracing::{Instrument, instrument::WithSubscriber};

    let (logger, dispatch) = setup(LogLevel::Info);
    let span =
        tracing::dispatcher::with_default(&dispatch, || tracing::info_span!("worker", id = 42));
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    runtime.block_on(async {
        tokio::spawn(
            async {
                tokio::task::yield_now().await;
                tracing::info!("after yield");
            }
            .instrument(span)
            .with_subscriber(dispatch),
        )
        .await
        .unwrap();
    });
    let records = logger.0.lock();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].spans[0].name, "worker");
    assert_eq!(records[0].spans[0].fields, vec![field("id", "42")]);
}

#[test]
fn initialization_is_process_wide_and_cannot_replace_the_handler() {
    // A subprocess keeps the irreversible global installation out of other tests.
    const CHILD: &str = "MAUSRITTER_LOGGING_INIT_TEST_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "logging::tests::initialization_is_process_wide_and_cannot_replace_the_handler",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }
    let first = Arc::new(RecordingLogger::default());
    initialize_logging(first.clone(), LogLevel::Info).unwrap();
    let second = Arc::new(RecordingLogger::default());
    assert!(matches!(
        initialize_logging(second.clone(), LogLevel::Trace),
        Err(LoggingInitError::AlreadyInitialized)
    ));
    std::thread::spawn(|| {
        tracing::debug!("disabled");
        tracing::info!("worker event");
    })
    .join()
    .unwrap();
    assert_eq!(first.0.lock().len(), 1);
    assert_eq!(first.0.lock()[0].message.as_deref(), Some("worker event"));
    assert!(second.0.lock().is_empty());
}
