use console;
use rand::{SeedableRng, seq::SliceRandom};
use std::{io::{self, Write}, vec};
use std::fs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use serde_json::Value;

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

struct Help
{
    list: Vec<String>,
    ans_list: Vec<String>,
}

impl Help
{
    fn update(&mut self, out: &Vec<char>, guess: &String) -> ()
    {
        let tar = &guess.to_ascii_lowercase();

        let mut new_list = vec![];
        for st in &self.list
        {
            let mut is_ok = true;
            let mut cnt = vec![0; 26];
            let mut not = vec![false; 26];
            for i in 0..5
            {
                if out[i] == 'G'
                {
                    if st.chars().nth(i).unwrap() != tar.chars().nth(i).unwrap()
                    {
                        is_ok = false;
                    }
                }
                else
                {
                    if st.chars().nth(i).unwrap() == tar.chars().nth(i).unwrap()
                    {
                        is_ok = false;
                    }
                    cnt[st.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
                }
                if out[i] == 'Y'
                {
                    cnt[tar.chars().nth(i).unwrap() as usize - 'a' as usize] -= 1;
                }
                if out[i] == 'R'
                {
                    not[tar.chars().nth(i).unwrap() as usize - 'a' as usize] = true;
                }
            }
            for i in 0..26
            {
                if not[i]
                {
                    if cnt[i] != 0
                    {
                        is_ok = false;
                    }
                }
                else
                {
                    if cnt[i] < 0
                    {
                        is_ok = false;
                    }
                }
            }
            if is_ok
            {
                new_list.push((*st).clone());
            }
        }
        self.list = new_list.clone();

        new_list = vec![];
        for st in &self.ans_list
        {
            let mut is_ok = true;
            let mut cnt = vec![0; 26];
            let mut not = vec![false; 26];
            for i in 0..5
            {
                if out[i] == 'G'
                {
                    if st.chars().nth(i).unwrap() != tar.chars().nth(i).unwrap()
                    {
                        is_ok = false;
                    }
                }
                else
                {
                    if st.chars().nth(i).unwrap() == tar.chars().nth(i).unwrap()
                    {
                        is_ok = false;
                    }
                    cnt[st.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
                }
                if out[i] == 'Y'
                {
                    cnt[tar.chars().nth(i).unwrap() as usize - 'a' as usize] -= 1;
                }
                if out[i] == 'R'
                {
                    not[tar.chars().nth(i).unwrap() as usize - 'a' as usize] = true;
                }
            }
            for i in 0..26
            {
                if not[i]
                {
                    if cnt[i] != 0
                    {
                        is_ok = false;
                    }
                }
                else
                {
                    if cnt[i] < 0
                    {
                        is_ok = false;
                    }
                }
            }
            if is_ok
            {
                new_list.push((*st).clone());
            }
        }
        self.ans_list = new_list.clone();
    }
}

struct Recomm
{
    map: Vec<(f64, String)>,
}

impl Recomm
{
    fn calc(answer: &String, guess: &String) -> usize
    {
        let mut res:Vec<i32> = vec![-1; 5];
        let mut delta = vec![0; 26];
        for i in 0..5
        {
            delta[answer.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
        }
        for i in 0..5
        {
            if answer.chars().nth(i).unwrap() == guess.chars().nth(i).unwrap()
            {
                res[i] = 0;
                delta[guess.chars().nth(i).unwrap() as usize - 'a' as usize] -= 1;
            }
        }
        for i in 0..5
        {
            if res[i] != 0
            {
                let index = guess.chars().nth(i).unwrap() as usize - 'a' as usize;
                if delta[index] > 0
                {
                    delta[index] -= 1;
                    res[i] = 1;
                }
                else
                {
                    res[i] = 2;
                }
            }
        }
        let mut ret: usize = 0;
        for i in 0..5
        {
            ret = ret * 3 + res[i] as usize;
        }
        return ret;
    }

    fn give_words(&mut self, list: &Vec<String>, ans_list: &Vec<String>) -> Vec<(String, f64)>
    {
        self.map = vec![];
        for i in 0..(*list).len()
        {
            let s1 = &list[i];
            let mut count = vec![0; 243];
            let mut sum = 0;
            for s2 in (*ans_list).clone()
            {
                count[Recomm::calc(&s2, s1)] += 1;
                sum += 1;
            }
            let mut key = 0.0;
            for j in 0..243
            {
                let pr = count[j] as f64 / sum as f64;
                if count[j] != 0
                {
                    key += pr * -pr.log2();
                }
            }
            self.map.push((-key, (*s1).clone()));
        }
        self.map.sort_by_cached_key(|k| ((*k).0 * 1000000.0) as i32);
        let mut res = vec![];
        for i in 0..min(self.map.len() as i32, 5) as usize
        {
            res.push((self.map[i].1.clone(), -self.map[i].0));
        }
        return res.clone();
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
        if (*s).to_string().to_ascii_uppercase() == (*guess)
        {
            return true;
        }
    }
    return false;
}

fn test(gs: &String, dict: &Dict) -> f64
{
    let mut guess = (*gs).clone();

    let mut count = 0;

    for answer in &dict.FINAL
    {
        //println!("{}", answer);
        let mut help = Help{list: dict.ACCEPTABLE.clone(), ans_list: dict.FINAL.clone()};
        let mut recomm = Recomm{map: vec![]};
    
        while true
        {
            let mut res = Recomm::calc(answer, &guess);

            count += 1;
    
            if res == 0
            {
                break;
            }

            let mut out: Vec<char> = vec!['X'; 5];
            for i in 0..5
            {
                if res % 3 == 0
                {
                    out[4 - i] = 'G';
                }
                if res % 3 == 1
                {
                    out[4 - i] = 'Y';
                }
                if res % 3 == 2
                {
                    out[4 - i] = 'R';
                }
                res = res / 3;
            }

            help.update(&out, &guess);
    
            let nex = recomm.give_words(&help.list, &help.ans_list);
            let st = nex[0].0.clone();
            guess = st.clone();
        }
    }
    return count as f64 / dict.FINAL.len() as f64;
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let mut dict = Dict{is_final: false, is_acceptable: false, FINAL: vec![], ACCEPTABLE: vec![], now_final: 0, now_acceptable: 0};
    dict.init();

    let is_tty = atty::is(atty::Stream::Stdout);

    let mut word = String::new();
    
    let mut is_test = false;
    let mut is_stat = false;

    for arg in std::env::args()
    {
        if arg == "-t".to_string() || arg == "--test".to_string()
        {
            is_test = true;
        }
        if arg == "-s".to_string() || arg == "--stat".to_string()
        {
            is_stat = true;
        }
    }

    if is_stat
    {
        let mut sum = 0.0;
        for guess in &dict.ACCEPTABLE
        {
            let aver = test(guess, &dict);
            println!("{} {}", guess, aver);
            sum += aver;
        }
        sum /= dict.ACCEPTABLE.len() as f64;
        println!("All Average: {}", sum);
        return Ok(());
    }

    let mut guess = String::new();
    print!("{}", console::style("Pick a word:").green());
    io::stdout().flush().unwrap();

    guess = String::new();
    io::stdin().read_line(&mut guess);
    guess = guess.trim().to_string().to_ascii_lowercase();
    
    if is_test
    {
        println!("Average: {}", test(&guess, &dict));
        return Ok(());
    }

    let mut help = Help{list: dict.ACCEPTABLE.clone(), ans_list: dict.FINAL.clone()};
    let mut recomm = Recomm{map: vec![]};

    let mut count = 0;

    while true
    {
        let mut output = String::new();

        io::stdin().read_line(&mut output);
        output = output.trim().to_string().to_ascii_uppercase();

        count += 1;

        if output == "GGGGG"
        {
            break;
        }

        let mut out: Vec<char> = vec![];
        for i in 0..5
        {
            out.push(output.chars().nth(i).unwrap());
        }
        help.update(&out, &guess);

        let nex = recomm.give_words(&help.list, &help.ans_list);
        let st = nex[0].0.clone();
        println!("{}", st);
        if help.list.len() == 1
        {
            break;
        }
        guess = st.clone();
    }

    println!("Congratulation! You have just tried {} time(s)!", count);

    Ok(())
}
