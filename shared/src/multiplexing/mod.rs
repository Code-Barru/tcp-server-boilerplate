/// Stream ID type alias for clarity
pub type StreamId = u32;

/// Control stream ID (reserved for protocol control messages)
pub const CONTROL_STREAM_ID: StreamId = 0;

/// Minimum stream ID for application data
pub const MIN_DATA_STREAM_ID: StreamId = 1;
