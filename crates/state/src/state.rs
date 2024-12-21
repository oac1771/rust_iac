use crate::provider::dummy_provider::prelude::*;
use state_macro::state;

state! {
    #[resource(name = foo)]
    DummyResourceA {
        id_a: 10,
    }
}

// fn foo() {
//     let foo = DummyResourceA { id_a: 10 };
// }
