macro_rules! impl_browse_includes {
    ($ty: ty, $(($args:ident, $inc: expr)),+) => {
        impl crate::BrowseQuery<$ty> {
               $(crate::query::browse::impl_browse_includes_inner!($args, $inc);)*

               crate::query::browse::impl_browse_relationships_includes!();
            }

            impl_relations_includes!(crate::BrowseQuery<$ty>);
        };
}

pub(crate) use impl_browse_includes;

macro_rules! impl_browse_relationships_includes {
    () => {
        crate::query::browse::impl_browse_includes_inner!(
            with_area_relations,
            Include::Relationship(Relationship::Area)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_artist_relations,
            Include::Relationship(Relationship::Artist)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_event_relations,
            Include::Relationship(Relationship::Event)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_genre_relations,
            Include::Relationship(Relationship::Genre)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_instrument_relations,
            Include::Relationship(Relationship::Instrument)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_label_relations,
            Include::Relationship(Relationship::Label)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_place_relations,
            Include::Relationship(Relationship::Place)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_recording_relations,
            Include::Relationship(Relationship::Recording)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_release_relations,
            Include::Relationship(Relationship::Release)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_release_group_relations,
            Include::Relationship(Relationship::ReleaseGroup)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_series_relations,
            Include::Relationship(Relationship::Series)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_url_relations,
            Include::Relationship(Relationship::Url)
        );
        crate::query::browse::impl_browse_includes_inner!(
            with_work_relations,
            Include::Relationship(Relationship::Work)
        );
    };
}

pub(crate) use impl_browse_relationships_includes;

macro_rules! impl_browse_includes_inner {
    ($args:ident, $inc: expr) => {
        pub fn $args(&mut self) -> &mut Self {
            self.inner.include = self.inner.include($inc).include.to_owned();
            self
        }
    };
}

pub(crate) use impl_browse_includes_inner;
