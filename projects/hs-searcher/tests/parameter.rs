use hs_checkpoint::{decode_rational_parameter, rational_parameter_ordinal, RATIONAL_PARAMETER_MAX_SHIFT};
use hs_searcher::{SearchBudget, enumerate_indices};
use hs_checkpoint::Observation;

#[test]
fn rational_parameter_ordinals_decode_in_order() {
    let stride = RATIONAL_PARAMETER_MAX_SHIFT + 1;
    for ordinal in 0..stride * 3 {
        let parameter = decode_rational_parameter(ordinal);
        assert_eq!(rational_parameter_ordinal(parameter.index, parameter.shift).unwrap(), ordinal);
    }
}

#[test]
fn enumerate_ordinals_preserves_merge_order() {
    let budget = SearchBudget::new(3);
    let report = enumerate_indices(0, None, &budget, 1, |ordinal| {
        let parameter = decode_rational_parameter(ordinal);
        Ok(Observation {
            kind: "finite_approximation_error_upper".into(),
            n: parameter.index,
            shift: Some(parameter.shift),
            series_terms: 64,
            approximant: hs_checkpoint::RationalData { num: ordinal.to_string(), den: "1".into() },
            error_upper: hs_checkpoint::RationalData {
                num: (100 - ordinal).to_string(),
                den: "1".into(),
            },
        })
    })
    .unwrap();
    assert_eq!(report.end_index, 3);
    assert!(report.bound_improved);
    assert_eq!(report.best.as_ref().unwrap().n, 0);
    assert_eq!(report.best.as_ref().unwrap().shift, Some(2));
}
