use std::backtrace::Backtrace;
use std::str::FromStr;
use std::{fmt, io};

use owo_colors::OwoColorize;
use tracing::metadata::LevelFilter;
use tracing::{Event, Level, Subscriber};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::filter::Directive;
use tracing_subscriber::fmt::{format, FmtContext, FormatEvent, FormatFields, FormattedFields};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use crate::cli::error::LogError;

use super::fs::try_create_dir;

///
/// The logging macro is defined here to allow us to change
/// only one file if we want to change the logging strategy.
#[macro_export]
macro_rules! ltr_log {
	($level:expr, $($args:tt)+) => {
		tracing::event!($level, $($args)+)
	};
}

#[macro_export]
macro_rules! ltr_info {
	($($args:tt)+) => {
		$crate::ltr_log!(tracing::Level::INFO, $($args)+)
	};
}

#[macro_export]
macro_rules! ltr_warn {
	($($args:tt)+) => {
		$crate::ltr_log!(tracing::Level::WARN, $($args)+)
	};
}

#[macro_export]
macro_rules! ltr_error {
	($($args:tt)+) => {
		$crate::ltr_log!(tracing::Level::ERROR, $($args)+)
	};
}

#[macro_export]
macro_rules! ltr_debug {
	($($args:tt)+) => {
		$crate::ltr_log!(tracing::Level::DEBUG, $($args)+)
	};
}

///
/// This function should be at the main file.
#[inline(always)]
pub fn setup_backtrace() {
    if let Ok(value) = std::env::var("LTR_BACKTRACE") {
        std::env::set_var("RUST_BACKTRACE", value);
    }
}

///
/// Logs the error backtrace if possible.
/// Remeber, to enable or disable you should modify
/// the variable RUST_BACKTRACE.
pub fn log_backtrace(backtrace: Backtrace) {
    match Backtrace::status(&backtrace) {
        std::backtrace::BacktraceStatus::Unsupported => {
            ltr_error!("Unfortunately, we can't detail the error deeply.");
            ltr_error!(
                "Capturing a backtrace is not supported, likely because it's not implemented for \
				 the current platform."
            )
        }
        std::backtrace::BacktraceStatus::Disabled => {
            ltr_error!(
                "Unfortunately, we can't detail the error because the backtrace is disabled"
            );
            ltr_error!(
                "Capturing a backtrace has been disabled through the LTR_BACKTRACE environment \
				 variable."
            )
        }
        std::backtrace::BacktraceStatus::Captured => {
            ltr_error!("Error backtrace:\n {backtrace}");
        }
        _ => unreachable!(),
    }
}

///
/// Utilitary function that prints long messages into
/// separate lines.
pub fn log_multiline_error(msg: String) {
    msg.split('\n')
        .into_iter()
        .for_each(|line| ltr_error!("{}", line));
}

/// Logging initialization. If something fails, it will panic.
pub fn init_logger() {
    // SAFETY: Panic here should be unreachable. The  only reason
    // that can make it panic would be if we already have another logger
    // initialized, which should never be the case!!
    setup_logger().expect("Logger couldn't take off successfully!")
}

/// Setup logs formatting and redirect to desired streams.
/// If the file logger initializes, it will not break the code.
fn setup_logger() -> Result<(), LogError> {
    let indicatif_layer = IndicatifLayer::new();

    let collector = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .event_format(LtrLogFormat)
                .with_writer(io::stdout)
                .with_filter(log_filter(*consts::LOG_LEVEL)),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(indicatif_layer.get_stdout_writer())
                .with_filter(disabled_logs_filter()),
        )
        .with(indicatif_layer);

    // SAFETY: the `tracing_appender` if the directory doesn't exist. Thus, we
    // guarantee that it exists or just don't initiliaze logfile and move forward
    // with stdout only.
    match try_create_dir(&consts::LOG_DIR) {
        Ok(_) => {
            let file_appender =
                tracing_appender::rolling::never(&*consts::LOG_DIR, &*consts::LOG_FILE);
            collector
                .with(
                    tracing_subscriber::fmt::layer()
                        .event_format(LtrLogFileFormat)
                        .with_writer(file_appender)
                        .with_filter(log_filter(*consts::LOG_LEVEL)),
                )
                .try_init()
        }
        Err(err) => {
            // Try to initialize the log before print the I/O error.
            //
            // This fails only if there's is another logger initialized as stdout
            // is not expected to panic.
            collector.try_init().map_err(LogError::from)?;

            // Write I/O error about file logging into log.
            tracing::warn!("{}", err);

            Ok(())
        }
    }
    .map_err(LogError::from)
}

///
/// Default formatting for terminal logs
struct LtrLogFormat;
impl<S, N> FormatEvent<S, N> for LtrLogFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        write_log(ctx, &mut writer, event, true)
    }
}

///
/// Default formatting for file  logs
struct LtrLogFileFormat;
impl<S, N> FormatEvent<S, N> for LtrLogFileFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        write_log(ctx, &mut writer, event, false)
    }
}

fn write_log<S, N>(
    ctx: &FmtContext<'_, S, N>,
    writer: &mut format::Writer<'_>,
    event: &Event<'_>,
    colorized: bool,
) -> fmt::Result
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    let metadata = event.metadata();

    if colorized {
        write!(
            writer,
            "{} | {:<15} | ",
            chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ"),
            colorize_level(metadata.level())
        )?;
    } else {
        write!(
            writer,
            "{} | {:<5} | ",
            chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ"),
            metadata.level()
        )?;
    };

    // Format all the spans in the event's span context.
    if let Some(scope) = ctx.event_scope() {
        for span in scope.from_root() {
            let ext = span.extensions();
            if let Some(fields) = &ext.get::<FormattedFields<N>>() {
                // Skip formatting the fields if the span had no fields.
                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, ": ")?;
            }
        }
    }
    // Write fields on the event
    ctx.field_format().format_fields(writer.by_ref(), event)?;
    writeln!(writer)
}

fn colorize_level(level: &Level) -> String {
    match *level {
        Level::TRACE => level.blue().to_string(),
        Level::DEBUG => level.blue().to_string(),
        Level::INFO => level.green().to_string(),
        Level::WARN => level.yellow().to_string(),
        Level::ERROR => level.red().to_string(),
    }
}

fn log_filter(level: Level) -> EnvFilter {
    EnvFilter::default()
        .add_directive(Directive::from(level))
        // SAFETY: static definitions should work always
        .add_directive(Directive::from_str("tokio_postgres=error").unwrap())
        .add_directive(Directive::from_str("postgres=error").unwrap())
        .add_directive(Directive::from_str("mysql=error").unwrap())
        .add_directive(Directive::from_str("kmt_core=error").unwrap())
        .add_directive(Directive::from_str("sqlcrypt=error").unwrap())
        .add_directive(Directive::from_str("sqlparse=error").unwrap())
        .add_directive(Directive::from_str("sqlparser=error").unwrap())
        .add_directive(Directive::from_str("metadata=error").unwrap())
}

fn disabled_logs_filter() -> EnvFilter {
    EnvFilter::default().add_directive(LevelFilter::OFF.into())
}

pub(crate) mod consts {
    use std::env;
    use std::path::PathBuf;

    use once_cell::sync::Lazy;

    use super::*;

    pub static LOG_LEVEL: Lazy<tracing::Level> = Lazy::new(|| {
        env::vars()
            .find(|(key, _)| key == "LTR_LOG_LEVEL")
            .and_then(|(_, level)| level.parse().ok())
            .unwrap_or(Level::INFO)
    });

    pub static LOG_FILE: Lazy<PathBuf> = Lazy::new(|| {
        env::vars()
            .find(|(key, _)| key == "LTR_LOG_FILE")
            .map(|(_, dir)| PathBuf::from(dir))
            .unwrap_or_else(|| {
                let timestamp = chrono::Utc::now().format("%Y-%m-%d-%H-%M-%S").to_string();
                PathBuf::from(format!("ltr-{timestamp}.log"))
            })
    });

    pub static DIR_NAME: Lazy<String> = Lazy::new(|| {
        env::vars()
            .find(|(key, _)| key == "LTR_DIR_NAME")
            .map(|(_, dir)| dir)
            .unwrap_or_else(|| String::from("logs"))
    });

    /// Directory location.
    ///
    /// - **Default**: `$LTR_DIR_NAME/log` where `LTR_DIR_NAME` is binary
    /// location.
    /// - **Overrides**: `LTR_LOG_DIR`.
    pub static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| {
        env::vars()
            .find(|(key, _)| key == "LTR_LOG_DIR")
            .map(|(_, dir)| PathBuf::from(dir))
            .unwrap_or_else(|| {
                env::current_exe()
                    .ok()
                    .and_then(|p| p.parent().map(|p| p.join(&*DIR_NAME.clone())))
                    .unwrap_or_else(env::temp_dir)
            })
    });
}
