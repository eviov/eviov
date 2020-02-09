//! Communication between systems.

use eviov_types::{Eci, ObjectId, Time};

codegen::proto! {
    /// Communication between systems.
    ///
    /// In this protocol, the server is the parent system, and the client is the child system.

    name = "eviov-intra";

    /// Binds the connection to a child.
    client message Handshake {
        /// The system of the child
        system: ObjectId,
    }

    /// Requests the ECI position and velocity of the child.
    client query WhereMe {} -> {
        /// Thr game time when the child was at the ECI posvel
        time: Time,
        /// The ECI position and velocity of the child
        eci: Eci,
    }

    /// Allows a session to observing this system.
    mutual query AllowObserve {
        /// The session to allow
        session: u64,
    } -> {}

    /// Denies a session from observing this system.
    mutual message RevokeObserve {
        /// The session to deny
        session: u64,
    }

    /// Transfers a child to the peer system.
    mutual query TransferChild {
        /// A full copy of the object to transfer
        object: FullObject,
    } -> {
        // ACK is needed. If no ACK, perform smooth collision.
    }

    /// Notifies that the child is to be transferred to a new system.
    ///
    /// The connection should be closed upon this message.j
    server message TransferYou {
        /// The object to transfer to
        dest: ObjectId,
    }
}

/// The full information about an object
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FullObject {
    id: ObjectId,
}
