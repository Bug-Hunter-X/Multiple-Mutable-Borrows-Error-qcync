fn main() {
    let mut x = 5;
    { //creating a block scope
        let y = &mut x; 
        *y += 1;
    }
    { //creating a block scope
        let z = &mut x; 
        *z += 1;
    }
    println!("x = {}", x);
}