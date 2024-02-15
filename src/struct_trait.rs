pub trait Animal {
    const NAME: &'static str;

    fn make_sound(&self) -> &'static str;
    
    fn say_name(&self) {
        println!("Iam a {}.", Self::NAME);
    }
 }

pub struct Dog {
    pub name: String,
}

impl Animal for Dog {
    const NAME: &'static str = "Dog";

    fn make_sound(&self) -> &'static str {
        "Woof!"
    }
}
