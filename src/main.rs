use traits::Anamal;
mod zmath;

fn main() {
    let bob = traits::Human::new(String::from("Bob"), 11);
    let gongzhu = traits::Cat::new(String::from("🐱"));
    println!("{}. Run {} m.", bob.ask(), bob.run());
    println!("{}. Run {} m.", gongzhu.ask(), gongzhu.run());

    let l = vec![0, 12, 32, 5435, 65, 5666];
    let _largNum = zmath::cmp::largest(&l);
}
