use console;
use rand::Rng;
use std::{io::{self, Write}, vec};

mod builtin_words;

fn print_c(st: String, c: char) -> ()
{
    match c
    {
        'G' => print!("{}", console::style(st).green()),
        'Y' => print!("{}", console::style(st).yellow()),
        'R' => print!("{}", console::style(st).red()),
        'X' => print!("{}", console::style(st).red()),
        _ => ()
    }
}

fn check(guess: &String) -> bool
{
    for s in builtin_words::ACCEPTABLE
    {
//        println!("{} {}", (*s).to_string().to_ascii_uppercase(), (*guess));
        if (*s).to_string().to_ascii_uppercase() == (*guess)
        {
            return true;
        }
    }
    return false;
}

fn check_diffcult(guess: &String, pre_delta: &Vec<i32>, delta: &Vec<i32>, pre_out: &Vec<char>, out: &Vec<char>) -> bool
{
    for i in 0..5
    {
        if pre_out[i] == 'G' && out[i] != 'G'
        {
            return false;
        }
    }
    for i in 0..26
    {
        if delta[i] > pre_delta[i]
        {
            return false;
        }
    }
    return true;
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let is_tty = atty::is(atty::Stream::Stdout);

    let mut word = String::new();

    let mut is_random = false;
    let mut is_word = false;
    let mut is_difficult = false;

    let mut pre: String = "".to_string();
    for arg in std::env::args()
    {
        if pre == "-w".to_string() || pre == "--word".to_string()
        {
            word = String::from(&arg);
            is_word = true;
        }
        if arg == "-r".to_string() || arg == "--random".to_string()
        {
            let index = rand::thread_rng().gen_range(0..builtin_words::FINAL.len());
            word = String::from(builtin_words::FINAL[index]);
            is_random = true;
//            println!("{}", word);
        }
        if arg == "-D".to_string() || arg == "--difficult".to_string()
        {
            is_difficult = true;
        }
        pre = String::from(&arg);
    }
    if is_tty
    {
        println!(
            "I am in a tty. Please print {}!",
            console::style("colorful characters").bold().blink().blue()
        );
    }
    else
    {
//        println!("I am not in a tty. Please print according to test requirements!");
    }

    if is_tty
    {
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        println!("Welcome to wordle, {}!", line.trim());
    }

    let mut word_vec = vec![0; 26];
    if ! is_random && ! is_word
    {
        io::stdin().read_line(&mut word)?;
    }
    word = word.trim().to_string().to_ascii_uppercase();
//    println!("{}", word);
    for i in 0..5
    {
        let c = word.chars().nth(i).unwrap();
        word_vec[c as usize - 'A' as usize] += 1;
    }
//    println!("{:?}", word_vec);

    let mut status = vec!['X'; 26];
    let mut count = 0;
    let mut success = false;
    let mut res = vec![];
    let mut words = vec![];
    let mut pre_delta = vec![];
    while count < 6
    {
        let mut guess = String::new();
        let mut guess_vec = vec![0; 26];
        io::stdin().read_line(&mut guess)?;
        guess = guess.trim().to_string().to_ascii_uppercase();
        if ! check(&guess)
        {
            println!("INVALID");
            continue;
        }
        for i in 0..5
        {
            let c = guess.chars().nth(i).unwrap();
            guess_vec[c as usize - 'A' as usize] += 1;
        }
        let mut delta = vec![0; 26];
        for i in 0..26
        {
            delta[i] = word_vec[i];
        }
        let mut out = vec!['S'; 5];
        let mut cnt = 0;
        let mut new_status = status.clone();
        for i in 0..5
        {
            if word.chars().nth(i) == guess.chars().nth(i)
            {
                out[i] = 'G';
                let index = guess.chars().nth(i).unwrap() as usize - 'A' as usize;
                delta[index] -= 1;
                cnt += 1;
                new_status[index] = 'G';
            }
        }
        for i in 0..5
        {
            if out[i] != 'G'
            {
                let index = guess.chars().nth(i).unwrap() as usize - 'A' as usize;
                if delta[index] > 0
                {
                    delta[index] -= 1;
                    out[i] = 'Y';
                    if new_status[index] != 'G'
                    {
                        new_status[index] = 'Y';
                    }
                }
                else
                {
                    out[i] = 'R';
                    if new_status[index] != 'G' && new_status[index] != 'Y'
                    {
                        new_status[index] = 'R';
                    }
                }
            }
        }

        if count > 0 && is_difficult && ! check_diffcult(&guess, &pre_delta, &delta, &res[count - 1], &out)
        {
            println!("INVALID");
            continue;
        }

        count += 1;
        status = new_status.clone();

        res.push(out);
        words.push(guess);
        if ! is_tty
        {
            for i in 0..5
            {
                print!("{}", res[count - 1][i]);
            }
            print!(" ");
            for i in 0..26
            {
                print!("{}", status[i]);
            }
            println!("");
        }
        else
        {
            for i in 0..count
            {
                for j in 0..5
                {
                    print_c(words[i][j..j + 1].to_string(), res[i][j]);
                }
                println!("");
            }
            for i in 0..26
            {
                print_c(((i as u8 + 65) as char).to_string(), status[i]);
            }
            println!("");
        }
        if cnt == 5
        {
            success = true;
            break;
        }
        pre_delta = delta.clone();
    }

    if success
    {
        println!("CORRECT {}", count);
    }
    else
    {
        println!("FAILED {}", word);
    }

    // example: print arguments
/*    print!("Command line arguments: ");
    for arg in std::env::args()
    {
        print!("{} ", arg);
    }
    println!("");*/
    // TODO: parse the arguments in `args`

    Ok(())
}
