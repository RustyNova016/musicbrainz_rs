use crate::entity::search::{SearchResult, Searchable};
use chrono::NaiveDateTime;
use serde::de::DeserializeOwned;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use std::fmt;
use std::marker::PhantomData;

const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%Z";

// Browse result fields in musicbrainz api v2 are prefixed with resource type :
// this impl provide a generic search result deserializer
impl<'de, T> Deserialize<'de> for SearchResult<T>
where
    T: DeserializeOwned + Searchable,
{
    fn deserialize<D>(deserializer: D) -> Result<SearchResult<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Searchable,
    {
        enum Field<T> {
            Created,
            Count,
            Offset,
            Entities(PhantomData<T>),
        }

        impl<'de, T> Deserialize<'de> for Field<T>
        where
            T: Searchable,
        {
            fn deserialize<D>(deserializer: D) -> Result<Field<T>, D::Error>
            where
                D: Deserializer<'de>,
                T: Searchable,
            {
                struct FieldVisitor<T>(PhantomData<T>);

                impl<T> Visitor<'_> for FieldVisitor<T>
                where
                    T: Searchable,
                {
                    type Value = Field<T>;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`created` or `count` or `offset`, `entities`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field<T>, E>
                    where
                        E: de::Error,
                        T: Searchable,
                    {
                        match value {
                            field if field == T::CREATED_FIELD => Ok(Field::Created),
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

        struct SearchResultVisitor<T> {
            _marker: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for SearchResultVisitor<T>
        where
            T: Searchable + Deserialize<'de>,
        {
            type Value = SearchResult<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Searchable<T>")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<SearchResult<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let created = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let count = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let offset = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let entities = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                Ok(SearchResult {
                    created,
                    count,
                    offset,
                    entities,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<SearchResult<T>, V::Error>
            where
                T: Searchable + Deserialize<'de>,
                V: MapAccess<'de>,
            {
                let mut created: Option<String> = None;
                let mut count: Option<i32> = None;
                let mut offset: Option<i32> = None;
                let mut entities: Option<Vec<T>> = None;

                while let Some(key) = map.next_key::<Field<T>>()? {
                    match key {
                        Field::Created => {
                            if created.is_some() {
                                return Err(de::Error::duplicate_field("created"));
                            }
                            created = Some(map.next_value()?);
                        }
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
                let created = created.ok_or_else(|| de::Error::missing_field("created"))?;
                let created = NaiveDateTime::parse_from_str(&created, FORMAT).unwrap();
                let count = count.ok_or_else(|| de::Error::missing_field("count"))?;
                let offset = offset.ok_or_else(|| de::Error::missing_field("offset"))?;

                let entities = entities.ok_or_else(|| de::Error::missing_field("entities"))?;
                Ok(SearchResult {
                    created,
                    count,
                    offset,
                    entities,
                })
            }
        }
        const FIELDS: &[&str] = &["created", "count", "offset", "artists"];

        deserializer.deserialize_struct(
            "SearchResult",
            FIELDS,
            SearchResultVisitor {
                _marker: PhantomData::<T>,
            },
        )
    }
}
