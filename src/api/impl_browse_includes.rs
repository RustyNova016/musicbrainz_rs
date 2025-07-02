macro_rules! impl_browse_includes {
    ($ty: ty, $(($args:ident, $inc: expr)),+) => {
        impl crate::BrowseQuery<$ty> {
               $(crate::api::impl_browse_includes::impl_browse_includes_inner!($args, $inc);)*

               crate::api::impl_browse_includes::impl_browse_relationships_includes!();
            }

            crate::api::impl_relations_includes::impl_relations_includes!(crate::BrowseQuery<$ty>);
        };
}

pub(crate) use impl_browse_includes;

macro_rules! impl_browse_relationships_includes {
    () => {
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_area_relations,
            Include::Relationship(crate::entity::Relationship::Area)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_artist_relations,
            Include::Relationship(crate::entity::Relationship::Artist)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_event_relations,
            Include::Relationship(crate::entity::Relationship::Event)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_genre_relations,
            Include::Relationship(crate::entity::Relationship::Genre)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_instrument_relations,
            Include::Relationship(crate::entity::Relationship::Instrument)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_label_relations,
            Include::Relationship(crate::entity::Relationship::Label)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_place_relations,
            Include::Relationship(crate::entity::Relationship::Place)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_recording_relations,
            Include::Relationship(crate::entity::Relationship::Recording)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_release_relations,
            Include::Relationship(crate::entity::Relationship::Release)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_release_group_relations,
            Include::Relationship(crate::entity::Relationship::ReleaseGroup)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_series_relations,
            Include::Relationship(crate::entity::Relationship::Series)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_url_relations,
            Include::Relationship(crate::entity::Relationship::Url)
        );
        crate::api::impl_browse_includes::impl_browse_includes_inner!(
            with_work_relations,
            Include::Relationship(crate::entity::Relationship::Work)
        );
    };
}

pub(crate) use impl_browse_relationships_includes;

macro_rules! impl_browse_includes_inner {
    ($args:ident, $inc: expr) => {
        pub fn $args(&mut self) -> &mut Self {
            self.inner.include($inc);
            self
        }
    };
}

pub(crate) use impl_browse_includes_inner;
