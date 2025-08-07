// This reproduces this [answer](https://stackoverflow.com/a/67953582/6944447).

fn chunker<I>(chunk_size    : usize, 
              data          : I
             ) -> impl Iterator<Item = impl Iterator<Item = I::Item>>
where I: Iterator + Clone,
      I::Item: Clone,
{
    let mut p = data.peekable();
    
    std::iter::from_fn(
        move || {
            if p.peek().is_some() {
                let c = p.clone();
                for _ in 0..chunk_size { if p.next().is_none() { break; } }
                Some(c.take(chunk_size))
            } else {
                None
            }
        })
}

pub fn main() 
{
    for chunk in chunker(2, [1, 2, 3, 4, 5, 6, 7].iter()) {
        for n in chunk {
            print!("{}, ", n);
        }
        print!("\n");
    }
}