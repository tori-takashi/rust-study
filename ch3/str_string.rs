fn main() {
    let ss: &str = "あいうえお";
    let so1: String = String::from(ss);
    let so2: String = ss.to_string();
    let ss2: &str = &so1;
    let ss3: &str = so1.as_str();
    println!("ss={:p}\nso1={:p}\nso2={:p}\nss2={:p}\nss3={:p}", ss, &so1, &so2, ss2, ss3);
}