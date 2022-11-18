pub fn tupple(){
    let tup: (i32, f64, u8) = (500, 64.0, 1);
    println!("tuple data type ---->,{:?}",tup);

    let five_hundred = tup.0;
    println!("five-hundred is: {five_hundred}");

    let six_point_four = tup.1;
    println!("floating is: {six_point_four}");

    let one = tup.2;
    println!("one is: {one}");

    let tuppp = (500, 6.4, 1);

    let (x, y, z) = tuppp;

    println!("The value of y is: {y}");
}