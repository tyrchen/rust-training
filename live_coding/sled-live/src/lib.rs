#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn sled_read_write_should_work() -> Result<()> {
        let db = sled::open("/tmp/sled_test")?;
        let key = b"hello";
        for i in 0..1000000 {
            db.insert(b"hello1", format!("world{i}").as_bytes())?;
        }
        let old_value = db.insert(key, b"world")?;
        assert_eq!(old_value, None);

        let v = db.get(key)?;
        assert_eq!(v, Some(b"world".into()));

        let old_value = db.remove(key)?;
        assert_eq!(old_value, Some(b"world".into()));
        Ok(())
    }
}
