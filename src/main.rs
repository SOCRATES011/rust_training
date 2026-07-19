// SESSION 003
// Methods: impl block
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    //method: takes &self (immutable borrow)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method: takes &mut self
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    //associated function (no self) - like a static method
    fn square(size: u32) -> Rectangle {
        Rectangle { 
            width: size, 
            height: size,
         }
    }
}

fn main(){
    let mut rect = Rectangle {width: 30, height: 50 };
    println!("Area: {}", rect.area());
    rect.double_size();
    println!("Doubled: {:#?}", rect); // pretty debug print, requires #[derive(Debug)]

    let sq = Rectangle::square(20);
    println!("Square: {:?}", sq);
}