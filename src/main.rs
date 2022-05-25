/* Project: BMI Calc
 * Developer: @usufdev
 * Date: 2020-05-26
 */
use learnrust::greet;
fn main(){
    let my_bmi:f32 = bmi_calculator(74.0, 180.0);
    println!("My BMI {}", my_bmi);
    greet();
}

fn bmi_calculator(weight: f32, height: f32) -> f32 {
    println!("Your Weight: {}kg", weight);
    println!("Your Height: {}cm", height);
    let height_in_m:f32 = height / 100.0;
    return weight / (height_in_m * height_in_m);
}