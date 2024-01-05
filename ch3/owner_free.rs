fn main() {
    {
        let s1 = String::from("Hello");
        let s3 = String::from("Good bye.");
        {
            let s2 = s1;
            println!("{}", s2);
        }
        println!("{}", s3);
    }
}