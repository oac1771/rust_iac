use crate::provider::dummy_provider::prelude::*;
use state_macro::state;

#[state]
mod foo {

    // #[resource(name = foo)]
    // struct DummyResourceA {
    //     id_a: 10,
    // }

    // #[resource(name = zip)]
    // struct DummyResourceA {
    //     ids: 10,
    // }
}

fn foo() {
    let foo = DummyResourceA {id_a: 10};
}