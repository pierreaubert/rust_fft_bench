use num_complex::Complex;
use rustfft::{num_complex::Complex32, num_complex::Complex64, FftPlanner};

/// Benchmark rustfft with Complex32 (f32)
pub fn rustfft_complex32(size: usize) -> Vec<Complex32> {
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(size);

    let mut buffer: Vec<Complex32> = (0..size)
        .map(|i| Complex32::new((i as f32).sin(), (i as f32).cos()))
        .collect();

    fft.process(&mut buffer);
    buffer
}

/// Benchmark rustfft with Complex64 (f64)
pub fn rustfft_complex64(size: usize) -> Vec<Complex64> {
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(size);

    let mut buffer: Vec<Complex64> = (0..size)
        .map(|i| Complex64::new((i as f64).sin(), (i as f64).cos()))
        .collect();

    fft.process(&mut buffer);
    buffer
}

/// Benchmark scirs2 with f32 input (note: scirs2 always returns Complex64)
pub fn scirs_complex32(size: usize) -> Vec<Complex<f64>> {
    let input: Vec<f32> = (0..size).map(|i| (i as f32).sin()).collect();

    scirs2::fft::fft(&input, None).unwrap()
}

/// Benchmark scirs2 with f64 input
pub fn scirs_complex64(size: usize) -> Vec<Complex<f64>> {
    let input: Vec<f64> = (0..size).map(|i| (i as f64).sin()).collect();

    scirs2::fft::fft(&input, None).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rustfft_complex32_size() {
        let result = rustfft_complex32(1024);
        assert_eq!(result.len(), 1024);
    }

    #[test]
    fn test_rustfft_complex64_size() {
        let result = rustfft_complex64(1024);
        assert_eq!(result.len(), 1024);
    }

    #[test]
    fn test_scirs_complex32_size() {
        let result = scirs_complex32(1024);
        assert_eq!(result.len(), 1024);
    }

    #[test]
    fn test_scirs_complex64_size() {
        let result = scirs_complex64(1024);
        assert_eq!(result.len(), 1024);
    }

    #[test]
    fn test_fft_parseval_theorem_rustfft() {
        // Parseval's theorem: sum of squares in time domain equals sum in frequency domain
        let size = 128;
        let input: Vec<Complex32> = (0..size)
            .map(|i| Complex32::new((i as f32).sin(), 0.0))
            .collect();

        let time_energy: f32 = input.iter().map(|c| c.norm_sqr()).sum();

        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(size);
        let mut buffer = input.clone();
        fft.process(&mut buffer);

        let freq_energy: f32 = buffer.iter().map(|c| c.norm_sqr()).sum::<f32>() / (size as f32);

        assert!((time_energy - freq_energy).abs() / time_energy < 0.01);
    }

    #[test]
    fn test_fft_parseval_theorem_scirs() {
        let size = 128;
        let input: Vec<f64> = (0..size).map(|i| (i as f64).sin()).collect();

        let time_energy: f64 = input.iter().map(|x| x * x).sum();

        let output = scirs2::fft::fft(&input, None).unwrap();

        let freq_energy: f64 = output.iter().map(|c| c.norm_sqr()).sum::<f64>() / (size as f64);

        assert!((time_energy - freq_energy).abs() / time_energy < 0.01);
    }
}
