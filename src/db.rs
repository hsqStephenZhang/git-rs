#[cfg(test)]
mod tests{
    #[test]
    fn t1(){
        std::fs::create_dir(".git").unwrap();
    }
}