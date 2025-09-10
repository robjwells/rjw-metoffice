pub trait Sealed: Sized {}

/// Unifying trait for the three forecast time periods
///
/// This is implemented by [`Hourly`], [`ThreeHourly`] and [`Daily`], and is sealed as these are
/// the only possible forecast time period types.
///
/// [`Hourly`]: crate::Hourly
/// [`ThreeHourly`]: crate::ThreeHourly
/// [`Daily`]: crate::Daily
pub trait TimePeriod: Sealed {}

impl Sealed for crate::Hourly {}
impl TimePeriod for crate::Hourly {}

impl Sealed for crate::ThreeHourly {}
impl TimePeriod for crate::ThreeHourly {}

impl Sealed for crate::Daily {}
impl TimePeriod for crate::Daily {}
