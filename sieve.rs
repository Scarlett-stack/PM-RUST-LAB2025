fn sieve(n: usize) {
    let mut prime = vec![true; n + 1]; //sau asa : (n + 1) as usize
    prime[0] = false;
    prime[1] = false;

    //indexing uses usize, i have 64 bit os -> usize is diffrent than u32 previosly declared
    let mut i = 2;
    while i*i <= n {
        if prime[i] {
            let mut j = i * i;
            while j <= n {
                prime[j] = false;
                j= j + i;
            }
        }
        i = i +1;
    }

    for p in 2..=n {
        if prime[p] == true {
            println!("{} ", p);
        }
    }
}
fn main() {
    sieve(50);
}