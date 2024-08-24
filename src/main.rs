use random_number_code::random_number::{lucky_number_function, mobile_number_function, ussd_code_function,};

mod random_number_code;

fn main(){
    mobile_number_function();
    ussd_code_function();
    lucky_number_function();
}