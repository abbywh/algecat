//! Operations that can appear in a dataflow graph.

/// Identifies a function used by a [`Op::Map`], [`Op::Filter`], or
/// [`Op::PartitionedGroupReduce`].
pub type FnId = usize;

/// Identifies an aggregation used by a reduce operation.
pub type AggId = usize;

/// Identifies an input source read by a scan operation.
pub type SourceId = usize;

/// Identifies another node in the graph.
pub type DepId = usize;

/// Identifies a field selected from a source/map
pub type FiledId = usize;

pub struct WindowSpec {
    size: Duration, // how big the window is
    step: Duration, // how often we update the window
    strategy: WindowStrategy,
}

pub enum WindowStrategy {
    SAWTOOTH, HOPPING, // pitc kind of sucks so maybe lets leave it out
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Scan { source: SourceId }, // should be something like catalyst util
    Map { f: FnId }, // should be something like catalyst util
    Filter { pred: FnId }, //should be something like catalyst util
    /// Reduces all input rows with an associative aggregation.
    Reduce { agg: AggId },
    /// Groups input rows by `key`, then aggregates each group.
    GroupReduce { key: Vec<FieldId>, agg: Vec<AggId>, time: FiledId, window_spec: WindowSpec },

    // TODO support external

    /// Combines this node's input with the input from another graph node.
    Zip { other: DepId },
}
