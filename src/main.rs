fn calculate_parity_bit(msg: [u8; 16]) -> bool {
    let mut sum = 0;
    for i in msg {
        sum += i;
    }
    sum % 2 == 0
}

fn main() {
    let even_message: [u8; 16] = [0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0];
    let odd_message: [u8; 16] = [0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0];

    println!(
        "parity for even message: {}",
        calculate_parity_bit(even_message),
    );
    println!(
        "parity for odd message: {}",
        calculate_parity_bit(odd_message),
    );
}
