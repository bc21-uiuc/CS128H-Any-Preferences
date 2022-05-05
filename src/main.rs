mod speechsentiment;
mod Audio;


fn main() {
    let something: Vec<i32>= speechsentiment::parse::giveInts();
    Audio::WAVMaker::makeWAV(something);
    print!("Hello World!");
}
