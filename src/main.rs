#![allow(unused)]

use std::io;
use primes::{Sieve};





fn main() {

    let hw_pages:[i32; 5] = [232,12,111,43,5];  

    let v = vec![10,11];
    let mut name = String::from("boneyrick");
    let mut balance:u64 = 10000;
    let mut exp:f64 = 128.153232332;



    println!("my new name is{}",);
}


fn modifyName(s: &mut String){
    s.push_str("");
    
    

}

fn showInfo(s: &mut String){
    s.capacity();
    s.push_str(string)
}




/* mutable references

fn main() {

    let mut v = vec![10,11];
    add(&mut v);
    v.push(13);

}


//fn add borrowing ownership of the Vec from the caller
fn add(v: &mut Vec<i32>){
    v.push(12); 
}


*/


/* use-after-free bug

let mut v = vec![10,11];
let ptr = &mut v[0]; // ptr didn't return the ownership of vec! to v 
v.push(12);// second borrow (x)
println!("{}",*ptr);//ptr is dangling pointer

*/



/* 
fn findPrimeAfter(){

    let mut pset = Sieve::new();
    let (ix,n) = pset.find(1_000_000);
 
    println!("Prime {} :{}",ix , n);
}



*/
