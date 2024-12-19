use state_macro::state;

#[state]
mod foo {

    #[resource(name = foo)]
    struct ResourceA {
        id_a: String,
    }

    #[resource(name = bar)]
    struct ResourceB {
        id_b: String,
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn foo() {}
}
