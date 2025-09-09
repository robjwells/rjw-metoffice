pub trait Sealed {}

pub trait TimePeriod: Sealed {}

impl Sealed for crate::Hourly {}
impl TimePeriod for crate::Hourly {}
