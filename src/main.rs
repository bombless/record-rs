use hound::WavReader;

fn main() {
    let mut reader = WavReader::open("../record.wav").unwrap();

    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    let mut max_value = 0.0;
    let mut max_index = 0;

    for (index, sample) in samples.iter().enumerate() {
        let sample = *sample as f32 / i16::MAX as f32;
        if max_value < sample.abs() {
            max_value = sample.abs();
            max_index = index;
        }
    }
    println!("max_value {max_value:.3}");
    println!("max_value time {:.1}", max_index as f32 / reader.spec().sample_rate as f32)
}
