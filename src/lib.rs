pub trait Anamal {
    fn ask(&self) -> String;
    fn run(&self) -> f64;
}

pub struct Human {
    name: String,
    age: u32,
    leg: f64,
}
impl Human {
    pub fn new(name: String, age: u32) -> Human {
        Human {
            name,
            age,
            leg: 0.8,
        }
    }
}

impl Anamal for Human {
    fn ask(&self) -> String {
        format!("I am {}, I am {} years old.", self.name, self.age)
    }
    fn run(&self) -> f64 {
        self.leg * 2.0
    }
}

pub struct Cat {
    kind: String,
    leg: f64,
}

impl Cat {
    pub fn new(kind: String) -> Cat {
        Cat { kind, leg: 0.2 }
    }
}

impl Anamal for Cat {
    fn ask(&self) -> String {
        format!("miao ~ miao ~ {}", self.kind)
    }
    fn run(&self) -> f64 {
        self.leg * 4.0
    }
}
