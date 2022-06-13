use anyhow::{Error, Result};
use async_trait::async_trait;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::metric_collector::{MetricCollector, MetricCollectorError};

// TODO: Consider using thiserror.

#[derive(Debug)]
pub enum RunnerError {
    /// We failed to collect metrics for some reason.
    MetricCollectorError(MetricCollectorError),

    /// We couldn't parse the metrics.
    ParseMetricsError(Error),

    UnknownError(Error),
}

impl Display for RunnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RunnerError {}

// TODO: When we have the metric_evaluator, include a vec of those here,
// as well as an overall evaluation.
struct RunnerResult {}

// This runner doesn't block in the multithreading sense, but from the user
// perspective. To run the health check, we pull metrics once, wait, and then
// pull the metrics again. It does not support continually running beyond this
// point. You can imagine smarter versions of this where you store the last seen
// set of metrics, then compare against that, or perhaps even multiple previously
// seen sets of metrics and do more complex analysis. Additionally we could leverage
// things like long polling +/ sticky routing to make it that the client request
// doesn't just hang waiting for the run to complete.

/// todo describe the trait
/// todo assert these trait constraints are necessary
///
/// Note:
///  - Clone is required because multiple calls to spawn need to be static but also share
///      the same todo instance (mostly for the in-memory versions).
///
///  - Sync + Send is required because this will be a member of the todo which needs
///      to be used across async boundaries
///
///  - 'static is required because this will be stored on the todo which needs to be 'static
#[async_trait]
pub trait Runner: Clone + Sync + Send + 'static {
    // TODO: add proper result type.
    async fn run<M: MetricCollector>(&self, target_retriever: M) -> Result<(), RunnerError>;
}