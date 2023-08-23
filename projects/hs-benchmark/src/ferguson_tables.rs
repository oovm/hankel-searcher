/// One Ferguson reference row from arXiv:2003.10616.
#[derive(Debug, Clone, Copy)]
pub struct FergusonReference {
    /// Approximant index `n`.
    pub n: usize,
    /// Decimal value of `P_n / Q_n`.
    pub value: f64,
    /// Optional exact rational string from the paper.
    pub rational: Option<&'static str>,
}

/// Table 1, Euler-Gompertz `δ`.
pub fn delta_table() -> &'static [FergusonReference] {
    &[
        FergusonReference { n: 0, value: 0.500_000_000_0, rational: Some("1/2") },
        FergusonReference { n: 1, value: 0.571_428_571_4, rational: Some("4/7") },
        FergusonReference { n: 2, value: 0.588_235_294_1, rational: Some("10/17") },
        FergusonReference { n: 3, value: 0.593_301_435_4, rational: Some("124/209") },
        FergusonReference { n: 4, value: 0.595_084_088_0, rational: Some("460/773") },
        FergusonReference { n: 5, value: 0.595_782_996_9, rational: Some("7940/13327") },
        FergusonReference { n: 6, value: 0.596_080_108_8, rational: Some("39020/65461") },
        FergusonReference { n: 7, value: 0.596_214_683_9, rational: Some("859580/1441729") },
        FergusonReference { n: 8, value: 0.596_278_854_1, rational: Some("748420/1255151") },
        FergusonReference { n: 9, value: 0.596_310_788_5, rational: None },
        FergusonReference { n: 10, value: 0.596_327_267_1, rational: None },
        FergusonReference { n: 11, value: 0.596_336_040_0, rational: None },
        FergusonReference { n: 12, value: 0.596_340_839_5, rational: None },
        FergusonReference { n: 13, value: 0.596_343_529_3, rational: None },
        FergusonReference { n: 14, value: 0.596_345_069_3, rational: None },
        FergusonReference { n: 15, value: 0.596_345_968_3, rational: None },
        FergusonReference { n: 17, value: 0.596_346_824_5, rational: None },
        FergusonReference { n: 19, value: 0.596_347_144_2, rational: None },
        FergusonReference { n: 21, value: 0.596_347_270_0, rational: None },
        FergusonReference { n: 23, value: 0.596_347_321_8, rational: None },
        FergusonReference { n: 25, value: 0.596_347_343_9, rational: None },
    ]
}

/// Table 1, Euler-Mascheroni `γ` (decimal column only).
pub fn gamma_table() -> &'static [FergusonReference] {
    &[
        FergusonReference { n: 0, value: 0.219_512_195_1, rational: Some("9/41") },
        FergusonReference { n: 1, value: 0.301_142_313_7, rational: None },
        FergusonReference { n: 2, value: 0.345_722_585_6, rational: None },
        FergusonReference { n: 3, value: 0.374_536_086_4, rational: None },
        FergusonReference { n: 4, value: 0.395_017_258_8, rational: None },
        FergusonReference { n: 5, value: 0.410_494_148_3, rational: None },
        FergusonReference { n: 6, value: 0.422_699_366_3, rational: None },
        FergusonReference { n: 7, value: 0.432_632_101_0, rational: None },
        FergusonReference { n: 8, value: 0.440_912_992_8, rational: None },
        FergusonReference { n: 9, value: 0.447_949_943_6, rational: None },
        FergusonReference { n: 10, value: 0.454_023_218_2, rational: None },
        FergusonReference { n: 11, value: 0.459_332_421_5, rational: None },
        FergusonReference { n: 12, value: 0.464_023_985_0, rational: None },
        FergusonReference { n: 13, value: 0.468_208_035_2, rational: None },
        FergusonReference { n: 14, value: 0.471_969_166_7, rational: None },
        FergusonReference { n: 15, value: 0.475_373_556_9, rational: None },
        FergusonReference { n: 17, value: 0.481_312_303_6, rational: None },
        FergusonReference { n: 19, value: 0.486_336_376_1, rational: None },
        FergusonReference { n: 21, value: 0.490_657_328_4, rational: None },
        FergusonReference { n: 23, value: 0.494_424_226_1, rational: None },
        FergusonReference { n: 25, value: 0.497_745_485_6, rational: None },
    ]
}

/// Table 2, `ζ(2)`.
pub fn zeta2_table() -> &'static [FergusonReference] {
    &[
        FergusonReference { n: 0, value: 1.333_333_333, rational: Some("4/3") },
        FergusonReference { n: 1, value: 1.516_853_933, rational: Some("135/89") },
        FergusonReference { n: 2, value: 1.575_512_966, rational: None },
        FergusonReference { n: 3, value: 1.601_458_618, rational: None },
        FergusonReference { n: 4, value: 1.615_170_202, rational: None },
        FergusonReference { n: 5, value: 1.623_286_170, rational: None },
        FergusonReference { n: 6, value: 1.628_483_935, rational: None },
        FergusonReference { n: 7, value: 1.632_011_765, rational: None },
        FergusonReference { n: 8, value: 1.634_515_372, rational: None },
        FergusonReference { n: 9, value: 1.636_356_043, rational: None },
        FergusonReference { n: 10, value: 1.637_748_743, rational: None },
        FergusonReference { n: 11, value: 1.638_827_873, rational: None },
        FergusonReference { n: 12, value: 1.639_680_964, rational: None },
        FergusonReference { n: 13, value: 1.640_367_005, rational: None },
        FergusonReference { n: 14, value: 1.640_926_928, rational: None },
        FergusonReference { n: 15, value: 1.641_389_854, rational: None },
        FergusonReference { n: 17, value: 1.642_103_939, rational: None },
        FergusonReference { n: 19, value: 1.642_622_098, rational: None },
        FergusonReference { n: 21, value: 1.643_009_963, rational: None },
        FergusonReference { n: 23, value: 1.643_307_821, rational: None },
        FergusonReference { n: 25, value: 1.643_541_511, rational: None },
    ]
}

/// Table 2, `ζ(3)`.
pub fn zeta3_table() -> &'static [FergusonReference] {
    &[
        FergusonReference { n: 0, value: 1.142_857_143, rational: Some("8/7") },
        FergusonReference { n: 1, value: 1.190_499_391, rational: Some("4887/4105") },
        FergusonReference { n: 2, value: 1.198_380_826, rational: None },
        FergusonReference { n: 3, value: 1.200_541_069, rational: None },
        FergusonReference { n: 4, value: 1.201_321_520, rational: None },
        FergusonReference { n: 5, value: 1.201_657_975, rational: None },
        FergusonReference { n: 6, value: 1.201_822_087, rational: None },
        FergusonReference { n: 7, value: 1.201_909_799, rational: None },
        FergusonReference { n: 8, value: 1.201_960_105, rational: None },
        FergusonReference { n: 9, value: 1.201_990_623, rational: None },
        FergusonReference { n: 10, value: 1.202_010_004, rational: None },
        FergusonReference { n: 11, value: 1.202_022_790, rational: None },
        FergusonReference { n: 12, value: 1.202_031_499, rational: None },
        FergusonReference { n: 13, value: 1.202_037_598, rational: None },
        FergusonReference { n: 14, value: 1.202_041_971, rational: None },
        FergusonReference { n: 15, value: 1.202_045_173, rational: None },
        FergusonReference { n: 17, value: 1.202_049_371, rational: None },
        FergusonReference { n: 19, value: 1.202_051_847, rational: None },
        FergusonReference { n: 21, value: 1.202_053_385, rational: None },
        FergusonReference { n: 23, value: 1.202_054_380, rational: None },
        FergusonReference { n: 25, value: 1.202_055_046, rational: None },
    ]
}
