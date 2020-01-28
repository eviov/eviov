#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ChannelId(pub u32);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ChannelType {
    Obs,
    Ctrl,
}

codegen::proto! {
    client query OpenChannel {
        ty: ChannelType,
    } -> {
        ch: ChannelId,
    }

    client message ObsClient {
        ch: ChannelId,
        inner: obs::FromClient,
    }

    server message ObsServer {
        ch: ChannelId,
        inner: obs::FromServer,
    }

    client message CtrlClient {
        ch: ChannelId,
        inner: ctrl::FromClient,
    }

    server message CtrlServer {
        ch: ChannelId,
        inner: ctrl::FromServer,
    }
}

pub mod obs {
    use crate::math::Time;

    codegen::proto! {
        client query Handshake {
            session: u64,
        } -> {
            time: Time,
            // TODO send initial states
        }

        server message Event {
            time: Time,
            content: EventContent,
        }

        server message Close {}
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub enum EventContent {
        Accel(crate::ObjectId),
    }
}

pub mod ctrl {
    codegen::proto! {
        client query Handshake {
            object: crate::ObjectId,
            password: u64,
        } -> {
            // TODO send initial states
        }

        client message Control {
            // TODO
        }

        server message Update {
            // TODO
        }
    }
}
