use itertools::Itertools;
use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use std::time::SystemTime;
use rayon::prelude::*;
use std::iter::FromIterator;

fn main() {
    //using http://wordlist.aspell.net/12dicts/
    let words = read_into_set("2of12.txt");
    let words_dict = read_into_dict("words_encoded.txt");
    let mut now = SystemTime::now();
    
    let word = "bowling";
    //The slow algorithm
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
    
    println!("{}", now.elapsed().unwrap().as_millis());
    now = SystemTime::now();

    //the better stronger faster algorithm
    let anagrams_faster = (3 .. word.len())
        .into_par_iter()
        .map(|x| {
            let mut anagrams = Vec::new();
            for mut j in word.bytes().combinations(x) {
                j.sort_unstable();
                let wrd = words_dict.get(&String::from_utf8(j).unwrap());
                if let Some(m) = wrd {
                     anagrams.extend(m)
                    //None => println!("failed to find"),
                }
            }
            return anagrams;
        });
        let mut full: Vec<char> = word.chars().collect();
        full.sort_unstable();

        anagrams_faster.for_each(|mut m| {
            m.sort_unstable();
            m.dedup();

            println!("{:?}\n", m)
        });
        println!("{:?}", words_dict.get(&String::from_iter(full)).unwrap_or(&[].to_vec()));
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

fn read_into_dict(path: &str) -> HashMap<String, Vec<String>> {
    let mut words = HashMap::new();
    let f = File::open(path).expect("failed opening");
    let f = BufReader::new(f);

    let mut wrds: Vec<String> = Vec::new();
    let mut keys: Vec<String> = Vec::new();
    for line in f.lines() {
        let wr = line.unwrap();
        let w: Vec<&str> = wr.split(';').collect();
        keys.push(w[1].to_owned());
        wrds.push(w[0].to_owned());
    }

    for key in keys {
        words.insert(key.to_string(), Vec::new());
    }
    for word in wrds {
        let mut word_sorted: Vec<char> = word.chars().collect();
        word_sorted.sort_unstable();
        if let Some(c) = words.get_mut(&String::from_iter(word_sorted)){
            c.push(word);
        }
    }
    words
}