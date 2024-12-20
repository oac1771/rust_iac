use provider_macro::provider;

#[provider]
mod dummy_provider {

    #[resource_definition]
    struct DummyResourceA {
        id_a: String,
    }

    #[resource_definition]
    struct DummyResourceB {
        id_b: String,
    }
}
