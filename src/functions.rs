pub fn funn(){
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    /// arrow function -------------------------------
    fn five() -> i32 {
        5
    }
    
    fn printotherfunction() {
        let x = five();
        println!("The value of x is: {x}");
    }
    printotherfunction();
}