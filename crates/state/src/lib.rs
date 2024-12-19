use state_macro::state;

#[state]
mod foo {

    #[resource()]
    struct ResourceA {
        id_a: String,
    }

    #[resource()]
    struct ResourceB {
        id_b: String,
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn foo() {}
}
