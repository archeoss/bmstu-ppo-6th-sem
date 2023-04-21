macro_rules! to_string_or_self {
    ($field:ident: &str) => {
        $field.to_string()
    };

    ($field:ident: $field_type:ty) => {
        $field
    };
}

macro_rules! setter {
    // Generate setter for a field
    (async $field:ident: $field_type:ty) => {
        paste::item! {
            pub async fn [<set_$field>](&mut self, value: $field_type)
            {
                ::defile::item! {
                    self.$field = to_string_or_self!(value: $field_type);
                }
            }
        }
    };
    ($field:ident: $field_type:ty) => {
        paste::item! {
            pub fn [<set_$field>](&mut self, value: $field_type)
            {
                ::defile::item! {
                    self.$field = to_string_or_self!(value: $field_type);
                }
            }
        }
    };
    ($($({ $prefix:tt })? $field:ident: $field_type:ty),+) => {
        $(setter!($($prefix)? $field: $field_type);)+
    };
}

macro_rules! getter {
    // generate getter for a field
    (const $field:ident: $field_type:ty) => {
        #[must_use]
        pub const fn $field(&self) -> $field_type {
            self.$field
        }
    };

    (async $field:ident: $field_type:ty) => {
        pub async fn $field(&self) -> $field_type {
            self.$field
        }
    };
    ($field:ident: $field_type:ty) => {
        #[must_use]
        pub fn $field(&self) -> $field_type {
            self.$field
        }
    };

    ($($({ $prefix:tt })? $field:ident: $field_type:ty),+) => {
        $(getter!($($prefix)? $field: $field_type);)+
    };
}

macro_rules! getter_mut {
    // generate getter for a field
    (async $field:ident: &mut $field_type:ty) => {
        paste::item! {
            pub async fn [<$field _mut>]<'get>(&'get mut self) -> &'get mut $field_type {
                &mut self.$field
            }
        }
    };

    ($field:ident: &mut $field_type:ty) => {
        paste::item! {
            #[must_use]
            pub fn [<$field _mut>]<'get>(&'get mut self) -> &'get mut $field_type {
                &mut self.$field
            }
        }
    };

    ($($({ $prefix:tt })? $field:ident: &mut $field_type:ty),+) => {
        $(getter_mut!($($prefix)? $field: &mut $field_type);)+
    };
}

macro_rules! getter_ref {
    // generate getter for a field
    (const $field:ident: &$field_type:ty) => {
        paste::item! {
            pub const fn [<$field _ref>]<'get>(&'get self) -> &'get $field_type {
                &self.$field
            }
        }
    };

    (async $field:ident: &$field_type:ty) => {
        paste::item! {
            pub async fn [<$field _ref>]<'get>(&'get self) -> &'get $field_type {
                &self.$field
            }
        }
    };

    ($field:ident: &$field_type:ty) => {
        paste::item! {
            pub fn [<$field _ref>]<'get>(&'get self) -> &'gt $field_type {
                &self.$field
            }
        }
    };

    ($($({ $prefix:tt })? $field:ident: &$field_type:ty),+) => {
        $(getter_ref!($($prefix)? $field: &$field_type);)+
    };
}

macro_rules! getter_setter {
    // Generate getter and setter for a field
    ( async $field:ident: $field_type:ty) => {
        getter!(async $field: $field_type);
        setter!(async $field: $field_type);
    };
    ( const $field:ident: $field_type:ty) => {
        getter!(const $field: $field_type);
        setter!(const $field: $field_type);
    };
    ($field:ident: $field_type:ty) => {
        getter!($field: $field_type);
        setter!($field: $field_type);
    };

    ($($({ $prefix:tt })? $field:ident: $field_type:ty),+) => {
        $(getter_setter!($($prefix)? $field: $field_type);)+
    };
}

pub(crate) use getter;
pub(crate) use getter_mut;
pub(crate) use getter_ref;
pub(crate) use getter_setter;
pub(crate) use setter;
pub(crate) use to_string_or_self;
