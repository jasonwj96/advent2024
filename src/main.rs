pub mod common {
    pub mod math;
}

fn main() {
    let scores: Vec<f32> = vec![1.0, 2.0, 3.0, 5.0, 19.0, 50.4];

    // Call the softmax function from the common module
    let probabilities= common::math::softmax(&scores);

    // Print the softmax probabilities
    println!("Softmax probabilities: {:?}", probabilities);
}