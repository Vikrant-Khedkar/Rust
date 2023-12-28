fn main(){
    let mut x = 20;
    while x>0{
        println!("{x}");
        x= x-1;
    }
    for x in 100..110{
        println!("{x}");
    }
    x=10;
    loop{
        println!("HI");
        x = x-1;
        if x<5{
            break;
        }
        
    }
}