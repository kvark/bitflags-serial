//! A wrapper around the standard bitflags macro that implements serde
//! serialization/deserialization by representing a bit mask as a
//! sequence of constant values defined by the user.

#[doc(hidden)]
pub extern crate core as _core;
#[doc(hidden)]
pub extern crate serde as _serde;


#[doc(hidden)]
pub struct _SingleBit<T>(pub T);

impl<'de, T> _serde::Deserialize<'de> for _SingleBit<T>
where
    T: Default + _serde::de::Visitor<'de, Value = T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: _serde::Deserializer<'de>
    {
        deserializer
            .deserialize_identifier(T::default())
            .map(_SingleBit)
    }
}


#[macro_export]
macro_rules! bitflags_serial {
    (
        $(#[$outer:meta])*
        struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )+
        }
    ) => {
        bitflags! {
            $(#[$outer])*
            struct $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    const $Flag = $value;
                )+
            }
        }

        impl Default for $BitFlags {
            fn default() -> Self {
                $BitFlags { bits: 0 }
            }
        }

        impl<'de> $crate::_serde::Deserialize<'de> for $BitFlags {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::_serde::Deserializer<'de>
            {
                #[derive(Deserialize)]
                #[repr(C)]
                enum __FlagEnum {
                    $(
                        #[allow(non_snake_case)]
                        $(? #[$attr $($args)*])*
                        $Flag = $value,
                    )+
                }

                let mut bits = 0;
                for value in Vec::<__FlagEnum>::deserialize(deserializer)? {
                    bits |= value as $T;
                }
                Ok($BitFlags { bits })
            }
        }

        impl $crate::_serde::Serialize for $BitFlags {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: $crate::_serde::Serializer
            {
                use $crate::_serde::ser::SerializeSeq;

                #[derive(Serialize)]
                enum __FlagEnum {
                    $(
                        #[allow(non_snake_case)]
                        $(? #[$attr $($args)*])*
                        $Flag,
                    )+
                }

                // This is taken from `Debug` implementation of `bitflags!`
                #[allow(non_snake_case)]
                trait __BitFlags {
                    $(
                        #[inline]
                        fn $Flag(&self) -> bool { false }
                    )+
                }
                impl __BitFlags for $BitFlags {
                    $(
                        __impl_bitflags! {
                            #[allow(deprecated)]
                            #[inline]
                            $(? #[$attr $($args)*])*
                            fn $Flag(&self) -> bool {
                                self.bits & Self::$Flag.bits == Self::$Flag.bits
                            }
                        }
                    )+
                }

                let mut seq = serializer.serialize_seq(None)?;
                $(
                    if <$BitFlags as __BitFlags>::$Flag(self) {
                        seq.serialize_element(&__FlagEnum::$Flag)?;
                    }
                )+
                seq.end()
            }
        }
    };
}
