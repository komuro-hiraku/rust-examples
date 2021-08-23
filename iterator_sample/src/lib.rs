// https://doc.rust-jp.rs/book-ja/ch13-02-iterators.html#%E7%92%B0%E5%A2%83%E3%82%92%E3%82%AD%E3%83%A3%E3%83%97%E3%83%81%E3%83%A3%E3%81%99%E3%82%8B%E3%82%AF%E3%83%AD%E3%83%BC%E3%82%B8%E3%83%A3%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%99%E3%82%8B

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// 独自イテレータ
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn shoe_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let in_my_size = shoe_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") },
            ]
        )
    }

    #[test]
    fn calling_next_direcly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_tarit_methods() {

        // (1, 2, 3, 4, 5, None) x (2, 3, 4, 5, None)
        // => 2, 6, 12, 20
        // => 6, 12 
        // => 18
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))   // 先頭をスキップ
                                    .map(|(a, b)| a * b)            // 二つの Counter を掛け合わせる
                                    .filter(|x| x % 3 == 0)         // 3 で割り切れるものを Filter
                                    .sum();                         // 全部足し合わせ

        assert_eq!(18, sum);
    }
}
