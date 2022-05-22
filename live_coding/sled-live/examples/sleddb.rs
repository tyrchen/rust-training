use anyhow::Result;

fn main() -> Result<()> {
    let db = sled::open("/tmp/sled_db")?;

    for i in 0..1000000 {
        db.insert(
            format!("hello{i}").as_bytes(),
            format!("world{i}").as_bytes(),
        )?;
        if i % 100000 == 0 {
            println!("{}", i);
        }
    }

    Ok(())
}
