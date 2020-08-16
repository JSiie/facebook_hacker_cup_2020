use std::io::{self};
use std::collections::BTreeMap;
use std::ops::Bound::{Included, Excluded};
use std::cmp;

fn readline() -> String{
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => return input,
        Err(_error) => {
            println!("error: {}", _error);
            return String::new()
        },
    }
}

fn readint() -> i32 {
    let input = readline();
    let i = match input
        .trim_end()
        .parse::<i32>() {
            Ok(_i) => _i,
            Err(_error) => {
                println!("error: {}", _error);
                -1
            }
        };
    i
}

fn readints() -> Vec<i32> {
    let input = readline();
    let numbers: Vec<i32> = input
    .split_whitespace()
    .map(|s| s.parse().expect("parse error"))
    .collect();
    numbers
}

fn calculatePerimetric(N: usize, K: usize, mut Li: Vec<i32>, ABCDl: Vec<i32>, mut Wi: Vec<i32>, ABCDw: Vec<i32>, mut Hi: Vec<i32>, ABCDh: Vec<i32> ) -> i32 {
    let mut result: i32 = 1;
    let mut Pi: Vec<i32> = Vec::new();
    let mut Psum: i64 = 0;
    let mut segments: BTreeMap<i32, i32> = BTreeMap::new();
    //first we complete Li, Wi and Hi vec
    for i in K..N {
        let L: i32 = ((ABCDl[0] as i64*Li[i-2] as i64 + ABCDl[1] as i64*Li[i-1] as i64+ ABCDl[2] as i64) % ABCDl[3] as i64) as i32 + 1;
        Li.push(L);
        let W: i32 = ((ABCDw[0] as i64*Wi[i-2] as i64 + ABCDw[1] as i64*Wi[i-1] as i64+ ABCDw[2] as i64) % ABCDw[3] as i64) as i32 + 1;
        Wi.push(W);
        let H: i32 = ((ABCDh[0] as i64*Hi[i-2] as i64 + ABCDh[1] as i64*Hi[i-1] as i64+ ABCDh[2] as i64) % ABCDh[3] as i64) as i32 + 1;
        Hi.push(H);
    }
    //now we will create the map, starting by lowest Li
    for i in 0..N {
        let mut Wj = Wi[i];
        let (mut Hg, mut Hd, mut counter): (i32, i32, i32) = (Hi[i], Hi[i], 0);
        let mut P: i64 = 0;
        //we look for overlapped segment
        let mut listofkeys: Vec<i32> = Vec::new();
        //then following segments included
        let mut firstvalue: i32 = Li[i];
        let mut lastvalue: i32 = Li[i] + Wi[i];
        //first previous segment
        for (&key, &value) in segments.range((Included(0), Excluded(Li[i]))).rev(){
            if(value >= Li[i]){
                counter = counter + 1;
                Hg = 0;
                Hd = 0;
                if(value > Li[i] + Wi[i]){
                    lastvalue = value;
                }
                firstvalue = key;
                listofkeys.push(key);
                Wj = Wj - (cmp::min(value, Li[i] + Wi[i]) - Li[i]);
            }
            break;
        }
        for (&key, &value) in segments.range((Included(Li[i]), Included(Li[i] + Wi[i]))){
            counter = counter + 1;
            Hg = 0;
            Hd = 0;
            Wj = Wj - (cmp::min(value, Li[i] + Wi[i]) - key);
            if(value > lastvalue){
                lastvalue = value;
            }
            listofkeys.push(key);
        }
        //we manage segments
        for i in &listofkeys{
            segments.remove(i);
        }
        
        segments.insert(firstvalue, lastvalue);

        if(Wj < 0){
            Wj = 0;
        }
        let mut Htoremove: i64 = 0;
        if(counter > 1){
            Htoremove = (counter-1) as i64*2 as i64*Hi[i] as i64;
        }
        P = 2*Wj as i64 + Hg as i64 + Hd as i64 - Htoremove;
        //println!("segment p1 {} p2 {} Wj {} Hg {} Hd {} counter*Li {} P {} Psum {}", firstvalue, lastvalue, Wj, Hg, Hd, Htoremove, P, Psum + P);
        result = ((result as i64 * ((P + Psum) % 1000000007) ) as i64 % 1000000007 ) as i32;
        Psum = Psum + P;
    }
    result
}

fn main(){
    let T = readint();
    for i in 0..T {
        let NK : Vec<i32> = readints();
        let N : usize = NK[0] as usize;
        let K : usize = NK[1] as usize;
        let mut Li : Vec<i32> = readints();
        let ABCDl : Vec<i32> = readints();
        let mut Wi : Vec<i32> = readints();
        let ABCDw : Vec<i32> = readints();
        let mut Hi : Vec<i32> = readints();
        let ABCDh : Vec<i32> = readints();
        println!("Case #{}: {}", i+1, calculatePerimetric(N, K, Li, ABCDl, Wi, ABCDw, Hi, ABCDh));
    }
}