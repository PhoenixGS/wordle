use console;
use std::mem;
use rand::Rng;
use std::{io::{self, Write}, vec, mem::swap};

mod builtin_words;

struct Stats
{
    count: Vec<i32>,
    list: Vec<usize>,
}

fn min(x: i32, y: i32) -> i32
{
    if x < y
    {
        return x;
    }
    else
    {
        return y;
    }
}

fn max(x: i32, y: i32) -> i32
{
    if x > y
    {
        return x;
    }
    else
    {
        return y;
    }
}

impl Stats
{
    fn get_id(guess: &String) -> usize
    {
        for i in 0..builtin_words::ACCEPTABLE.len()
        {
            if builtin_words::ACCEPTABLE[i].to_string().to_ascii_uppercase() == (*guess)
            {
                return i;
            }
        }
        panic!("ERROR");
    }

    fn update(&mut self, guess: &String) -> ()
    {
        let index = Stats::get_id(guess);
        self.count[index] += 1;
        let mut now: usize = usize::MAX;
        for i in 0..self.list.len()
        {
            if self.list[i] == index
            {
                now = i;
                break;
            }
        }
        if now == usize::MAX
        {
            if self.list.len() < 6
            {
                self.list.push(index);
                now = self.list.len() - 1;
            }
            else
            {
                self.list[5] = index;
                now = 5;
            }
        }
        while now > 0
        {
            if (self.count[self.list[now]], -(self.list[now] as i32)) > (self.count[self.list[now - 1]], -(self.list[now - 1] as i32))
            {
                let t = self.list[now];
                self.list[now] = self.list[now - 1];
                self.list[now - 1] = t;
                now -= 1;
            }
            else
            {
                break;
            }
        }
    }

    fn print(&mut self) -> ()
    {
        for i in 0..min(self.list.len() as i32, 5) as usize
        {
            if i != 0
            {
                print!(" ");
            }
            print!("{} {}", builtin_words::ACCEPTABLE[self.list[i]].to_string().to_ascii_uppercase(), self.count[self.list[i]]);
        }
        println!("");
    }
}

fn print_c(st: String, c: char) -> () //Make the output colored
{
    match c
    {
        'G' => print!("{}", console::style(st).green()),
        'Y' => print!("{}", console::style(st).yellow()),
        'R' => print!("{}", console::style(st).red()),
        'X' => print!("{}", console::style(st).blue()),
        _ => ()
    }
}

fn check(guess: &String) -> bool //Determine whether the word is a valid word
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

fn check_diffcult(pre_delta: &Vec<i32>, delta: &Vec<i32>, pre_out: &Vec<char>, out: &Vec<char>) -> bool //Determine whether a word is a valid word in hard mode
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
    let mut stats = Stats{count: vec![0; builtin_words::ACCEPTABLE.len()], list: vec![]};

    let is_tty = atty::is(atty::Stream::Stdout);

    let mut word = String::new();

    //命令行参数
    let mut is_random = false;
    let mut is_word = false;
    let mut is_difficult = false;
    let mut is_stats = false;

    let mut game_cnt = 0;
    let mut success_cnt = 0;
    let mut success_try_cnt = 0;

    //Handle command line options
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
            is_random = true;
//            println!("{}", word);
        }
        if arg == "-D".to_string() || arg == "--difficult".to_string()
        {
            is_difficult = true;
        }
        if arg == "-t".to_string() || arg == "--stats".to_string()
        {
            is_stats = true;
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

    while true
    {
        game_cnt += 1;

        let mut word_vec = vec![0; 26];
        if is_random
        {
            //TODO 保证每次不重复
            let index = rand::thread_rng().gen_range(0..builtin_words::FINAL.len());
            word = String::from(builtin_words::FINAL[index]);
        }
        if ! is_random && ! is_word
        {
            word = String::new();
            io::stdin().read_line(&mut word)?;
        }
        word = word.trim().to_string().to_ascii_uppercase();
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

            if count > 0 && is_difficult && ! check_diffcult(&pre_delta, &delta, &res[count - 1], &out)
            {
                println!("INVALID");
                continue;
            }

            stats.update(&guess);

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
            success_cnt += 1;
            success_try_cnt += count;
        }
        else
        {
            println!("FAILED {}", word);
        }

        if is_stats
        {
            println!("{} {} {:.2}", success_cnt, game_cnt - success_cnt, success_try_cnt as f32 / max(1, success_cnt) as f32);
            stats.print();
        }

        if ! is_word
        {
            let mut next_game = String::new();
            if io::stdin().read_line(&mut next_game).unwrap() == 0 || next_game.trim().to_string() == "N".to_string()
            {
                break;
            }
        }
        else
        {
            break;
        }
    }

    Ok(())
}
