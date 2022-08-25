use console;
use rand::{SeedableRng, seq::SliceRandom};
use std::{io::{self, Write}, vec};
use std::fs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use serde_json::Value;

mod builtin_words;

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

fn main()
{
    let mut file = File::create("data.rs").unwrap();
    for i in 0..builtin_words::ACCEPTABLE.len()
    {
        file.write(b"[").unwrap();
        for j in 0..builtin_words::ACCEPTABLE.len()
        {
            if j != 0
            {
                file.write(b",").unwrap();
            }
            file.write(calc(&builtin_words::ACCEPTABLE[j].to_string(), &builtin_words::ACCEPTABLE[i].to_string()).to_string().as_bytes()).unwrap();
        }
        file.write(b"],\n").unwrap();
    }
}