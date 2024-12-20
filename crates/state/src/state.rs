use state_macro::state;

#[state]
mod foo {

    #[resource(name = foo)]
    struct ResourceA {
        id_a: String,
    }

    #[resource(name = zip)]
    struct ResourceB {
        ids: String,
    }
}
