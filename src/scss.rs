use std::iter::from_fn;
use std::collections::HashMap;


fn is_in(s1: &str, s2: &str) -> bool {
    for (i, c) in s2
        .chars()
        .enumerate() {
        if c == s1
            .chars()
            .nth(0)
            .expect("is_in 1") {
            if s1.len() + i > s2.len() {
                return false
            }
            for n in 0..s1.len() {
                if s2.chars()
                    .nth(n + i)
                    .expect("is_in 2") != s1.chars()
                    .nth(n)
                    .expect("is_in 3") {
                    break
                } else if n == s1.len() - 1 {
                    return true
                }
            }
        }
    }
    false
}

fn gen_subs(s: &'static str) -> impl Iterator<Item = String> {
    let l = s.len();
    let mut span = l;
    let mut window_start = 0;

    from_fn(move || {
        let mut substring = String::new();
        let window_end = window_start + span;
        for n in window_start..(window_end) {
            substring.push(s.chars().nth(n).expect("gen_subs"));
        }
        if span == 0 {
            return None
        }
        if window_end == l {
            window_start = 0;
            span -= 1;
        } else {
            window_start += 1;
        }
        Some(substring)
    })
}


pub fn test_scss() {
    //let mut cache = HashMap::new();
    let s1 = "jhdjhd23487623123456789adjysdyas";
    let s2 = "sldkjhdladylsjasdaskhdalsj123456789sd,fhsdflw74ysfljsfg33489";
    let s1_subs = gen_subs(s1);
    let mut longest = String::new();
    for subseq in s1_subs {
        if is_in(&subseq, &s2) {
            longest = subseq;
            break
        }
    }
    println!("{} {}", longest, longest.len());
}