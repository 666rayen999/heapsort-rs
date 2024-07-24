#![feature(test)]
extern crate test;

pub trait HeapSort {
    fn heap_sort(&mut self);
    fn heap_sort_rev(&mut self);
}

#[inline]
fn heap_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    if n > 1 {
        for i in 1..n {
            if unsafe { arr.get_unchecked(i) } > unsafe { arr.get_unchecked((i - 1) >> 1) } {
                let mut j = i;
                while unsafe { arr.get_unchecked(j) } > unsafe { arr.get_unchecked((j - 1) >> 1) } {
                    (arr[j], arr[(j - 1) >> 1]) = (arr[(j - 1) >> 1], arr[j]);
                    j = (j - 1) >> 1;
                    if j < 1 {
                        break;
                    }
                }
            }
        }
        for i in (1..n).rev() {
            let temp = unsafe { *arr.get_unchecked(i) };
            arr[i] = unsafe { *arr.get_unchecked(0) };
            arr[0] = temp;
            let mut j = 0;
            loop {
                let mut index = (j << 1) + 1;
                if index < i - 1 {
                    if unsafe { arr.get_unchecked(index) } < unsafe { arr.get_unchecked(index + 1) }
                    {
                        index += 1;
                    }
                }
                if index < i {
                    if unsafe { arr.get_unchecked(j) } < unsafe { arr.get_unchecked(index) } {
                        (arr[j], arr[index]) = (arr[index], arr[j]);
                    }
                }
                j = index;
                if index >= i {
                    break;
                }
            }
        }
    }
}

#[inline]
fn heap_sort_rev<T: Copy + PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    if n > 1 {
        for i in 1..n {
            if unsafe { arr.get_unchecked(i) } < unsafe { arr.get_unchecked((i - 1) >> 1) } {
                let mut j = i;
                while unsafe { arr.get_unchecked(j) } < unsafe { arr.get_unchecked((j - 1) >> 1) } {
                    (arr[j], arr[(j - 1) >> 1]) = (arr[(j - 1) >> 1], arr[j]);
                    j = (j - 1) >> 1;
                    if j < 1 {
                        break;
                    }
                }
            }
        }
        for i in (1..n).rev() {
            let temp = unsafe { *arr.get_unchecked(i) };
            arr[i] = unsafe { *arr.get_unchecked(0) };
            arr[0] = temp;
            let mut j = 0;
            loop {
                let mut index = (j << 1) + 1;
                if index < i - 1 {
                    if unsafe { arr.get_unchecked(index) } > unsafe { arr.get_unchecked(index + 1) }
                    {
                        index += 1;
                    }
                }
                if index < i {
                    if unsafe { arr.get_unchecked(j) } > unsafe { arr.get_unchecked(index) } {
                        (arr[j], arr[index]) = (arr[index], arr[j]);
                    }
                }
                j = index;
                if index >= i {
                    break;
                }
            }
        }
    }
}

impl<T: Copy + PartialOrd> HeapSort for [T] {
    fn heap_sort(&mut self) {
        heap_sort(self);
    }
    fn heap_sort_rev(&mut self) {
        heap_sort_rev(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        let mut arr = [3, 2, 6, 1, 5, 4];
        arr.heap_sort();
        assert_eq!([1, 2, 3, 4, 5, 6], arr);
        arr.heap_sort_rev();
        assert_eq!([6, 5, 4, 3, 2, 1], arr);
    }

    #[bench]
    fn bench_heap_sort(b: &mut Bencher) {
        b.iter(|| {
            let mut arr = [
                3, 2, 1, 7, 3, 2, 9, 8, 1, 7, 6, 5, 2, 3, 9, 0, 1, 2, 3, 6, 1, 4, 6, 9,
            ];
            arr.heap_sort();
        })
    }

    #[bench]
    fn bench_sort(b: &mut Bencher) {
        b.iter(|| {
            let mut arr = [
                3, 2, 1, 7, 3, 2, 9, 8, 1, 7, 6, 5, 2, 3, 9, 0, 1, 2, 3, 6, 1, 4, 6, 9,
            ];
            arr.sort();
        })
    }
}
