pub fn double(iter: impl Iterator<Item = u32>) -> Vec<u32> {
    iter
        .map(|n| n*2)
        .collect::<Vec<_>>()
}

pub fn double_explicit<I: Iterator<Item = u32>>(iter: I) -> Vec<u32> {
    iter
        .map(|n| n*2)
        .collect::<Vec<_>>()
}


pub fn main() {
    let data1: Vec<u32> = vec![1, 2, 3];
    let data2 = [2, 4, 6];
    let data3 = vec![0, 0, 0];
    let data4 = [1; 3];

    println!(
        "{:?}",
        double(data1.into_iter())
    );

    println!(
        "{:?}",
        double(data2.into_iter())
    );

    // double above can expect many concrete containers, but intellisense will hide the concrete, 
    // even though we know it's a Vec's IntoIter<T> for example at call site.
    // Intellesence says:
    // rs_repro::repro::impl_trait_arg1_concrete
    // pub fn double(iter: impl Iterator<Item = u32>) -> Vec<u32>


    println!(
        "{:?}",
        double_explicit(data3.into_iter())
    );

    // Note with explicit double, intellsense knows we're working with a conrete type.
    // rs_repro::repro::impl_trait_arg1_concrete
    // pub fn double_explicit<I>(iter: I) -> Vec<u32>
    // where
    //     I: Iterator<Item = u32>,
    // I = IntoIter<u32>

    println!(
        "{:?}",
        double_explicit(data4.into_iter())
    );

    // Intellisense preserves concrete type on double_explicit:
    // rs_repro::repro::impl_trait_arg1_concrete
    // pub fn double_explicit<I>(iter: I) -> Vec<u32>
    // where
    //     I: Iterator<Item = u32>,
    // I = IntoIter<u32, 3>


}

/*
[2, 4, 6]
[4, 8, 12]
[0, 0, 0]
[2, 2, 2]
 */