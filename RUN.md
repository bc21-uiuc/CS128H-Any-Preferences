```
git clone https://github.com/bc21-uiuc/CS128H-Any-Preferences
cargo build
cargo run
```
edit `src/main.rs` to try out different components. (some example material is already provided)

Note: This project does not work however some components like the hilbert curve and the parse into vector works. But if it were to work, these would be the steps:

## Hilbert curve component:

The file `hilbert.rs` provides the following functions: `hilbert_2_to_1(dim: usize, point: (usize, usize))`, `hilbert_1_to_2(dim: usize, value: usize)`. The first function accepts a coordinate point and the dimension of the space. The dimension must be a power of 2, and point’s coordinates have to be within the dimensions. There is only one dimension parameter because it assumes the image is square. The function returns the value along the straightened out Hilbert curve. The second function performs the reverse operation, taking a value along a line, and returning the resulting point if the line were to be bent into the shape of the Hilbert curve. See [Wikipedia](https://en.wikipedia.org/wiki/Hilbert_curve) for more information.There’s also a [3Blue1Brown video](https://www.youtube.com/watch?v=3s7h2MHQtxc) on the topic.

example
```
mod hilbert;
fn main() {
     for i in 0..8 * 8 {
          let point = hilbert::hilbert_1_to_2(8, i).unwrap();
          print!("{:?}", point);
     }
}
```
this will print out the coordinates (0-indexed) points along the Hilbert curve for a 8x8 square, starting at the bottom-left corner and ending at the bottom-right corner.

## Parse into vector component:

The only function in the ‘parse.rs’ file you need is the ‘giveInts()’ function which does not require any inputs (we’ve only used it for testing some html files) but can be modified to require an input text of the html file name. This function will return a vector of i32 in the range 0..256 based on factors like the analyzed sentiment, case, and rng. The sentiment analysis is taken from https://github.com/ckw017/vader-sentiment-rust/blob/master/src/demo.rs. 
