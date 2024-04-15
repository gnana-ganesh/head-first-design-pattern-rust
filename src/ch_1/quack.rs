pub(crate) trait Quack {
    fn quack(&self);
}

pub(crate) struct QuackNormal;

impl Quack for QuackNormal {
    fn quack(&self) {
        println!("Quack!");
    }
}

pub(crate) struct Squeak;

impl Quack for Squeak {
    fn quack(&self) {
        println!("Squeak!");
    }
}

pub(crate) struct MuteQuack;

impl Quack for MuteQuack {
    fn quack(&self) {
        println!("<< Silence >>");
    }
}