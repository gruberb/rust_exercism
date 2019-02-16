pub fn nth(n: u32) -> u32 {
    let mut vec: Vec<_> = (2...limit).collect();
    
    if n == 0 {
        return 2 
    }

    for p in 2..n {
        vec.retain(|&x| x <= p || x % p != 0);
    }
    println!("{:?}", vec);
    vec
}
