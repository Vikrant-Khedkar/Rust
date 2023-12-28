fn main(){
    let greeting:&str = "Greetings";
    let star:&str = "🌟";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(star);
    println!("Final sentence: {}",sentence);
    println!("{:?}",&sentence[0..5]);
    println!("{:?}", &sentence[5..7]);
}