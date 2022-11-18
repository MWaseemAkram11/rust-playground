pub fn Exm_p1(){
      // Fixed-size array (type signature is superfluous)
      let xs: [i32; 5] = [1, 2, 3, 4, 5];

      // All elements can be initialized to the same value
      let ys: [i32; 500] = [0; 500];
  
      // Indexing starts at 0
      println!("first element of the array: {}", xs[0]);
      println!("second element of the array: {}", xs[1]);

      // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());


    for i in 0..xs.len() + 1 { // OOPS, one element too far
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}