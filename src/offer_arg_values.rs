
#[derive(Clone, Debug)]
pub struct OfferValues(Vec<clap::builder::PossibleValue>);

impl OfferValues {
    pub fn new(values: impl Into<OfferValues>) -> Self {
        values.into()
    }
}

impl clap::builder::TypedValueParser for OfferValues {
    type Value = String;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        clap::builder::TypedValueParser::parse(self, cmd, arg, value.to_owned())
    }

    fn parse(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: std::ffi::OsString,
    ) -> Result<String, clap::Error> {
        value.into_string().map_err(|_| {
            clap::Error::new(clap::error::ErrorKind::InvalidUtf8).with_cmd(cmd)
        })
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::builder::PossibleValue> + '_>> {
        Some(Box::new(self.0.iter().cloned()))
    }
}

impl<I, T> From<I> for OfferValues
where
    I: IntoIterator<Item = T>,
    T: Into<clap::builder::PossibleValue>,
{
    fn from(values: I) -> Self {
        Self(values.into_iter().map(|t| t.into()).collect())
    }
}
