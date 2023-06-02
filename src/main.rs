use std::ops::Range;
use std::convert::From;

fn main(){
    let domain = Domain::new(0, 8);
    println!("{:?}", domain);
    let domain_a = Domain::from(0..9);
    println!("{:?}, contains 9? {}", domain_a, domain_a.contains(9));
    println!("{:?}, is_bound? {}", domain_a.iterate(), domain_a.is_bound());
    let domain_b: Domain = (0..100).into();
    println!("{:?}", domain_b.iterate());
}

#[derive(Debug)]
struct Domain {
    size: usize,
    values: Vec<isize>,
    indices: Vec<usize>
}

impl Domain {
    fn new(min: usize, max: usize) -> Domain {
        let size = max - min + 1;
        let indices: Vec<usize> = (min..=max).map(|i| i).collect();
        let values: Vec<isize> = (min..=max).map(|v| v as isize).collect();
        Domain { size: size, 
            values: values, 
            indices: indices 
        }
    }

    fn contains(&self, v: usize) -> bool {
        match self.indices.get(v) {
            Some(&index) => index < self.size,
            None => false,
        }
    }

    fn is_bound(&self) -> bool {
        self.size == 1
    }

    fn iterate(&self) -> &[isize] {
        &self.values[0..self.size]
    }

}

impl From<Range<usize>> for Domain {
    fn from(item: Range<usize>) -> Self {
        Domain::new(item.start, item.end-1)
    }
}
