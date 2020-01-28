use crate::math::Time;

codegen::proto! {
    client query Ask {} -> {
        time: Time,
    }
}
