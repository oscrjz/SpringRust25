

fn main() {

   
    let x =5;
    let rem = x%3;
    let phrase = match rem {
        0 => "Remainder is Zero",
        1 => "Remainder is 1",
        2 => {
            println!("This was an amazing choice!");
            2
            // "Remainder is Two"
        },
        _ => "#", //println!(),
    };

    println!("{}", phrase);
}

    //let x = true;
   // match x {
      //  true => !("True "),
    //    false => !("False",)
    //}
