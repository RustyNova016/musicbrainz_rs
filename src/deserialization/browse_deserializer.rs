use crate::entity::Browsable;
use crate::entity::BrowseResult;
use serde::de::DeserializeOwned;
use std::fmt;
use std::marker::PhantomData;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

// Browse result fields in musicbrainz api v2 are prefixed with resource type :
// this impl provide a generic browse result deserializer
impl<'de, T> Deserialize<'de> for BrowseResult<T>
where
    T: DeserializeOwned + Browsable,
{
    fn deserialize<D>(deserializer: D) -> Result<BrowseResult<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Browsable,
    {
        enum Field<T> {
            Count,
            Offset,
            Entities(PhantomData<T>),
        }

        impl<'de, T> Deserialize<'de> for Field<T>
        where
            T: Browsable,
        {
            fn deserialize<D>(deserializer: D) -> Result<Field<T>, D::Error>
            where
                D: Deserializer<'de>,
                T: Browsable,
            {
                struct FieldVisitor<T>(PhantomData<T>);

                impl<T> Visitor<'_> for FieldVisitor<T>
                where
                    T: Browsable,
                {
                    type Value = Field<T>;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`count` or `offset`, `entities`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field<T>, E>
                    where
                        E: de::Error,
                        T: Browsable,
                    {
                        match value {
                            field if field == T::COUNT_FIELD => Ok(Field::Count),
                            field if field == T::OFFSET_FIELD => Ok(Field::Offset),
                            field if field == T::ENTITIES_FIELD => {
                                Ok(Field::Entities(PhantomData::<T>))
                            }
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor(PhantomData::<T>))
            }
        }

        struct BrowseResultVisitor<T> {
            phatom: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for BrowseResultVisitor<T>
        where
            T: Browsable + Deserialize<'de>,
        {
            type Value = BrowseResult<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Browsable<T>")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<BrowseResult<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let count = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let offset = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let entities = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(BrowseResult {
                    count,
                    offset,
                    entities,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<BrowseResult<T>, V::Error>
            where
                T: Browsable + Deserialize<'de>,
                V: MapAccess<'de>,
            {
                let mut count: Option<i32> = None;
                let mut offset: Option<i32> = None;
                let mut entities: Option<Vec<T>> = None;
                while let Some(key) = map.next_key::<Field<T>>()? {
                    match key {
                        Field::Count => {
                            if count.is_some() {
                                return Err(de::Error::duplicate_field("count"));
                            }
                            count = Some(map.next_value()?);
                        }
                        Field::Offset => {
                            if offset.is_some() {
                                return Err(de::Error::duplicate_field("offset"));
                            }
                            offset = Some(map.next_value()?);
                        }
                        Field::Entities(_t) => {
                            if entities.is_some() {
                                return Err(de::Error::duplicate_field("entities"));
                            }
                            entities = Some(map.next_value()?);
                        }
                    }
                }
                let count = count.ok_or_else(|| de::Error::missing_field("count"))?;
                let offset = offset.ok_or_else(|| de::Error::missing_field("offset"))?;
                let entities = entities.ok_or_else(|| de::Error::missing_field("entities"))?;
                Ok(BrowseResult {
                    count,
                    offset,
                    entities,
                })
            }
        }
        const FIELDS: &[&str] = &["count", "offset", "artists"];

        deserializer.deserialize_struct(
            "BrowseResult",
            FIELDS,
            BrowseResultVisitor {
                phatom: PhantomData::<T>,
            },
        )
    }
}
