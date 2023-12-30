// My Code
// fn collatz_length(mut n:i32) -> u32{
//     let mut lenght:u32 = 0;
// loop{
//         if n % 2 == 0{
//         n = n/2;
//         lenght = lenght +1;
        
//     }
//     else if n == 1{
//         return lenght
//     }
//     else{
//         n = 3*n+1;
//         lenght =  lenght +1;
//     }
// }
    

// }

// fn main() {
//     let  n:i32 = 4;
//     println!("Lenght for {} is {}",n,collatz_length(n));

// }
// Optimal Code
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    println!("Length: {}", collatz_length(11));
}