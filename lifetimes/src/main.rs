fn main() {
    // b goes out of scope before a does, aka b's lifetime does not  contain a's lifetime.
    // let a;
    // {
    //     let b = 2;
    //     a = &b;
    // }
    //
    // println!("{}", a);
    let a;
    {
        let b = 2;
        a = b;
    }
    println!("{}", a);
}
