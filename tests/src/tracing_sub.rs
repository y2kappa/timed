

pub mod foo {
    pub mod bar {
        pub mod baz {
            pub fn foobar() {
                use crate::Trace;
                Trace::begin("Tracing", "foobar");
                println!("Foobar");
                Trace::begin("Tracing", "foobar");
            }
        }
    }
}