use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = vec![25, 12, 47, 28, 57, 12, 29];  //odd
    // let mut v: Vec<i32> = vec![25, 12, 47, 28];  //even
    v.sort(); //sort needed for median fn.

    //mean (average value)
    println!("mean {}", mean(&v));  //odd: 33.8, even: 28

    //median (when sorted value in middle position) 25 + 28 = 26.5
    println!("median {}", median(&v)); //odd: 28, even: 26.5

    //mode
    println!("mode {:?}", mode(&v));

    // test vector
    println!("v {:?}", v);
}

// functions
fn mean(v: &Vec<i32>) -> f64 {
    v.iter().sum::<i32>() as f64 / v.len() as f64
}
fn median(v: &Vec<i32>) -> f64 {    
    if v.len() % 2 == 0 {
        // return average of two middle values
        (v[v.len() / 2 - 1] + v[v.len() / 2]) as f64 / 2.0
    } else {
        v[v.len() / 2] as f64
    }
}
fn mode(v: &Vec<i32>) -> i32 {
    let mut h_map = HashMap::new();

    // count how many times key repeats.
    for d in v {
        let count = h_map.entry(*d).or_insert(0);
        *count += 1
    }
    // get biggest key: value
    let mut key: i32 = 0;
    let mut value: i32 = 0;
    for (k, v) in &h_map { 
        if v > &mut value {
            key = *k;
            value = *v;
        }
    }
    key
}


