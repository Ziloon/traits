use traits::Anamal;



fn main() {
    let bob = traits::Human::new(String::from("Bob"), 11);
    let gongzhu = traits::Cat::new(String::from("🐱"));
    println!("{}. Run {} m.", bob.ask(), bob.run());
    println!("{}. Run {} m.", gongzhu.ask(), gongzhu.run());
}
