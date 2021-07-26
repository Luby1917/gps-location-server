table! {
    geo_locations (d) {
        d -> Timestamptz,
        id -> Varchar,
        lat -> Nullable<Float4>,
        lon -> Nullable<Float4>,
        sat -> Nullable<Int4>,
        s -> Nullable<Float4>,
    }
}
