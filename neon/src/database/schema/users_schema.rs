table! {
  users (id) {
     id -> Integer,
     username -> Text,
     password_md5 -> Text,
     email -> Text,
     register_datetime -> Integer,
     latest_activity -> Integer,
     privileges -> Integer,
     donor_expire -> Integer,
  }
}
