fn get_average(marks: &[f32]) -> f32 {
    (marks.iter().sum::<f32>() / (marks.len() as f32)).floor()
}
