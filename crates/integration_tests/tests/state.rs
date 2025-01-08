#[cfg(test)]
mod test {

    use provider_macro::provider;
    use state_macro::state;

    #[provider]
    mod test_provider {

        #[provider_definition]
        struct TestProvider;

        #[provider_implementation]
        impl Provider for TestProvider {
            const url: &'static str = "http://test.com";

            fn get<R: Resource>(&self, resource: &R) -> R::Payload {
                resource.payload()
            }
        }

        #[resource_definition]
        struct TestResourceA {
            id: i32,
        }

        #[resource_implementation]
        impl Resource for TestResourceA {
            type Payload = ();

            fn payload(&self) -> Self::Payload {}
        }

        #[resource_definition(outputs = {id: i32})]
        struct TestResourceB {
            id: i32,
        }

        #[resource_implementation]
        impl Resource for TestResourceB {
            type Payload = ();

            fn payload(&self) -> Self::Payload {}

            fn set_outputs(&mut self) {
                self.set_id(420);
            }
        }
    }

    #[test]
    fn test_state() {
        use crate::test::test_provider::prelude::*;

        state! {

            #[resource(name = resource_a)]
            TestResourceA {
                id: resource_b.get_id(),
            }

            #[resource(name = resource_b)]
            TestResourceB {
                id: 10,
            }
        }

        let state = State::new(TestProvider);
        state.plan();
    }
}
