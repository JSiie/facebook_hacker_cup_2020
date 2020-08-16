use std::io::{self};

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

fn calculatePerimetric(N: usize, K: usize, W: i32, mut Li: Vec<i32>, ABCDl: Vec<i32>, mut Hi: Vec<i32>, ABCDh: Vec<i32> ) -> i32 {
    let mut result: i32 = 1;
    let mut Pi: Vec<i32> = Vec::new();
    let mut Psum: i64 = 0;
    //first we complete Li and Hi vec
    for i in K..N {
        let L: i32 = ((ABCDl[0] as i64*Li[i-2] as i64 + ABCDl[1] as i64*Li[i-1] as i64+ ABCDl[2] as i64) % ABCDl[3] as i64) as i32 + 1;
        Li.push(L);
        let H: i32 = ((ABCDh[0] as i64*Hi[i-2] as i64 + ABCDh[1] as i64*Hi[i-1] as i64+ ABCDh[2] as i64) % ABCDh[3] as i64) as i32 + 1;
        Hi.push(H);
    }
    //now we will create the map, starting by lowest Li
    for i in 0..N {
        let mut Hj = Hi[i];
        let mut Wj = W;
        let mut P: i64 = 0;
        //we look for overlapped rooms
        for j in (0..i).rev(){
            if(Li[j] + W < Li[i]){
                break;
            }
            if(Li[i] - Li[j] < Wj){
                Wj = Li[i] - Li[j]; 
            }
            if(Hi[j] < Hi[i] && Hi[i] - Hi[j] < Hj){
                Hj = Hi[i] - Hi[j];  
            }
            else if(Hi[j] >= Hi[i]){
                Hj = 0;  
            }
        }
        if(Hj > 0){
            P = Hj as i64*2 + 2*Wj as i64;
        }
        else{
            P = 2*Wj as i64;
        }
        result = ((result as i64 * ((P + Psum) % 1000000007) ) as i64 % 1000000007 ) as i32;
        Psum = Psum + P;
    }
    result
}

fn main(){
    let T = readint();
    for i in 0..T {
        let NKW : Vec<i32> = readints();
        let N : usize = NKW[0] as usize;
        let K : usize = NKW[1] as usize;
        let W : i32 = NKW[2];
        let mut Li : Vec<i32> = readints();
        let ABCDl : Vec<i32> = readints();
        let mut Hi : Vec<i32> = readints();
        let ABCDh : Vec<i32> = readints();
        println!("Case #{}: {}", i+1, calculatePerimetric(N, K, W, Li, ABCDl, Hi, ABCDh));
    }
}