extern crate serde;

use {Ascii, Encoding, UniCase};

use self::serde::{ser, de};

impl<T: AsRef<str>> ser::Serialize for Ascii<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(self.0.as_ref())
    }
}

impl<T: AsRef<str>> ser::Serialize for UniCase<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let txt = match self.0 {
            Encoding::Ascii(ref x) => x.0.as_ref(),
            Encoding::Unicode(ref x) => x.0.as_ref(),
        };

        serializer.serialize_str(txt)
    }
}

impl<'de> de::Deserialize<'de> for Ascii<String> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let txt = String::deserialize(deserializer)?;

        Ok(Ascii::new(txt))
    }
}

impl<'de> de::Deserialize<'de> for Ascii<&'de str> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let txt = <&str>::deserialize(deserializer)?;

        Ok(Ascii::new(txt))
    }
}

impl<'de> de::Deserialize<'de> for UniCase<String> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let txt = String::deserialize(deserializer)?;

        Ok(UniCase::new(txt))
    }
}

impl<'de> de::Deserialize<'de> for UniCase<&'de str> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let txt = <&str>::deserialize(deserializer)?;

        Ok(UniCase::new(txt))
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_test;

    use super::*;

    use self::serde_test::{Token, assert_ser_tokens, assert_de_tokens};

    #[test]
    fn test_serialize_ascii_str() {
        let ascii = Ascii::new("hello world");

        assert_ser_tokens(&ascii, &[
            Token::Str("hello world"),
        ]);
    }

    #[test]
    fn test_serialize_ascii_string() {
        let ascii = Ascii::new(String::from("hello world"));

        assert_ser_tokens(&ascii, &[
            Token::String("hello world"),
        ]);
    }

    #[test]
    fn test_serialize_unicase_str() {
        let uc = UniCase::new("hello world");

        assert_ser_tokens(&uc, &[
            Token::Str("hello world"),
        ]);
    }

    #[test]
    fn test_serialize_unicase_string() {
        let uc = UniCase::new(String::from("hello world"));

        assert_ser_tokens(&uc, &[
            Token::Str("hello world"),
        ]);
    }

    #[test]
    fn test_deserialize_ascii_str() {
        let ascii = Ascii::new("hello world");

        assert_de_tokens(&ascii, &[
            Token::BorrowedStr("hello world"),
        ]);
    }

    #[test]
    fn test_deserialize_ascii_string() {
        let ascii = Ascii::new(String::from("hello world"));

        assert_de_tokens(&ascii, &[
            Token::String("hello world"),
        ]);
    }

    #[test]
    fn test_deserialize_unicase_str() {
        let unicase = UniCase::new("hello world");

        assert_de_tokens(&unicase, &[
            Token::BorrowedStr("hello world"),
        ]);
    }

    #[test]
    fn test_deserialize_unicase_string() {
        let unicase = UniCase::new(String::from("hello world"));

        assert_de_tokens(&unicase, &[
            Token::String("hello world"),
        ]);
    }
}
