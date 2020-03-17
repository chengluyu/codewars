fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi = weight as f32 / (height * height);
    if bmi <= 18.5f32 {
        "Underweight"
    } else if bmi <= 25.0f32 {
        "Normal"
    } else if bmi <= 30.0f32 {
        "Overweight"
    } else {
        "Obese"
    }
}
