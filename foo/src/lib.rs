pub fn foo() {
    #[cfg(feature = "a")]
    println!("a");
    #[cfg(feature = "b")]
    println!("b");
}
