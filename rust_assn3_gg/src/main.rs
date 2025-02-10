fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        return 0;
    }
    else if guess > secret{
        return 1;
    }
    else{
        return -1;
    }
}

fn main(){
    let mut secret = 7;
    let mut guess_count = 0;

    loop {

        guess_count += 1;
        let guess = 2 + guess_count;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! The secret number is {}. It took you {} guesses.", secret, guess_count);
            break;
        } else if result == 1 {
            println!("Guess {}: Too high!", guess);
        } else {
            println!("Guess {}: Too low!", guess);
        }
    }


}