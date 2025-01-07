use crate::provider::dummy_provider::prelude::*;
use state_macro::state;

state! {

    #[resource(name = resource_a)]
    DummyResourceB {
        id_b: resource_b.get_id(),
    }

    #[resource(name = resource_b)]
    DummyResourceA {
        id_a: 10,
    }
}
