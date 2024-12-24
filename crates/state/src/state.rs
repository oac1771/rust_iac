use crate::provider::dummy_provider::prelude::*;
use state_macro::state;

state! {

    #[resource(name = zip)]
    DummyResourceB {
        id_b: foo.id_a,
    }

    #[resource(name = foo)]
    DummyResourceA {
        id_a: 10,
    }
}
