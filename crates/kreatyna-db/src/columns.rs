//! Column types that wrap a validated domain type, shared by the ingredient
//! and recipe rows. Reading a value that the domain type refuses fails, so a
//! broken file never becomes a domain value.

/// A decimal column stored as exact text, such as "12.3400", for a nutype
/// over `Decimal`. Reading text that is not an exact decimal, or that the
/// domain type refuses, fails.
macro_rules! db_decimal {
    ($name:ident, $domain:ty) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, ::diesel::AsExpression, ::diesel::FromSqlRow,
        )]
        #[diesel(sql_type = ::diesel::sql_types::Text)]
        pub(crate) struct $name(pub $domain);

        impl From<$domain> for $name {
            fn from(value: $domain) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $domain {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl ::diesel::deserialize::FromSql<::diesel::sql_types::Text, ::diesel::sqlite::Sqlite>
            for $name
        {
            fn from_sql(
                value: ::diesel::sqlite::SqliteValue<'_, '_, '_>,
            ) -> ::diesel::deserialize::Result<Self> {
                let text = <String as ::diesel::deserialize::FromSql<
                    ::diesel::sql_types::Text,
                    ::diesel::sqlite::Sqlite,
                >>::from_sql(value)?;
                let decimal = ::rust_decimal::Decimal::from_str_exact(&text)?;
                Ok(Self(<$domain>::try_new(decimal)?))
            }
        }

        impl ::diesel::serialize::ToSql<::diesel::sql_types::Text, ::diesel::sqlite::Sqlite>
            for $name
        {
            fn to_sql<'b>(
                &'b self,
                out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::sqlite::Sqlite>,
            ) -> ::diesel::serialize::Result {
                out.set_value(::rust_decimal::Decimal::from(self.0).to_string());
                Ok(::diesel::serialize::IsNull::No)
            }
        }
    };
}

/// A text column for a nutype over `String`, such as a name. Reading a text
/// that the domain type refuses, such as a blank name, fails.
macro_rules! db_text {
    ($name:ident, $domain:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, ::diesel::AsExpression, ::diesel::FromSqlRow)]
        #[diesel(sql_type = ::diesel::sql_types::Text)]
        pub(crate) struct $name(pub $domain);

        impl From<$domain> for $name {
            fn from(value: $domain) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $domain {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl ::diesel::deserialize::FromSql<::diesel::sql_types::Text, ::diesel::sqlite::Sqlite>
            for $name
        {
            fn from_sql(
                value: ::diesel::sqlite::SqliteValue<'_, '_, '_>,
            ) -> ::diesel::deserialize::Result<Self> {
                let text = <String as ::diesel::deserialize::FromSql<
                    ::diesel::sql_types::Text,
                    ::diesel::sqlite::Sqlite,
                >>::from_sql(value)?;
                Ok(Self(<$domain>::try_new(text)?))
            }
        }

        impl ::diesel::serialize::ToSql<::diesel::sql_types::Text, ::diesel::sqlite::Sqlite>
            for $name
        {
            fn to_sql<'b>(
                &'b self,
                out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::sqlite::Sqlite>,
            ) -> ::diesel::serialize::Result {
                out.set_value(self.0.as_ref());
                Ok(::diesel::serialize::IsNull::No)
            }
        }
    };
}

/// A text column for a plain Rust enum, each variant stored as its own
/// text, such as `active` or `g`. Any other text fails to read.
macro_rules! db_enum {
    ($name:ident, $domain:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, ::diesel::AsExpression, ::diesel::FromSqlRow)]
        #[diesel(sql_type = ::diesel::sql_types::Text)]
        pub(crate) struct $name(pub $domain);

        impl ::diesel::deserialize::FromSql<::diesel::sql_types::Text, ::diesel::sqlite::Sqlite>
            for $name
        {
            fn from_sql(
                value: ::diesel::sqlite::SqliteValue<'_, '_, '_>,
            ) -> ::diesel::deserialize::Result<Self> {
                let text = <String as ::diesel::deserialize::FromSql<
                    ::diesel::sql_types::Text,
                    ::diesel::sqlite::Sqlite,
                >>::from_sql(value)?;
                match text.as_str() {
                    $($text => Ok(Self($domain::$variant)),)+
                    other => Err(format!(
                        concat!("unknown ", stringify!($domain), " {:?}"),
                        other
                    )
                    .into()),
                }
            }
        }

        impl ::diesel::serialize::ToSql<::diesel::sql_types::Text, ::diesel::sqlite::Sqlite>
            for $name
        {
            fn to_sql<'b>(
                &'b self,
                out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::sqlite::Sqlite>,
            ) -> ::diesel::serialize::Result {
                out.set_value(match self.0 {
                    $($domain::$variant => $text,)+
                });
                Ok(::diesel::serialize::IsNull::No)
            }
        }
    };
}

pub(crate) use db_decimal;
pub(crate) use db_enum;
pub(crate) use db_text;
