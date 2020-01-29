use crate::ObjectId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ChannelId(pub u32);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ChannelType {
    Obs,
    Ctrl,
}

codegen::proto! {
    "eviov-client-system";

    /// Ooens a new channel with the specified system
    client query OpenChannel {
        /// The type of channel to open
        ty: ChannelType,
        /// The system to oprn channel with
        system: ObjectId,
    } -> {
        /// The channel ID
        ch: ChannelId,
    }

    /// Wraps a message on an `obs` channel from client to server
    client message ObsClient {
        /// The channel to send to
        ch: ChannelId,
        /// The content to send
        inner: obs::FromClient,
    }

    /// Wraps a message on an `obs` channel from server to client
    server message ObsServer {
        /// The channel to send to
        ch: ChannelId,
        /// The content to send
        inner: obs::FromServer,
    }

    /// Wraps a message on an `ctrl` channel from client to server
    client message CtrlClient {
        /// The channel to send to
        ch: ChannelId,
        /// The content to send
        inner: ctrl::FromClient,
    }

    /// Wraps a message on an `ctrl` channel from server to client
    server message CtrlServer {
        /// The channel to send to
        ch: ChannelId,
        /// The content to send
        inner: ctrl::FromServer,
    }
}

/// The observer channel
pub mod obs {
    use crate::math::Time;

    codegen::proto! {
        "eviov-cs-observer";

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

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub enum EventContent {
        Accel(crate::ObjectId), // add info
    }
}

/// The control channel
pub mod ctrl {
    codegen::proto! {
        "eviov-cs-control";

        /// Requests control on an object.
        client query Handshake {
            /// The object to control
            object: crate::ObjectId,
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
