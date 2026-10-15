//! Operations that can appear in a dataflow graph.

use std::time::Duration;

/// Identifies a function used by a [`Op::Map`], [`Op::Filter`], or
/// [`Op::GroupReduce`].
pub type FnId = usize;

/// Identifies an aggregation used by a reduce operation.
pub type AggId = usize;

/// Identifies an input source read by a scan operation.
pub type SourceId = usize;

/// Identifies another node in the graph.
pub type DepId = usize;

/// Identifies a field selected from a source/map
pub type FieldId = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowSpec {
    pub size: Duration, // how big the window is
    pub step: Duration, // how often we update the window
    pub strategy: WindowStrategy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowStrategy {
    Sawtooth,
    Hopping, // PITC kind of sucks, so maybe let's leave it out.
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Scan { source: SourceId }, // should be something like catalyst util
    Map { f: FnId }, // should be something like catalyst util
    Filter { pred: FnId }, //should be something like catalyst util
    /// Reduces all input rows with an associative aggregation.
    Reduce { agg: AggId },
    /// Groups input rows by `key`, then aggregates each group.
    GroupReduce {
        key: Vec<FieldId>,
        agg: Vec<AggId>,
        time: FieldId,
        window_spec: WindowSpec,
    },

    // TODO support external

    /// Combines this node's input with the input from another graph node.
    Zip { other: DepId },
}
