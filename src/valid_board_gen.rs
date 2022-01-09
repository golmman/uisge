pub fn generate_valid_boards() {
    for i in 0..11058116888u64 {
        if i % 100000000 == 0 {
            println!("{}", i)
        }
    }
}