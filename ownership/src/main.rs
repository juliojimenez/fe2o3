fn main() {
    {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
        
        let s1 = s.clone();
        println!("s = {}, s2 = {}", s, s1);
        
        let s1 = String::from("hello");
        takes_ownership(s1);

        let x = 5;
        makes_copy(x);
        
        let s2 = gives_ownership();
        println!("{}", s2);
        
        let s3 = String::from("hello");
        let s4 = takes_and_gives_back(s3);
        println!("{}", s4);
    }
    
    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}", s2, len);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
