# A const-ready version std::ops arithmetic traits

All traits are the same as in std / core but prefixed with the #[const_trait] attribute to allow usage in const situations.

This crate is designed in such a way that. as soon as the #[const_trait] attribute is used in core / std that all imports of this crate can simply be deleted.