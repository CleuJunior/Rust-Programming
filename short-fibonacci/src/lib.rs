pub fn create_empty() -> Vec<u8> {
    vec![]
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut result_vector: Vec<u8> = vec![];

    for _times in 0..count{
        result_vector.push(0);
    }

    result_vector
}

pub fn fibonacci() -> Vec<u8> {
    let mut result_vector = vec![1];
    let mut first_number = 0;
    let mut second_number = 1;

    for _times in 0..4 {
        result_vector.push(first_number + second_number);
        let temp_first = first_number;
        first_number = second_number;
        second_number = second_number + temp_first;
    };

    result_vector
}
