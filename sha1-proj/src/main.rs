use sha1_proj::hash;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = std::env::args()
        .nth(1)
        .ok_or("Missing filename")?;
    let mut res = String::from("");
    for w in hash(&m) {
        res += &format!("{:08x}", w)
    }
    println!("{}", res);
    Ok(())
}