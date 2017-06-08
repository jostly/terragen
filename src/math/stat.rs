use std::f32;

pub fn into_variance<I>(iter: I) -> f32
    where I: Iterator<Item = f32>
{
    let mut n = 0;
    let mut sum = 0.0;
    let mut sum2 = 0.0;

    for x in iter {
        n += 1;
        sum += x;
        sum2 += x * x;
    }

    if n < 2 {
        f32::NAN
    } else {
        (sum2 - sum * sum / n as f32) / n as f32
    }
}

pub fn variance<'a, I>(iter: I) -> f32
    where I: Iterator<Item = &'a f32>
{
    let mut n = 0;
    let mut sum = 0.0;
    let mut sum2 = 0.0;

    for x in iter {
        n += 1;
        sum += *x;
        sum2 += *x * *x;
    }

    if n < 2 {
        f32::NAN
    } else {
        (sum2 - sum * sum / n as f32) / n as f32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_variance() {
        let mut input_data = vec![1.0f32, 2.0f32, 3.0f32];
        input_data.reverse();
        let iter = input_data.iter();

        // mean = 2.0f
        // variance = ((1 - 2)^2 + (2 - 2)^2 + (3 - 2)^2) / 3 = (1^2 + 0^2 + 1^2) / 3 = 2/3

        assert_eq!(variance(iter), 2.0f32 / 3.0f32);
    }
}
