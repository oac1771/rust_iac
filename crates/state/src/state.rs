use crate::provider::dummy_provider::prelude::*;
use state_macro::state;

state! {

    #[resource(name = zip)]
    DummyResourceB {
        id_b: bar.id_a,
    }

    #[resource(name = bar)]
    DummyResourceA {
        id_a: 10,
    }
}
