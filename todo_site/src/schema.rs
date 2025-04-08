// @generated automatically by Diesel CLI.

diesel::table! {
    tehtavat (id) {
        id -> Int4,
        paivamaara -> Date,
        tyoaika -> Int4,
        lyhyt_kuvaus -> Text,
        tarkempi_kuvaus -> Text,
    }
}
