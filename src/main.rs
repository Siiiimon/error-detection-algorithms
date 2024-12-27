fn calculate_parity_bit(msg: [u8; 16]) -> bool {
    let mut sum = 0;
    for i in msg {
        sum += i;
    }
    sum % 2 == 1
}

fn main() {
    let even_message: [u8; 16] = [0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0];
    let odd_message: [u8; 16] = [0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0];

    let errored_even_message = {
        let mut msg = even_message;
        msg[10] ^= 1;
        msg
    };

    let errored_odd_message = {
        let mut msg = odd_message;
        msg[10] ^= 1;
        msg
    };

    assert!(!calculate_parity_bit(even_message));
    assert!(calculate_parity_bit(odd_message));
    assert!(calculate_parity_bit(errored_even_message));
    assert!(!calculate_parity_bit(errored_odd_message));
}
