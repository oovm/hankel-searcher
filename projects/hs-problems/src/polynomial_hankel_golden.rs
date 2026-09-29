//! Shared golden comparison helpers for polynomial Hankel energy reports.

use crate::ZetaPolynomialEnergyReport;

/// Tolerance for `log` field comparisons in offline golden runs.
pub const LOG_TOLERANCE: f64 = 0.05;

/// Expected energy logs at construction index `n=1` with offline N-q sweep optimum scaling.
#[derive(Debug, Clone, Copy)]
pub struct ZetaPolynomialGoldenN1 {
    /// `max |coeff P_K| bits` from primitive `Δ_K`.
    pub max_primitive_coeff_bits: usize,
    /// `log S_K`.
    pub log_s_k: f64,
    /// `log Δ_K(ζ(s))`.
    pub log_delta_at_zeta: f64,
    /// `log F_K(ζ(s))`.
    pub log_f_k: f64,
    /// `log content(F_K)`.
    pub log_content_f_k: f64,
    /// `log P_K(ζ(s))` for the primitive integer polynomial.
    pub log_primitive_at_zeta: f64,
}

impl ZetaPolynomialGoldenN1 {
    /// Compare `log S_K` only (no `Δ_K` required).
    pub fn assert_log_s_k(&self, log_s_k: f64) -> Result<(), String> {
        Self::assert_log("log S_K", log_s_k, self.log_s_k)
    }

    /// Compare an energy report against embedded golden values.
    pub fn assert_energy(&self, report: &ZetaPolynomialEnergyReport) -> Result<(), String> {
        if report.max_primitive_coeff_bits != self.max_primitive_coeff_bits {
            return Err(format!(
                "max |coeff P_K| bits: got {}, expected {}",
                report.max_primitive_coeff_bits,
                self.max_primitive_coeff_bits
            ));
        }
        self.assert_log_s_k(report.log_s_k)?;
        Self::assert_log("log Delta_K", report.log_delta_at_zeta, self.log_delta_at_zeta)?;
        Self::assert_log("log F_K", report.log_f_k, self.log_f_k)?;
        Self::assert_log("log content(F_K)", report.log_content_f_k, self.log_content_f_k)?;
        Self::assert_log("log P_K", report.log_primitive_at_zeta, self.log_primitive_at_zeta)?;
        Ok(())
    }

    fn assert_log(label: &str, actual: f64, expected: f64) -> Result<(), String> {
        if (actual - expected).abs() > LOG_TOLERANCE {
            return Err(format!("{label}: got {actual:.6}, expected {expected:.6}"));
        }
        Ok(())
    }
}

/// Golden expectations for `ζ(2)` at `n=1` (`K=40`, `N=2`, `q=4`, `h=38`, offline N-q sweep best).
pub fn zeta2_golden_n1() -> ZetaPolynomialGoldenN1 {
    ZetaPolynomialGoldenN1 {
        max_primitive_coeff_bits: 5395,
        log_s_k: 107.387,
        log_delta_at_zeta: -3769.136,
        log_f_k: -3661.749,
        log_content_f_k: -2240.255,
        log_primitive_at_zeta: -1457.538,
    }
}

/// Golden expectations for `ζ(3)` at `n=1` (`K=40`, `N=3`, `q=4`, `h=37`, offline N-q sweep best).
pub fn zeta3_golden_n1() -> ZetaPolynomialGoldenN1 {
    ZetaPolynomialGoldenN1 {
        max_primitive_coeff_bits: 5151,
        log_s_k: 60.861,
        log_delta_at_zeta: -3041.575,
        log_f_k: -2980.714,
        log_content_f_k: -1323.810,
        log_primitive_at_zeta: -1656.904,
    }
}
