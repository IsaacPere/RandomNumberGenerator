pub mod random_number {
    use rand::Rng;
    use std::io;

    pub fn lucky_number_function() {
        let mut random_number = rand::thread_rng();
        let first_random_number_details: u64 = random_number.gen_range(1..(10u64).pow(10));
        println!("The first lucky random number is : {}", first_random_number_details);
        let second_random_number_details: u64 = random_number.gen_range(1..(10u64).pow(10));
        println!("The second lucky rnadom number is : {}", second_random_number_details);
    }

    pub fn mobile_number_function() {
        let mut phone_number = String::new();
        println!("Please eanter mobile number");
        io::stdin().read_line(&mut phone_number).expect("Failed to read line");
        let kenyan_phone_number = phone_number.trim();
        if kenyan_phone_number.chars().all(char::is_numeric) && kenyan_phone_number.len() == 10 {
            println!("And your number is {}", kenyan_phone_number);
        } else {
            println!("You entered the wrong mobile number");
        }
    }

    pub fn ussd_code_function() {
        let mut the_ussd_code = String::new();
        println!("Please enter the USSD Code");
        io::stdin().read_line(&mut the_ussd_code).expect("Please enter the correct USSD Code");
    }


}
