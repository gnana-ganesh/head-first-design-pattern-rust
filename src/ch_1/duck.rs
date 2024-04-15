use std::fmt::{Display, Formatter};
use crate::ch_1::fly::Fly;
use crate::ch_1::quack::Quack;

struct Duck {
    fly_behavior: Box<dyn Fly>,
    quack_behavior: Box<dyn Quack>,
}

impl Duck {
    fn perform_fly(&self) {
        self.fly_behavior.fly();
    }

    fn perform_quack(&self) {
        self.quack_behavior.quack();
    }

    fn swim(&self) {
        println!("All ducks float, even decoys!");
    }

    fn set_fly_behavior(&mut self, fly_behavior: Box<dyn Fly>) {
        self.fly_behavior = fly_behavior;
    }

    fn set_quack_behavior(&mut self, quack_behavior: Box<dyn Quack>) {
        self.quack_behavior = quack_behavior;
    }
}


impl Display for Duck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "I'm a duck!")
    }
}


#[cfg(test)]
mod tests {
    use crate::ch_1::fly::{FlyRocketPowered, FlyWithWings, NoFly};
    use crate::ch_1::quack::{MuteQuack, QuackNormal};
    use super::*;

    #[test]
    fn test_duck() {
        let mallard = Duck {
            fly_behavior: Box::new(FlyWithWings),
            quack_behavior: Box::new(QuackNormal),
        };
        mallard.perform_fly();
        mallard.perform_quack();
        mallard.swim();
    }

    #[test]
    fn test_model_duck() {
        let mut model = Duck {
            fly_behavior: Box::new(NoFly),
            quack_behavior: Box::new(MuteQuack),
        };
        model.perform_fly();
        model.perform_quack();
        model.set_fly_behavior(Box::new(FlyRocketPowered));
        model.perform_fly();
    }
}
