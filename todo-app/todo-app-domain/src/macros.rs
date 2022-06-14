#[macro_export]
macro_rules! uuid {
    ($s:expr) => {{
        uuid::Uuid::parse_str($s).unwrap()
    }};
}

#[macro_export]
macro_rules! user_id {
    ($s:expr) => {{
        $crate::aggregate_root::user::value_object::UserId::from(uuid::Uuid::parse_str($s).unwrap())
    }};
}

#[macro_export]
macro_rules! user_name {
    ($s:expr) => {{
        $crate::aggregate_root::user::value_object::UserName::try_from(String::from($s)).unwrap()
    }};
}

#[macro_export]
macro_rules! email {
    ($s:expr) => {{
        $crate::aggregate_root::user_credential::value_object::Email::try_from(String::from($s))
            .unwrap()
    }};
}

#[macro_export]
macro_rules! password_hash {
    ($s:expr) => {{
        $crate::aggregate_root::user_credential::value_object::PasswordHash::try_from(String::from(
            $s,
        ))
        .unwrap()
    }};
}

#[macro_export]
macro_rules! password {
    ($s:expr) => {{
        $crate::aggregate_root::user_credential::value_object::Password::try_from(String::from($s))
            .unwrap()
    }};
}

#[macro_export]
macro_rules! todo_id {
    ($s:expr) => {{
        $crate::aggregate_root::todo::value_object::TodoId::from(uuid::Uuid::parse_str($s).unwrap())
    }};
}

#[macro_export]
macro_rules! todo_title {
    ($s:expr) => {{
        $crate::aggregate_root::todo::value_object::TodoTitle::try_from(String::from($s)).unwrap()
    }};
}

pub(crate) use email;
pub(crate) use password;
pub(crate) use password_hash;
pub(crate) use todo_id;
pub(crate) use todo_title;
pub(crate) use user_id;
pub(crate) use user_name;
pub(crate) use uuid;
