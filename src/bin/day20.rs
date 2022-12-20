use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone)]
struct CircularListItem {
    value: i64,
    prev: usize,
    next: usize,
}

struct CircularList {
    list: Vec<CircularListItem>,
    zero_pos: usize,
}

impl CircularList {
    fn from(v: Vec<i64>, multiplier: i64) -> Self {
        let l = v.len();
        let mut list = Vec::new();
        let mut zero_pos = 0;
        for (index, value) in v.iter().enumerate() {
            list.push(CircularListItem {
                value: *value * multiplier,
                prev: (index + l - 1) % l,
                next: (index + l + 1) % l,
            });
            if *value == 0 {
                zero_pos = index;
            }
        }
        CircularList { list, zero_pos }
    }

    fn yank(&mut self, index: usize) {
        let item = self.list[index];
        self.list[item.next].prev = item.prev;
        self.list[item.prev].next = item.next;
    }

    fn insert_before(&mut self, index: usize, before: usize) {
        let new_prev = self.list[before].prev;
        let new_next = before;

        self.list[before].prev = index;
        self.list[new_prev].next = index;
        self.list[index].next = new_next;
        self.list[index].prev = new_prev;
    }

    fn insert_after(&mut self, index: usize, after: usize) {
        let new_next = self.list[after].next;
        let new_prev = after;

        self.list[after].next = index;
        self.list[new_next].prev = index;
        self.list[index].next = new_next;
        self.list[index].prev = new_prev;
    }

    fn mix(mut self) -> Self {
        for index in 0..self.list.len() {
            let value = self.list[index].value;
            let steps = (value.abs() as usize) % (self.list.len() - 1);
            if value > 0 {
                let mut after = index;
                for _ in 0..steps {
                    after = self.list[after].next;
                }
                self.yank(index);
                self.insert_after(index, after);
            }
            if value < 0 {
                let mut before = index;
                for _ in 0..steps {
                    before = self.list[before].prev;
                }
                self.yank(index);
                self.insert_before(index, before);
            }
        }
        self
    }

    fn nth_from_zero(&self, i: usize) -> i64 {
        let mut index = self.zero_pos;
        for _ in 0..i {
            index = self.list[index].next;
        }
        self.list[index].value
    }

    fn score(&self) -> i64 {
        let i = 1000 % self.list.len();
        let j = 2000 % self.list.len();
        let k = 3000 % self.list.len();
        self.nth_from_zero(i) + self.nth_from_zero(j) + self.nth_from_zero(k)
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = input.split("\n").filter(|s| s.len() > 0);
    let input: Vec<i64> = input.into_iter().map(|s| s.parse().unwrap()).collect();

    let list = CircularList::from(input.clone(), 1);
    let list = list.mix();
    println!("The grove coordinates before decryption: {}", list.score());

    let mut list = CircularList::from(input, 811589153);
    for _ in 0..10 {
        list = list.mix();
    }
    println!("The grove coordinates after decryption: {}", list.score());

    Ok(())
}
