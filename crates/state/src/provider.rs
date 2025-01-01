use provider_macro::provider;

#[provider]
mod dummy_provider {

    #[provider_definition]
    struct DummyProvider;

    #[resource_definition]
    struct DummyResourceA {
        id_a: i32,
    }

    #[resource_implementation]
    impl Resource for DummyResourceA {
        fn foo(&self) -> String {
            "DummyResourceA".to_string()
        }
    }

    #[resource_definition]
    struct DummyResourceB {
        id_b: i32,
    }

    #[resource_implementation]
    impl Resource for DummyResourceB {
        fn foo(&self) -> String {
            "DummyResourceB".to_string()
        }
    }
}
