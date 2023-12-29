fn palindrome( s:String){
    let reverse : String= s.chars().rev().collect();
    if s == reverse {
        println!("{} is palindrome",s);
    }
    else{
        println!("{} is not a palindrome",s);
    }
}
fn main(){
    let mut s = String::new();
    let word:&str = "madam";
    s.push_str(word);
    palindrome(s);
}