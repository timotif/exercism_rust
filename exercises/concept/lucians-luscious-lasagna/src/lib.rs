pub fn expected_minutes_in_oven() -> i32 {
    // todo!("return expected minutes in the oven")
    return 40;
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
//    todo!(
//        "calculate remaining minutes in oven given actual minutes in oven: {actual_minutes_in_oven}"
//    )
    return expected_minutes_in_oven() - actual_minutes_in_oven   
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
//     todo!("calculate preparation time in minutes for number of layers: {number_of_layers}")
    return number_of_layers * 2
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
//    todo!(
//        "calculate elapsed time in minutes for number of layers {number_of_layers} and actual minutes in oven {actual_minutes_in_oven}"
//    )
    return preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
