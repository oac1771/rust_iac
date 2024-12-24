use crate::provider::dummy_provider::prelude::*;
use state_macro::state;

state! {

    #[resource(name = foo)]
    DummyResourceA {
        id_a: 10,
    }

    #[resource(name = zip)]
    DummyResourceB {
        id_b: foo.id_a,
    }

    #[resource(name = bar)]
    DummyResourceA {
        id_a: zip.id_b,
    }

}
