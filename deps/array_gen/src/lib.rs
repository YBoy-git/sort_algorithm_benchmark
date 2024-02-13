pub fn gen_sorted(len: usize) -> Vec<i32> {
    let mut vec = vec![0; len];
    for (i, item) in vec.iter_mut().enumerate().take(len) {
        *item = i as i32 + 1;
    }
    vec
}

pub fn gen_reversed(len: usize) -> Vec<i32> {
    let mut vec = vec![0; len];
    for (i, item) in vec.iter_mut().enumerate().take(len) {
        *item = (len - i) as i32;
    }
    vec
}

pub fn gen_random(len: usize) -> Vec<i32> {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let mut vec = gen_sorted(len);
    for i in 0..len {
        vec.swap(i, rng.gen_range(0..len))
    }
    vec
}