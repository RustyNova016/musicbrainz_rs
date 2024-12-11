macro_rules! impl_relations_includes {
    ($ty: ty) => {
        impl_includes!(
            $ty,
            (
                with_area_relations,
                Include::Relationship(Relationship::Area)
            ),
            (
                with_artist_relations,
                Include::Relationship(Relationship::Artist)
            ),
            (
                with_event_relations,
                Include::Relationship(Relationship::Event)
            ),
            (
                with_genre_relations,
                Include::Relationship(Relationship::Genre)
            ),
            (
                with_instrument_relations,
                Include::Relationship(Relationship::Instrument)
            ),
            (
                with_label_relations,
                Include::Relationship(Relationship::Label)
            ),
            (
                with_place_relations,
                Include::Relationship(Relationship::Place)
            ),
            (
                with_recording_relations,
                Include::Relationship(Relationship::Recording)
            ),
            (
                with_release_relations,
                Include::Relationship(Relationship::Release)
            ),
            (
                with_release_group_relations,
                Include::Relationship(Relationship::ReleaseGroup)
            ),
            (
                with_series_relations,
                Include::Relationship(Relationship::Series)
            ),
            (with_url_relations, Include::Relationship(Relationship::Url)),
            (
                with_work_relations,
                Include::Relationship(Relationship::Work)
            )
        );
    };
}

pub(crate) use impl_relations_includes;
