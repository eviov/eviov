//! Client-system communications.

/// The observer channel
pub mod obs {
    use eviov_types::{Eci, Length, ObjectId, Time, Vector};
    use serde::{Deserialize, Serialize};

    codegen::proto! {
        /// Client-system observer channel

        name = "eviov-cs-observer";

        /// Identifies the client to acknowledge that it is authorized to observe
        client query Handshake {
            /// The session ID
            session: u64,
        } -> {
            /// The game time for the states described in this response.
            ///
            /// This time value should NOT be used for time calibration.
            /// Time calibration should use the `time` protocol.
            time: Time,
            // TODO send initial states
        }

        /// Reports a change of object states in the system.
        server message Event {
            /// The game time at which this event occurred.
            ///
            /// This time is deliberately expected to be inconsistent with the synchronized time.
            /// The client should perform extrapolation to calculate the propagated effects.
            time: Time,
            /// The changes involved in this event.
            content: EventContent,
        }

        /// Acknowledges thst the channnel is closed, especially if the client is denied from
        /// observation.
        server message Close {}
    }

    /// Contents of an event.
    #[derive(Debug, Serialize, Deserialize)]
    pub enum EventContent {
        /// The ECI and acceleration of an object has changed.
        Accel(AccelEvent), // add info
    }

    /// Data for an acceleration event.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AccelEvent {
        /// The object described in this event.
        pub object: ObjectId,
        /// The new acceleration of the object.
        pub accel: Vector<Length>,
        /// The ECI posvel of the object.
        pub eci: Eci,
    }
}

/// The control channel
pub mod ctrl {
    use eviov_types::ObjectId;

    codegen::proto! {
        /// Client-system controller channel.

        name = "eviov-cs-control";

        /// Requests control on an object.
        client query Handshake {
            /// The object to control
            object: ObjectId,
            /// The password to authenticate control
            password: u64,
        } -> {
            // TODO send initial states
        }

        /// Describes the client's request to update control states.
        client message Control {
            // TODO
        }

        /// Reports updates to the control states of the controlled object.
        server message Update {
            // TODO
        }
    }
}
