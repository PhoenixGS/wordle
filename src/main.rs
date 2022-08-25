use console;
use std::{mem, f32::consts::E};
use rand::{Rng, SeedableRng, seq::SliceRandom};
use std::{io::{self, Write}, vec, mem::swap};
use std::io::prelude::*;
use std::fs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use serde_json::{json, Value};
use serde_json::from_str;
use valico::json_schema;

mod builtin_words;


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

struct Dict
{
    is_final: bool,
    is_acceptable: bool,
    FINAL: Vec<String>,
    ACCEPTABLE: Vec<String>,
    now_final: usize,
    now_acceptable: usize,
}

impl Dict
{
    fn check(st: &String, now: &mut usize) -> bool //Determine whether the word is a valid word
    {
        while now < &mut builtin_words::ACCEPTABLE.len()
        {
            let index: usize = *now;
            if builtin_words::ACCEPTABLE[index].to_string() == *st
            {
                return true;
            }
            *now += 1;
        }
        return false;
    }

    fn init(&mut self) -> ()
    {
        for i in 0..builtin_words::FINAL.len()
        {
            self.FINAL.push(builtin_words::FINAL[i].to_string());
        }
        for i in 0..builtin_words::ACCEPTABLE.len()
        {
            self.ACCEPTABLE.push(builtin_words::ACCEPTABLE[i].to_string());
        }
        self.now_final = 0;
        self.now_acceptable = 0;
    }

    fn get_FINAL(&self) -> &[String]
    {
        return &self.FINAL;
    }

    fn get_ACCEPTABLE(&self) -> &[String]
    {
        return &self.ACCEPTABLE;
    }

    fn get_FINAL_len(&self) -> usize
    {
        return self.FINAL.len();
    }

    fn get_ACCEPTABLE_len(&self) -> usize
    {
        return self.ACCEPTABLE.len();
    }

    fn update_final(&mut self, file_name: &String) -> bool
    {
        self.FINAL = vec![];
        self.is_final = true;
        let text = fs::read_to_string(file_name).unwrap();
        for st in text.split('\n')
        {
            self.FINAL.push(st.to_string().trim().to_string().to_ascii_lowercase());
        }
        self.FINAL.sort();
        for i in 0..self.FINAL.len() - 1
        {
            if self.FINAL[i] == self.FINAL[i + 1]
            {
                return false;
            }
        }
        for st in &self.FINAL
        {
            if ! Dict::check(&st, &mut self.now_final)
            {
                return false;
            }
        }
        true
    }

    fn update_acceptable(&mut self, file_name: &String) -> bool
    {
        self.ACCEPTABLE = vec![];
        self.is_acceptable = true;
        let text = fs::read_to_string(file_name).unwrap();
        for st in text.split('\n')
        {
            self.ACCEPTABLE.push(st.to_string().trim().to_string().to_ascii_lowercase());
        }
        self.ACCEPTABLE.sort();
        for i in 0..self.ACCEPTABLE.len() - 1
        {
            if self.ACCEPTABLE[i] == self.ACCEPTABLE[i + 1]
            {
                return false;
            }
        }
        for st in &self.ACCEPTABLE
        {
            if ! Dict::check(&st, &mut self.now_acceptable)
            {
                return false;
            }
        }
        true
    }
}

struct Stats
{
    count: Vec<i32>,
    list: Vec<usize>,
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
//            println!("{}", builtin_words::ACCEPTABLE[i].to_string().to_ascii_uppercase());
        }
        println!("{}", guess);
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

//Save Part

#[derive(Serialize, Deserialize)]
struct Game
{
    answer: String,
    guesses: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct State
{
    total_rounds: usize,
    games: Vec<Game>,
}

fn read_json_typed(raw_json: &str) -> State {
    let parsed: State = serde_json::from_str(raw_json).unwrap();
    return parsed
    
}

//Make the output colored
fn print_c(st: String, c: char) -> ()
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

//Determine whether the word is a valid word
fn check(guess: &String, dict: &Dict) -> bool
{
    for s in dict.get_ACCEPTABLE()
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
    let mut dict = Dict{is_final: false, is_acceptable: false, FINAL: vec![], ACCEPTABLE: vec![], now_final: 0, now_acceptable: 0};
    dict.init();

    let is_tty = atty::is(atty::Stream::Stdout);

    let mut word = String::new();

    //命令行参数
    let mut is_random = false;
    let mut is_word = false;
    let mut is_difficult = false;
    let mut is_stats = false;
    let mut is_seed = false;
    let mut is_day = false;
    let mut is_final = false;
    let mut is_acceptable = false;
    let mut is_state = false;

    let mut success_cnt = 0;
    let mut success_try_cnt = 0;
    let mut seed = 0;
    let mut day = 0;
    let mut save = String::new();

    //Handle command line options
    let mut pre: String = "".to_string();
    for arg in std::env::args()
    {
        if pre == "-w".to_string() || pre == "--word".to_string()
        {
            word = String::from(&arg);
            is_word = true;
        }
        if pre == "-s".to_string() || pre == "--seed".to_string()
        {
            seed = arg.parse::<u64>().unwrap();
            is_seed = true;
        }
        if pre == "-d".to_string() || pre == "--day".to_string()
        {
            day = arg.parse::<usize>().unwrap();
            is_day = true;
            //TODO 不能超过答案词库的大小
        }
        if pre == "-f".to_string() || pre == "--final-set".to_string()
        {
            if ! dict.update_final(&arg)
            {
                panic!("Final Set Error");
            }
            is_final = true;
        }
        if pre == "-a".to_string() || pre == "--acceptable-set".to_string()
        {
            if ! dict.update_acceptable(&arg)
            {
                panic!("Acceptable Set Error");
            }
            is_acceptable = true;
        }
        if pre == "-S".to_string() || pre == "--state".to_string()
        {
            save = arg.to_string();
            is_state = true;
        }
        if arg == "-r".to_string() || arg == "--random".to_string()
        {
            is_random = true;
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

    //Stats
    let mut stats = Stats{count: vec![0; dict.get_ACCEPTABLE_len()], list: vec![]};

    //State
    let mut state: State = State{total_rounds: 0, games: vec![]};
    if is_state
    {
        let middle = fs::read_to_string(&save.as_str());
        
        match middle
        {
            Ok(json) =>
            {
                let v: Result<Value, serde_json::Error> = serde_json::from_str(&json.as_str());
                match v
                {
                    Ok(T) =>
                    {
                        let z = T["total_rounds"].to_string().parse::<i32>();
                        match z
                        {
                            Ok(rounds) =>
                            {
                                state.total_rounds = rounds as usize;
                                for i in 0..state.total_rounds
                                {
//                                    let str = T["games"][i]["answer"].as_str().unwrap();
//                                    println!("{} {}", str, T["games"][i]["answer"].to_string());
                                    state.games.push(Game{answer: T["games"][i]["answer"].as_str().unwrap().to_string(), guesses: vec![]});
                                    for st in T["games"][i]["guesses"].as_array().unwrap()
                                    {
                                        state.games[i].guesses.push(st.as_str().unwrap().to_string());
                                    }
                                }
                            },
                            _ => ()
                        }
                    },
                    Err(T) => return Err(Box::new(T)),
                }
//                state = read_json_typed(&json.as_str())
            },
            _ => ()
        }
    }
    for game in &state.games
    {
        if game.guesses[game.guesses.len() - 1] == game.answer
        {
            success_cnt += 1;
            success_try_cnt += game.guesses.len();
        }
        for st in &game.guesses
        {
            stats.update(&st.to_ascii_uppercase());
        }
    }

    //Check whether parameters conflict
    if is_word && (is_random || is_seed || is_day)
    {
        panic!("Parameters Error");
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

    //Rand
    let mut R = rand::rngs::StdRng::seed_from_u64(seed);
    let mut rand_list = vec![0; dict.get_FINAL_len()];
    for i in 0..dict.get_FINAL_len()
    {
        rand_list[i] = i;
    }
    rand_list.shuffle(&mut R);

    //Day
    let mut now_day;
    if is_day
    {
        now_day = day;
    }
    else
    {
        now_day = 1;
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
        state.total_rounds += 1;

        let mut word_vec = vec![0; 26];
        if is_random
        {
            let index = rand_list[now_day - 1];
            word = dict.get_FINAL()[index].clone();
        }
        if ! is_random && ! is_word
        {
            word = String::new();
            io::stdin().read_line(&mut word)?;
        }
        word = word.trim().to_string().to_ascii_uppercase();
        state.games.push(Game{answer: word.to_ascii_uppercase(), guesses: vec![]});
        for i in 0..5
        {
            let c = word.chars().nth(i).unwrap();
            word_vec[c as usize - 'A' as usize] += 1;
        }
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
            if ! check(&guess, &mut dict)
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
            words.push(guess.clone());
            state.games[state.total_rounds - 1].guesses.push(guess.to_ascii_uppercase());
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
            println!("{} {} {:.2}", success_cnt, state.total_rounds - success_cnt, success_try_cnt as f32 / max(1, success_cnt as i32) as f32);
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
        now_day += 1;
    }

    if is_state
    {
        let json = serde_json::to_string(&state).unwrap();
        let mut file = File::create(&save.as_str()).unwrap();
        file.write(json.as_bytes()).unwrap();
    }

    Ok(())
}
