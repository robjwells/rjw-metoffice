pub trait Sealed: Sized {}

pub trait TimePeriod: Sealed {}

impl Sealed for crate::Hourly {}
impl TimePeriod for crate::Hourly {}

impl Sealed for crate::ThreeHourly {}
impl TimePeriod for crate::ThreeHourly {}

impl Sealed for crate::Daily {}
impl TimePeriod for crate::Daily {}
