#![feature(async_fn_in_trait)]

pub trait Meow {
    /// Who's a good dog?
    async fn woof();
}
