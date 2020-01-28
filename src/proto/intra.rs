use crate::math::Eci;
use crate::ObjectId;

// Server = parent large body, Client = child large body

codegen::proto! {
    client message Handshake {
        system: ObjectId,
    }

    client query WhereMe {} -> {
        eci: Eci,
    }

    mutual query AllowObserve {
        session: u64,
    } -> {}

    mutual message RevokeObserve {
        session: u64,
    }

    mutual query TransferChild {
        object: FullObject,
    } -> {
        // ACK is needed. If no ACK, perform smooth collision.
    }

    server message TransferYou  {
        dest: ObjectId,
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FullObject {
    id: ObjectId,
}
