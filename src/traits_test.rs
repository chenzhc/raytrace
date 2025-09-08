#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use log::info;


struct Dog {

}

struct Antelope {

}

struct Bear {

}

pub trait AnimalEating {
    fn eat_food(&self);
}

pub trait AnimalSound {
    fn make_sound(&self);
}

pub trait Animal : AnimalEating + AnimalSound {
    
}

impl AnimalEating for Dog {
    fn eat_food(&self) {
        info!("Dog is eating dog food");
    }
}

impl AnimalEating for Antelope {   
    fn eat_food(&self) {
        info!("Antelope is eating natural desert plants");
    }
    
}

impl AnimalEating for Bear {   
    fn eat_food(&self) {
        info!("Bear is eating some other animal");
    }
    
}

impl AnimalSound for Dog {
    fn make_sound(&self) {
        info!("Dog is barking!");
    }
}

impl Animal for Dog {

}

impl AnimalSound for Antelope {
    fn make_sound(&self) {
        info!("Antelope is bleating!");
    }
}

impl Animal for Antelope {

}

impl AnimalSound for Bear {
    fn make_sound(&self) {
        info!("Bear is raring!");
    }
}

impl Animal for Bear {

}

pub fn run_traits_test() {
    let dog01 = Dog { };
    let antelope01 = Antelope { };
    make_some_nose(&dog01);
    make_some_nose(&antelope01);

    eat_some_food(&dog01);
    eat_some_food(&antelope01);

    let animal01 = get_animal();
    animal01.make_sound();
    animal01.eat_food();

}

fn make_some_nose(a: &dyn Animal) {
    a.make_sound();
}

fn eat_some_food(a: &dyn Animal) {
    a.eat_food();
}

fn get_animal() -> Box<dyn Animal> {
    let animal = Antelope {};

    return Box::from(animal);
}

// fn make_some_nose<Animal: AnimalSound> (a: Animal) {
//     a.make_sound();
// }

#[cfg(test)]
mod tests {
    use log::info;
    use super::*;

    #[test]
    fn it_test01() {
        crate::init();
        info!("test");

        run_traits_test();

    }
}