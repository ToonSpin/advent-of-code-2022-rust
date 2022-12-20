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

    fn mix(mut self) -> Self {
        for i in 0..self.list.len() {
            let mut item = self.list[i];
            let steps = (item.value.abs() as usize) % (self.list.len() - 1);
            if item.value < 0 {
                for _ in 0..steps {
                    self.list[item.next].prev = item.prev;
                    self.list[item.prev].next = item.next;

                    let new_prev = self.list[item.prev].prev;
                    let new_next = item.prev;

                    self.list[i].next = new_next;
                    self.list[i].prev = new_prev;
                    self.list[new_prev].next = i;
                    self.list[new_next].prev = i;

                    item = self.list[i];
                }
            } else {
                for _ in 0..steps {
                    self.list[item.prev].next = item.next;
                    self.list[item.next].prev = item.prev;

                    let new_next = self.list[item.next].next;
                    let new_prev = item.next;

                    self.list[i].prev = new_prev;
                    self.list[i].next = new_next;
                    self.list[new_next].prev = i;
                    self.list[new_prev].next = i;

                    item = self.list[i];
                }
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
