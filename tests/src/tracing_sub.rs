
pub mod foo {
    pub mod bar {
        pub mod baz {
            use timed::timed;
            #[timed(tracing=true)]
            pub fn foobar() {
                println!("Foobar");
            }
        }
    }
}