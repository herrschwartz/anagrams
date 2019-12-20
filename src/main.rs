use itertools::Itertools;
use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::time::SystemTime;
use rayon::prelude::*;

fn main() {
    let words = read_into_set("2of12.txt");
    let now = SystemTime::now();
    
    let word = "sample";
    let anagrams = (3 ..= word.len())
        .into_par_iter()
        .map(|x| {
            let mut anagrams = Vec::new();
            for j in word.bytes().permutations(x) {
                // cnt += 1;
                let wrd = str::from_utf8(&j).unwrap();
                if words.contains(wrd){
                    anagrams.push(wrd.to_string())
                }
            }
            return anagrams;
        });
    
    anagrams.for_each(|mut m| {
        m.sort_unstable();
        m.dedup();

        println!("{:?}\n", m)
    });
    
    //println!("{:?}", anagrams);
    // let anagrams = get_words("fantastic", words);
    //prettyprint_anagrams(anagrams);

    println!("{}", now.elapsed().unwrap().as_millis());
}

fn read_into_set(path: &str) -> HashSet<String>{
    let mut words = HashSet::new();
    let f = File::open(path).expect("failed opening");
    let f = BufReader::new(f);

    for line in f.lines() {
        words.insert(line.expect("failed to read line"));
    }
    return words;
}

fn get_words(word: &str, words: HashSet<String>, i: usize) -> Vec<String> {
    let mut anagrams = Vec::new();
    let mut cnt = 0;
    // for i in 3..word.len()+1 {
        for j in word.bytes().permutations(i) {
            cnt += 1;
            let wrd = str::from_utf8(&j).unwrap();
            if words.contains(wrd){
                anagrams.push(wrd.to_string())
            }
        }
    // }
    println!("{}",cnt);
    return anagrams;
}

fn prettyprint_anagrams(mut words: Vec<String>) {
    words.sort_unstable();
    words.dedup();
    words.sort_unstable_by(|a, b| a.len().cmp(&b.len()));

    let mut lastlen = 0;
    for w in words {
        if w.len() > lastlen{
            println!("\n");
        }
        print!("{} ", w);
        lastlen = w.len();
    }
    //Wprintln!("\n {}", words.len())
}