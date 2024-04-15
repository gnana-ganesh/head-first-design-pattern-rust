

pub(crate) trait Fly {
    fn fly(&self);
}

pub(crate) struct FlyWithWings;

impl Fly for FlyWithWings {
    fn fly(&self) {
        println!("I'm flying!!");
    }
}

pub(crate) struct NoFly;

impl Fly for NoFly {
    fn fly(&self) {
        println!("I can't fly");
    }
}

pub(crate) struct FlyRocketPowered;

impl Fly for FlyRocketPowered {
    fn fly(&self) {
        println!("I'm flying with a rocket!");
    }
}
