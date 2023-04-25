//!
//! Implementation of a Progress Bar event tracker
use indicatif::ProgressStyle;
use tracing::{info_span, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt;

/// Utilitary function that creates a progress bar
pub fn start(size: u64) -> Span {
    let header_span = info_span!(""); // Needs to be const. That's why it's not a parameter

    // SAFETY: Should never fail static definition
    let style = ProgressStyle::with_template("{wide_bar} {pos}/{len}rows ({per_sec})").unwrap();

    header_span.pb_set_style(&style);
    header_span.pb_set_length(size);

    header_span
}

/// Utilitary function that increments the current progress bar.
///
/// To use this you have to enter in the progress bar first:
///
/// ````
/// let pb = progress_bar::start(10);
/// let pb_enter = pb.enter();
///
/// progress_bar::inc(1);
/// ```
pub fn inc(val: u64) {
    Span::current().pb_inc(val);
}
