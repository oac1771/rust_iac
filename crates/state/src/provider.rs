use provider_macro::provider;

#[provider]
mod dummy_provider {

    #[provider_definition]
    struct DummyProvider;

    #[provider_implementation]
    impl Provider for DummyProvider {
        const url: &'static str = "http://foo.com";

        fn get<R: Resource>(&self, resource: R) -> R::Response {
            resource.payload()
        }
    }

    #[resource_definition]
    struct DummyResourceA {
        id_a: i32,
    }

    #[resource_implementation]
    impl Resource for DummyResourceA {
        type Response = String;

        fn payload(&self) -> Self::Response {
            "DummyResourceA".to_string()
        }
    }

    #[resource_definition]
    struct DummyResourceB {
        id_b: i32,
    }

    #[resource_implementation]
    impl Resource for DummyResourceB {
        type Response = String;

        fn payload(&self) -> Self::Response {
            "DummyResourceB".to_string()
        }
    }
}
