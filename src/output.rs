use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    query_builder::bind_collector::RawBytesBindCollector,
    serialize::ToSql,
    sql_types::Binary,
};

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = Binary, check_for_backend(Pg))]
pub enum Body {
    Text(String),
}

impl Body {
    pub fn decode(body: Vec<u8>) -> Result<Self, PageError> {
        Ok(if body.starts_with(b":") {
            if body.starts_with(b":text:") {
                Self::Text(String::from_utf8(body)?.split_off(":text:".len()))
            } else {
                return Err(PageError::Invalid);
            }
        } else {
            Self::Text(String::from_utf8(body)?)
        })
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Body::Text(text) => write!(f, ":text:{text}"),
        }
    }
}

impl<DB> FromSql<Binary, DB> for Body
where
    DB: Backend,
    Vec<u8>: FromSql<Binary, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        Ok(Self::decode(Vec::from_sql(bytes)?)?)
    }
}

impl<DB> ToSql<Binary, DB> for Body
where
    DB: Backend,
    [u8]: ToSql<Binary, DB>,
    for<'c> DB: Backend<BindCollector<'c> = RawBytesBindCollector<DB>>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        format!("{self}").as_bytes().to_sql(&mut out.reborrow())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PageError {
    #[error("stored text is invalid")]
    Invalid,

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
}
