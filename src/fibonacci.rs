use std::collections::HashMap;


pub fn fib(n: usize) -> usize {
    fn memoised(i: usize, memo: &mut HashMap<usize, usize>) -> usize {
        if let Some(result) = memo.get(&i) {
            return *result
        } else {
            let result = {
                if i < 2 {
                    1
                } else {
                    memoised(i - 1, memo) + memoised(i - 2, memo)
                }
            };
            memo.insert(i, result);
            result
        }
    }
    memoised(n, &mut HashMap::new())
}
