use crate::RationalData;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::Zero;

pub fn ratio_from_data(data: &RationalData) -> Result<Ratio<BigInt>, String> {
    let num = data.num.parse::<BigInt>().map_err(|e| e.to_string())?;
    let den = data.den.parse::<BigInt>().map_err(|e| e.to_string())?;
    if den <= BigInt::zero() {
        return Err("rational denominator must be positive".into());
    }
    let ratio = Ratio::new(num, den);
    let canonical = RationalData::from_ratio(&ratio);
    if canonical.num != data.num || canonical.den != data.den {
        return Err("rational must be canonical".into());
    }
    Ok(ratio)
}
