#[macro_export]
macro_rules! define_id {
    ($name:ident: $type:ty) => {
        #[derive(Clone, Copy, Debug, Eq, ::derive_more::From, Hash, PartialEq)]
        pub struct $name(pub $type);
    };
}

#[macro_export]
macro_rules! define_key {
    ($data:ty, $table:ident, $key:ty, $value:ty) => {
        impl Key for $key {
            type Data = $data;
            type Value = $value;

            fn table_ref(data: &Self::Data) -> &Table<Self, Self::Value> {
                &data.$table
            }

            fn table_mut(data: &mut Self::Data) -> &mut Table<Self, Self::Value> {
                &mut data.$table
            }
        }
    };
}
