use provider_macro::provider;

#[provider]
mod provider {
    struct ResourceA {
        id_a: String,
    }
    struct ResourceB {
        id_b: String,
    }
}
