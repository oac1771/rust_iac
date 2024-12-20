use provider_macro::provider;

struct ResourceA {
    id_a: String,
}
struct ResourceB {
    id_b: String,
}

#[provider]
struct FooProvider {
    resourceA: ResourceA,
    resourceB: ResourceB,
}
