use provider_macro::provider;

#[provider]
mod dummy_provider {

    #[provider_definition]
    struct DummyProvider;

    #[resource_definition]
    struct DummyResourceA {
        id_a: i32,
    }

    #[resource_definition]
    struct DummyResourceB {
        id_b: i32,
    }
}
