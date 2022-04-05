//////////////////// users //////////////
table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        phone -> Varchar,
        _type -> Varchar,
        gender -> Varchar,
        device -> Varchar,
        language -> Varchar,
        perm -> Varchar,
        level -> Int4,
        password -> Varchar,
        have_link -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        b_avatar -> Nullable<Text>,
        s_avatar -> Nullable<Text>,
        email -> Nullable<Varchar>,
    }
}
