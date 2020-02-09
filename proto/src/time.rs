//! Time synchronization

use eviov_types::Time;

codegen::proto! {
    /// Time synchronization.
    ///
    /// Time synchronization is used to synchronize the "game time" between processes.

    name = "eviov-time";

    /// Queries the current game time
    client query When {} -> {
        /// The current gsme time when the server received the query
        time: Time,
    }
}
