use postgres_types::{FromSql, ToSql};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "xacttype")]
pub enum XactType {
    #[postgres(name = "cr")]
    Cr,
    #[postgres(name = "dr")]
    Dr,
}

impl std::fmt::Display for XactType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xact = match self {
            XactType::Cr => "Cr",
            XactType::Dr => "Dr",
        };

        write!(f, "{}", xact)
    }
}

#[cfg(test)]
mod tests {

    use super::XactType;

    #[test]
    pub fn test_std_fmt_display() {
        assert_eq!(
            XactType::Cr.to_string(),
            "Cr",
            "String representation of XactType::Cr is 'Cr'"
        );
        assert_eq!(
            XactType::Dr.to_string(),
            "Dr",
            "String representation of XactType::Dr is 'Dr'"
        );
    }
}
