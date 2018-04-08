use std::cmp::min;
use std::convert::From;

fn highest_bit(mut x: u64) -> usize {
    if x == 0 {
        panic!("Zero has no bits set");
    }

    let mut ans = 0;
    let mut sz = 32;
    while sz != 0 {
        if (x & (((1 << sz) - 1) << sz)) != 0 {
            ans += sz;
            x >>= sz;
        }
        sz >>= 1;
    }
    ans
}

pub struct SparseTable<T> {
    table: Vec<Vec<T>>,
    row: Vec<usize>
}

impl<T> SparseTable<T> where T: Ord + Clone {
    fn from_vec(seq: Vec<T>) -> Self {
        let size = seq.len();
        let mut rows = vec![seq];
        let mut i = 1;

        while (1 << i) <= size {
            let span = 1 << i;
            let next_row: Vec<T>;
            {
                let prev_row = rows.last().unwrap();
                next_row = prev_row.iter().zip(prev_row.iter().skip(span / 2))
                                          .map(|(l, r)| min(l, r).clone())
                                          .collect();
            }
            rows.push(next_row);
            i += 1;
        }

        SparseTable {
            table: rows,
            row: (0..size + 1).map(|x| {
                if x != 0 {
                    highest_bit(x as u64)
                } else {
                    0
                }
            }).collect()
        }
    }

    pub fn new(seq: &[T]) -> Self {
        Self::from_vec(seq.to_vec())
    }

    pub fn smallest(&self, l: usize, r: usize) -> &T {
        if l >= r {
            panic!("No smallest element in an empty range");
        }
        if r > self.table[0].len() {
            panic!("Right bound is out of bounds");
        }
        let row = self.row[r - l];
        let span = 1 << row;
        min(&self.table[row][l], &self.table[row][r - span])
    }

    pub fn smallest_with_default(&self, l: usize, r: usize, default: &T) -> T {
        if l >= r || l >= self.table[0].len() {
            return default.clone();
        }
        self.smallest(l, min(r, self.table[0].len())).clone()
    }
}

impl<S, T> From<S> for SparseTable<T> where S: Into<Vec<T>>, T: Ord + Clone {
    fn from(seq: S) -> Self {
        SparseTable::<T>::from_vec(seq.into())
    }
}

#[test]
#[should_panic]
fn test_highest_bit_fails_on_zero() {
    highest_bit(0);
}

#[test]
fn test_highest_bit_powers_of_two() {
    assert_eq!(highest_bit(1), 0);
    assert_eq!(highest_bit(2), 1);
    assert_eq!(highest_bit(4), 2);
    assert_eq!(highest_bit(8), 3);
    assert_eq!(highest_bit(16), 4);
    assert_eq!(highest_bit(32), 5);
    assert_eq!(highest_bit(64), 6);
    assert_eq!(highest_bit(128), 7);
    assert_eq!(highest_bit(256), 8);
    assert_eq!(highest_bit(512), 9);
    assert_eq!(highest_bit(1024), 10);
    assert_eq!(highest_bit(2048), 11);
    assert_eq!(highest_bit(4096), 12);
    assert_eq!(highest_bit(8192), 13);
    assert_eq!(highest_bit(16384), 14);
    assert_eq!(highest_bit(32768), 15);
    assert_eq!(highest_bit(65536), 16);
    assert_eq!(highest_bit(131072), 17);
    assert_eq!(highest_bit(262144), 18);
    assert_eq!(highest_bit(524288), 19);
    assert_eq!(highest_bit(1048576), 20);
    assert_eq!(highest_bit(2097152), 21);
    assert_eq!(highest_bit(4194304), 22);
    assert_eq!(highest_bit(8388608), 23);
    assert_eq!(highest_bit(16777216), 24);
    assert_eq!(highest_bit(33554432), 25);
    assert_eq!(highest_bit(67108864), 26);
    assert_eq!(highest_bit(134217728), 27);
    assert_eq!(highest_bit(268435456), 28);
    assert_eq!(highest_bit(536870912), 29);
    assert_eq!(highest_bit(1073741824), 30);
    assert_eq!(highest_bit(2147483648), 31);
    assert_eq!(highest_bit(4294967296), 32);
    assert_eq!(highest_bit(8589934592), 33);
    assert_eq!(highest_bit(17179869184), 34);
    assert_eq!(highest_bit(34359738368), 35);
    assert_eq!(highest_bit(68719476736), 36);
    assert_eq!(highest_bit(137438953472), 37);
    assert_eq!(highest_bit(274877906944), 38);
    assert_eq!(highest_bit(549755813888), 39);
    assert_eq!(highest_bit(1099511627776), 40);
    assert_eq!(highest_bit(2199023255552), 41);
    assert_eq!(highest_bit(4398046511104), 42);
    assert_eq!(highest_bit(8796093022208), 43);
    assert_eq!(highest_bit(17592186044416), 44);
    assert_eq!(highest_bit(35184372088832), 45);
    assert_eq!(highest_bit(70368744177664), 46);
    assert_eq!(highest_bit(140737488355328), 47);
    assert_eq!(highest_bit(281474976710656), 48);
    assert_eq!(highest_bit(562949953421312), 49);
    assert_eq!(highest_bit(1125899906842624), 50);
    assert_eq!(highest_bit(2251799813685248), 51);
    assert_eq!(highest_bit(4503599627370496), 52);
    assert_eq!(highest_bit(9007199254740992), 53);
    assert_eq!(highest_bit(18014398509481984), 54);
    assert_eq!(highest_bit(36028797018963968), 55);
    assert_eq!(highest_bit(72057594037927936), 56);
    assert_eq!(highest_bit(144115188075855872), 57);
    assert_eq!(highest_bit(288230376151711744), 58);
    assert_eq!(highest_bit(576460752303423488), 59);
    assert_eq!(highest_bit(1152921504606846976), 60);
    assert_eq!(highest_bit(2305843009213693952), 61);
    assert_eq!(highest_bit(4611686018427387904), 62);
    assert_eq!(highest_bit(9223372036854775808), 63);
}

#[test]
fn test_highest_bit() {
    assert_eq!(highest_bit(3), 1);
    assert_eq!(highest_bit(3 << 32), 1 + 32);
    assert_eq!(highest_bit(3 << 48), 1 + 48);
    assert_eq!(highest_bit(5), 2);
    assert_eq!(highest_bit(5 << 32), 2 + 32);
    assert_eq!(highest_bit(5 << 48), 2 + 48);
    assert_eq!(highest_bit(7), 2);
    assert_eq!(highest_bit(7 << 32), 2 + 32);
    assert_eq!(highest_bit(7 << 48), 2 + 48);
    assert_eq!(highest_bit(9), 3);
    assert_eq!(highest_bit(9 << 32), 3 + 32);
    assert_eq!(highest_bit(9 << 48), 3 + 48);
}

#[test]
#[should_panic]
fn test_empty_sparse_table_panics() {
    let st = SparseTable::<u32>::new(&[]);
    st.smallest(0, 1);
}

#[test]
#[should_panic]
fn test_sparse_table_empty_range_panics() {
    let st = SparseTable::<u32>::new(&[]);
    st.smallest(0, 0);
}

#[test]
fn test_sparse_table_from_vec() {
    let seq: Vec<u32> = vec![0, 1, 2, 3];
    let st = SparseTable::from(seq);
    assert_eq!(*st.smallest(0, 1), 0);
    assert_eq!(*st.smallest(1, 2), 1);
    assert_eq!(*st.smallest(2, 3), 2);
    assert_eq!(*st.smallest(3, 4), 3);
    assert_eq!(*st.smallest(0, 2), 0);
    assert_eq!(*st.smallest(1, 3), 1);
    assert_eq!(*st.smallest(2, 4), 2);
    assert_eq!(*st.smallest(0, 3), 0);
    assert_eq!(*st.smallest(1, 4), 1);
    assert_eq!(*st.smallest(0, 4), 0);
}

#[test]
fn test_sparse_table() {
    let st1 = SparseTable::<u32>::new(&[1, 0, 1, 0, 1, 0]);
    assert_eq!(*st1.smallest(0, 1), 1);
    assert_eq!(*st1.smallest(1, 2), 0);
    assert_eq!(*st1.smallest(2, 3), 1);
    assert_eq!(*st1.smallest(3, 4), 0);
    assert_eq!(*st1.smallest(4, 5), 1);
    assert_eq!(*st1.smallest(5, 6), 0);
    assert_eq!(*st1.smallest(0, 2), 0);
    assert_eq!(*st1.smallest(1, 3), 0);
    assert_eq!(*st1.smallest(2, 4), 0);
    assert_eq!(*st1.smallest(3, 5), 0);
    assert_eq!(*st1.smallest(4, 6), 0);
    assert_eq!(*st1.smallest(0, 3), 0);
    assert_eq!(*st1.smallest(1, 4), 0);
    assert_eq!(*st1.smallest(2, 5), 0);
    assert_eq!(*st1.smallest(3, 6), 0);
    assert_eq!(*st1.smallest(0, 4), 0);
    assert_eq!(*st1.smallest(1, 5), 0);
    assert_eq!(*st1.smallest(2, 6), 0);
    assert_eq!(*st1.smallest(0, 5), 0);
    assert_eq!(*st1.smallest(1, 6), 0);
    assert_eq!(*st1.smallest(0, 6), 0);

    assert_eq!(st1.smallest_with_default(0, 0, &42), 42);
    assert_eq!(st1.smallest_with_default(1, 1, &42), 42);
    assert_eq!(st1.smallest_with_default(2, 2, &42), 42);
    assert_eq!(st1.smallest_with_default(3, 3, &42), 42);
    assert_eq!(st1.smallest_with_default(4, 4, &42), 42);
    assert_eq!(st1.smallest_with_default(5, 5, &42), 42);
    assert_eq!(st1.smallest_with_default(6, 6, &42), 42);
    assert_eq!(st1.smallest_with_default(7, 7, &42), 42);
    assert_eq!(st1.smallest_with_default(6, 7, &42), 42);

    let st2 = SparseTable::<u32>::new(&[0, 1, 2, 3, 4, 5]);
    assert_eq!(*st2.smallest(0, 1), 0);
    assert_eq!(*st2.smallest(1, 2), 1);
    assert_eq!(*st2.smallest(2, 3), 2);
    assert_eq!(*st2.smallest(3, 4), 3);
    assert_eq!(*st2.smallest(4, 5), 4);
    assert_eq!(*st2.smallest(5, 6), 5);
    assert_eq!(*st2.smallest(0, 2), 0);
    assert_eq!(*st2.smallest(1, 3), 1);
    assert_eq!(*st2.smallest(2, 4), 2);
    assert_eq!(*st2.smallest(3, 5), 3);
    assert_eq!(*st2.smallest(4, 6), 4);
    assert_eq!(*st2.smallest(0, 3), 0);
    assert_eq!(*st2.smallest(1, 4), 1);
    assert_eq!(*st2.smallest(2, 5), 2);
    assert_eq!(*st2.smallest(3, 6), 3);
    assert_eq!(*st2.smallest(0, 4), 0);
    assert_eq!(*st2.smallest(1, 5), 1);
    assert_eq!(*st2.smallest(2, 6), 2);
    assert_eq!(*st2.smallest(0, 5), 0);
    assert_eq!(*st2.smallest(1, 6), 1);
    assert_eq!(*st2.smallest(0, 6), 0);

    assert_eq!(st2.smallest_with_default(0, 0, &42), 42);
    assert_eq!(st2.smallest_with_default(1, 1, &42), 42);
    assert_eq!(st2.smallest_with_default(2, 2, &42), 42);
    assert_eq!(st2.smallest_with_default(3, 3, &42), 42);
    assert_eq!(st2.smallest_with_default(4, 4, &42), 42);
    assert_eq!(st2.smallest_with_default(5, 5, &42), 42);
    assert_eq!(st2.smallest_with_default(6, 6, &42), 42);
    assert_eq!(st2.smallest_with_default(7, 7, &42), 42);
    assert_eq!(st2.smallest_with_default(6, 7, &42), 42);
}
