use crate::math::{Eci, Time};
use crate::ObjectId;

// Server = parent large body, Client = child large body

codegen::proto! {
    "eviov-intra";

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FullObject {
    id: ObjectId,
}
