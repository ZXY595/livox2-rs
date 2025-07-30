macro_rules! BuilderMethods {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident {
        $(
            $(#[$field_attr:meta])*
            $field_vis:vis $field:ident: $ty:ty
        ),* $(,)?
        }
    ) => {
        impl $name {
            $(
                $(#[$field_attr])*
                $field_vis const fn $field(mut self, new_value: $ty) -> Self {
                    self.$field = new_value;
                    self
                }
            )*
        }
    };
}
