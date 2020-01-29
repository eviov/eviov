use crate::math::Time;

codegen::proto! {
    /// Time synchronization.
    ///
    /// Time synchronization is used to synchronize the "game time" between processes.
    "eviov-time";

    /// Queries the current game time
    client query Ask {} -> {
        /// The current gsme time when the server received the query
        time: Time,
    }
}
