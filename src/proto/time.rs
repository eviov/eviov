use crate::math::Time;

codegen::proto! {
    /// Queries the current game time
    client query Ask {} -> {
        /// The current gsme time when the server received the query
        time: Time,
    }
}
