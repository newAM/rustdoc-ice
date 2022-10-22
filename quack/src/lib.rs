#![feature(async_fn_in_trait)]

struct Oink {}

impl moo::Meow for Oink {
    async fn woof() {
        todo!()
    }
}
