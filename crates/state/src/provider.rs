use provider_macro::provider;

#[provider]
mod dummy_provider {

    #[provider_definition]
    struct DummyProvider;

    #[provider_implementation]
    impl Provider for DummyProvider {
        const url: &'static str = "http://foo.com";

        fn get<R: Resource>(&self, resource: &R) -> R::Payload {
            resource.payload()
        }
    }

    #[resource_definition(outputs = {output_id: String})]
    struct DummyResourceA {
        id_a: i32,
    }

    #[resource_implementation]
    impl Resource for DummyResourceA {
        type Payload = String;

        fn payload(&self) -> Self::Payload {
            "DummyResourceA".to_string()
        }
    }

    #[resource_definition]
    struct DummyResourceB {
        id_b: i32,
    }

    #[resource_implementation]
    impl Resource for DummyResourceB {
        type Payload = String;

        fn payload(&self) -> Self::Payload {
            "DummyResourceB".to_string()
        }
    }
}
