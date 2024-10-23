fn validate_iterable(vector: &Vec<f32>) -> bool {
    vector.iter().any(|&x| x.is_nan() || x.is_infinite())
}

pub fn softmax(vector: &Vec<f32>) -> Vec<f32> {
    if validate_iterable(&vector) {
        return vector.clone();
    }

    let exp_values: Vec<f32> = vector.iter().map(|&x| x.exp()).collect();
    let sum_exp: f32 = exp_values.iter().sum();
    return exp_values.iter().map(|&x| x / sum_exp).collect();
}