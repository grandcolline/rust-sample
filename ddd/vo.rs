pub struct ScreeningId(String);

impl TryFrom<String> for ScreeningId {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Ulid::from_string(&value) {
            Ok(ulid) => Ok(Self(ulid)),
            Err(e) => Err(()), // FIXME: 独自エラー
        }
    }
}

impl ScreeningId {
    pub fn new() -> Self {
        Self(Ulid::new())
    }
}

