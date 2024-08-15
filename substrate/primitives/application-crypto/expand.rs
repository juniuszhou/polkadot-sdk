#![feature(prelude_import)]
//! Traits and macros for constructing application specific strongly typed crypto wrappers.
#![warn(missing_docs)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use sp_core::crypto::{key_types, CryptoTypeId, KeyTypeId};
#[doc(hidden)]
#[cfg(feature = "full_crypto")]
pub use sp_core::crypto::{DeriveError, Pair, SecretStringError};
#[cfg(any(feature = "full_crypto", feature = "serde"))]
pub use sp_core::crypto::{DeriveJunction, Ss58Codec};
#[doc(hidden)]
pub use sp_core::{
    self,
    crypto::{ByteArray, CryptoType, Derive, IsWrappedBy, Public, UncheckedFrom, Wraps},
    RuntimeDebug,
};
#[doc(hidden)]
pub use codec;
#[doc(hidden)]
pub use scale_info;
#[doc(hidden)]
#[cfg(feature = "serde")]
pub use serde;
#[doc(hidden)]
pub use sp_std::{ops::Deref, vec::Vec};
pub mod ecdsa {
    //! Ecdsa crypto types.
    use crate::{KeyTypeId, RuntimePublic};
    use sp_std::vec::Vec;
    pub use sp_core::ecdsa::*;
    mod app {
        /// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
        #[codec(crate = crate::codec)]
        pub struct Public(super::Public);
        #[automatically_derived]
        impl ::core::clone::Clone for Public {
            #[inline]
            fn clone(&self) -> Public {
                Public(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Public {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Public {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<super::Public>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Public {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Public {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Public {
            #[inline]
            fn eq(&self, other: &Public) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Public {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Public,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Public {
            #[inline]
            fn cmp(&self, other: &Public) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl crate::codec::Encode for Public {
                fn size_hint(&self) -> usize {
                    crate::codec::Encode::size_hint(&&self.0)
                }
                fn encode_to<
                    __CodecOutputEdqy: crate::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    crate::codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
                }
                fn encode(
                    &self,
                ) -> crate::codec::alloc::vec::Vec<::core::primitive::u8> {
                    crate::codec::Encode::encode(&&self.0)
                }
                fn using_encoded<
                    R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R,
                >(&self, f: F) -> R {
                    crate::codec::Encode::using_encoded(&&self.0, f)
                }
            }
            #[automatically_derived]
            impl crate::codec::EncodeLike for Public {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl crate::codec::Decode for Public {
                fn decode<__CodecInputEdqy: crate::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, crate::codec::Error> {
                    ::core::result::Result::Ok(
                        Public({
                            let __codec_res_edqy = <super::Public as crate::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Public.0`"),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                    __codec_res_edqy
                                }
                            }
                        }),
                    )
                }
            }
        };
        impl core::fmt::Debug for Public {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Public").field(&self.0).finish()
            }
        }
        const _: () = {
            impl crate::codec::MaxEncodedLen for Public {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize.saturating_add(<super::Public>::max_encoded_len())
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for Public {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "Public",
                                "sp_application_crypto::ecdsa::app",
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .docs(
                            &[
                                "A generic `AppPublic` wrapper type over $public crypto; this has no specific App.",
                            ],
                        )
                        .composite(
                            ::scale_info::build::Fields::unnamed()
                                .field(|f| {
                                    f.ty::<super::Public>().type_name("super::Public")
                                }),
                        )
                }
            }
        };
        impl crate::Wraps for Public {
            type Inner = super::Public;
        }
        impl From<super::Public> for Public {
            fn from(inner: super::Public) -> Self {
                Self(inner)
            }
        }
        impl From<Public> for super::Public {
            fn from(outer: Public) -> Self {
                outer.0
            }
        }
        impl AsRef<super::Public> for Public {
            fn as_ref(&self) -> &super::Public {
                &self.0
            }
        }
        impl AsMut<super::Public> for Public {
            fn as_mut(&mut self) -> &mut super::Public {
                &mut self.0
            }
        }
        impl crate::CryptoType for Public {
            type Pair = Pair;
        }
        impl crate::AppCrypto for Public {
            type Public = Public;
            type Pair = Pair;
            type Signature = Signature;
            const ID: crate::KeyTypeId = sp_core::testing::ECDSA;
            const CRYPTO_ID: crate::CryptoTypeId = super::CRYPTO_ID;
        }
        impl crate::Derive for Public {
            fn derive<Iter: Iterator<Item = crate::DeriveJunction>>(
                &self,
                path: Iter,
            ) -> Option<Self> {
                self.0.derive(path).map(Self)
            }
        }
        impl core::fmt::Display for Public {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                use crate::Ss58Codec;
                f.write_fmt(format_args!("{0}", self.0.to_ss58check()))
            }
        }
        impl crate::serde::Serialize for Public {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> core::result::Result<S::Ok, S::Error>
            where
                S: crate::serde::Serializer,
            {
                use crate::Ss58Codec;
                serializer.serialize_str(&self.to_ss58check())
            }
        }
        impl<'de> crate::serde::Deserialize<'de> for Public {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: crate::serde::Deserializer<'de>,
            {
                use crate::{module_format_string_prelude::*, Ss58Codec};
                Public::from_ss58check(&String::deserialize(deserializer)?)
                    .map_err(|e| crate::serde::de::Error::custom({
                        let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                        res
                    }))
            }
        }
        impl AsRef<[u8]> for Public {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }
        impl AsMut<[u8]> for Public {
            fn as_mut(&mut self) -> &mut [u8] {
                self.0.as_mut()
            }
        }
        impl crate::ByteArray for Public {
            const LEN: usize = <super::Public>::LEN;
        }
        impl crate::Public for Public {}
        impl crate::AppPublic for Public {
            type Generic = super::Public;
        }
        impl<'a> TryFrom<&'a [u8]> for Public {
            type Error = ();
            fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
                <super::Public>::try_from(data).map(Into::into)
            }
        }
        impl Public {
            /// Convert into wrapped generic public key type.
            pub fn into_inner(self) -> super::Public {
                self.0
            }
        }
        /// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
        pub struct Signature(super::Signature);
        #[automatically_derived]
        impl ::core::hash::Hash for Signature {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Signature {
            #[inline]
            fn clone(&self) -> Signature {
                Signature(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Signature {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Signature {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<super::Signature>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Signature {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Signature {
            #[inline]
            fn eq(&self, other: &Signature) -> bool {
                self.0 == other.0
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for Signature {
                fn size_hint(&self) -> usize {
                    ::codec::Encode::size_hint(&&self.0)
                }
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    ::codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
                }
                fn encode(&self) -> ::codec::alloc::vec::Vec<::core::primitive::u8> {
                    ::codec::Encode::encode(&&self.0)
                }
                fn using_encoded<
                    R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R,
                >(&self, f: F) -> R {
                    ::codec::Encode::using_encoded(&&self.0, f)
                }
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for Signature {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for Signature {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    ::core::result::Result::Ok(
                        Signature({
                            let __codec_res_edqy = <super::Signature as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Signature.0`"),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                    __codec_res_edqy
                                }
                            }
                        }),
                    )
                }
            }
        };
        impl core::fmt::Debug for Signature {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Signature").field(&self.0).finish()
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for Signature {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "Signature",
                                "sp_application_crypto::ecdsa::app",
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .docs(
                            &[
                                "A generic `AppPublic` wrapper type over $public crypto; this has no specific App.",
                            ],
                        )
                        .composite(
                            ::scale_info::build::Fields::unnamed()
                                .field(|f| {
                                    f.ty::<super::Signature>().type_name("super::Signature")
                                }),
                        )
                }
            }
        };
        impl crate::Wraps for Signature {
            type Inner = super::Signature;
        }
        impl From<super::Signature> for Signature {
            fn from(inner: super::Signature) -> Self {
                Self(inner)
            }
        }
        impl From<Signature> for super::Signature {
            fn from(outer: Signature) -> Self {
                outer.0
            }
        }
        impl AsRef<super::Signature> for Signature {
            fn as_ref(&self) -> &super::Signature {
                &self.0
            }
        }
        impl AsMut<super::Signature> for Signature {
            fn as_mut(&mut self) -> &mut super::Signature {
                &mut self.0
            }
        }
        impl crate::CryptoType for Signature {
            type Pair = Pair;
        }
        impl crate::AppCrypto for Signature {
            type Public = Public;
            type Pair = Pair;
            type Signature = Signature;
            const ID: crate::KeyTypeId = sp_core::testing::ECDSA;
            const CRYPTO_ID: crate::CryptoTypeId = super::CRYPTO_ID;
        }
        impl crate::Deref for Signature {
            type Target = [u8];
            fn deref(&self) -> &Self::Target {
                self.0.as_ref()
            }
        }
        impl AsRef<[u8]> for Signature {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }
        impl crate::AppSignature for Signature {
            type Generic = super::Signature;
        }
        impl<'a> TryFrom<&'a [u8]> for Signature {
            type Error = ();
            fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
                <super::Signature>::try_from(data).map(Into::into)
            }
        }
        impl TryFrom<crate::Vec<u8>> for Signature {
            type Error = ();
            fn try_from(data: crate::Vec<u8>) -> Result<Self, Self::Error> {
                Self::try_from(&data[..])
            }
        }
        impl Signature {
            /// Convert into wrapped generic signature type.
            pub fn into_inner(self) -> super::Signature {
                self.0
            }
        }
        /// A generic `AppPublic` wrapper type over $pair crypto; this has no specific App.
        pub struct Pair(super::Pair);
        #[automatically_derived]
        impl ::core::clone::Clone for Pair {
            #[inline]
            fn clone(&self) -> Pair {
                Pair(::core::clone::Clone::clone(&self.0))
            }
        }
        impl crate::Wraps for Pair {
            type Inner = super::Pair;
        }
        impl From<super::Pair> for Pair {
            fn from(inner: super::Pair) -> Self {
                Self(inner)
            }
        }
        impl From<Pair> for super::Pair {
            fn from(outer: Pair) -> Self {
                outer.0
            }
        }
        impl AsRef<super::Pair> for Pair {
            fn as_ref(&self) -> &super::Pair {
                &self.0
            }
        }
        impl AsMut<super::Pair> for Pair {
            fn as_mut(&mut self) -> &mut super::Pair {
                &mut self.0
            }
        }
        impl crate::CryptoType for Pair {
            type Pair = Pair;
        }
        impl crate::Pair for Pair {
            type Public = Public;
            type Seed = <super::Pair as crate::Pair>::Seed;
            type Signature = Signature;
            fn generate_with_phrase(
                password: Option<&str>,
            ) -> (Self, String, Self::Seed) {
                let r = <super::Pair>::generate_with_phrase(password);
                (Self(r.0), r.1, r.2)
            }
            fn from_phrase(
                phrase: &str,
                password: Option<&str>,
            ) -> Result<(Self, Self::Seed), crate::SecretStringError> {
                <super::Pair>::from_phrase(phrase, password).map(|r| (Self(r.0), r.1))
            }
            fn derive<Iter: Iterator<Item = crate::DeriveJunction>>(
                &self,
                path: Iter,
                seed: Option<Self::Seed>,
            ) -> Result<(Self, Option<Self::Seed>), crate::DeriveError> {
                self.0.derive(path, seed).map(|x| (Self(x.0), x.1))
            }
            fn from_seed(seed: &Self::Seed) -> Self {
                Self(<super::Pair>::from_seed(seed))
            }
            fn from_seed_slice(seed: &[u8]) -> Result<Self, crate::SecretStringError> {
                <super::Pair>::from_seed_slice(seed).map(Self)
            }
            fn sign(&self, msg: &[u8]) -> Self::Signature {
                Signature(self.0.sign(msg))
            }
            fn verify<M: AsRef<[u8]>>(
                sig: &Self::Signature,
                message: M,
                pubkey: &Self::Public,
            ) -> bool {
                <super::Pair>::verify(&sig.0, message, pubkey.as_ref())
            }
            fn public(&self) -> Self::Public {
                Public(self.0.public())
            }
            fn to_raw_vec(&self) -> crate::Vec<u8> {
                self.0.to_raw_vec()
            }
        }
        impl crate::AppCrypto for Pair {
            type Public = Public;
            type Pair = Pair;
            type Signature = Signature;
            const ID: crate::KeyTypeId = sp_core::testing::ECDSA;
            const CRYPTO_ID: crate::CryptoTypeId = super::CRYPTO_ID;
        }
        impl crate::AppPair for Pair {
            type Generic = super::Pair;
        }
        impl Pair {
            /// Convert into wrapped generic key pair type.
            pub fn into_inner(self) -> super::Pair {
                self.0
            }
        }
    }
    #[cfg(feature = "full_crypto")]
    pub use app::Pair as AppPair;
    pub use app::{Public as AppPublic, Signature as AppSignature};
    impl RuntimePublic for Public {
        type Signature = Signature;
        fn all(key_type: KeyTypeId) -> crate::Vec<Self> {
            sp_io::crypto::ecdsa_public_keys(key_type)
        }
        fn generate_pair(key_type: KeyTypeId, seed: Option<Vec<u8>>) -> Self {
            sp_io::crypto::ecdsa_generate(key_type, seed)
        }
        fn sign<M: AsRef<[u8]>>(
            &self,
            key_type: KeyTypeId,
            msg: &M,
        ) -> Option<Self::Signature> {
            sp_io::crypto::ecdsa_sign(key_type, self, msg.as_ref())
        }
        fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
            sp_io::crypto::ecdsa_verify(signature, msg.as_ref(), self)
        }
        fn to_raw_vec(&self) -> Vec<u8> {
            sp_core::crypto::ByteArray::to_raw_vec(self)
        }
    }
}
pub mod ed25519 {
    //! Ed25519 crypto types.
    use crate::{KeyTypeId, RuntimePublic};
    use sp_std::vec::Vec;
    pub use sp_core::ed25519::*;
    mod app {
        /// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
        #[codec(crate = crate::codec)]
        pub struct Public(super::Public);
        #[automatically_derived]
        impl ::core::clone::Clone for Public {
            #[inline]
            fn clone(&self) -> Public {
                Public(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Public {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Public {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<super::Public>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Public {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Public {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Public {
            #[inline]
            fn eq(&self, other: &Public) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Public {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Public,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Public {
            #[inline]
            fn cmp(&self, other: &Public) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl crate::codec::Encode for Public {
                fn size_hint(&self) -> usize {
                    crate::codec::Encode::size_hint(&&self.0)
                }
                fn encode_to<
                    __CodecOutputEdqy: crate::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    crate::codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
                }
                fn encode(
                    &self,
                ) -> crate::codec::alloc::vec::Vec<::core::primitive::u8> {
                    crate::codec::Encode::encode(&&self.0)
                }
                fn using_encoded<
                    R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R,
                >(&self, f: F) -> R {
                    crate::codec::Encode::using_encoded(&&self.0, f)
                }
            }
            #[automatically_derived]
            impl crate::codec::EncodeLike for Public {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl crate::codec::Decode for Public {
                fn decode<__CodecInputEdqy: crate::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, crate::codec::Error> {
                    ::core::result::Result::Ok(
                        Public({
                            let __codec_res_edqy = <super::Public as crate::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Public.0`"),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                    __codec_res_edqy
                                }
                            }
                        }),
                    )
                }
            }
        };
        impl core::fmt::Debug for Public {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Public").field(&self.0).finish()
            }
        }
        const _: () = {
            impl crate::codec::MaxEncodedLen for Public {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize.saturating_add(<super::Public>::max_encoded_len())
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for Public {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "Public",
                                "sp_application_crypto::ed25519::app",
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .docs(
                            &[
                                "A generic `AppPublic` wrapper type over $public crypto; this has no specific App.",
                            ],
                        )
                        .composite(
                            ::scale_info::build::Fields::unnamed()
                                .field(|f| {
                                    f.ty::<super::Public>().type_name("super::Public")
                                }),
                        )
                }
            }
        };
        impl crate::Wraps for Public {
            type Inner = super::Public;
        }
        impl From<super::Public> for Public {
            fn from(inner: super::Public) -> Self {
                Self(inner)
            }
        }
        impl From<Public> for super::Public {
            fn from(outer: Public) -> Self {
                outer.0
            }
        }
        impl AsRef<super::Public> for Public {
            fn as_ref(&self) -> &super::Public {
                &self.0
            }
        }
        impl AsMut<super::Public> for Public {
            fn as_mut(&mut self) -> &mut super::Public {
                &mut self.0
            }
        }
        impl crate::CryptoType for Public {
            type Pair = Pair;
        }
        impl crate::AppCrypto for Public {
            type Public = Public;
            type Pair = Pair;
            type Signature = Signature;
            const ID: crate::KeyTypeId = sp_core::testing::ED25519;
            const CRYPTO_ID: crate::CryptoTypeId = super::CRYPTO_ID;
        }
        impl crate::Derive for Public {
            fn derive<Iter: Iterator<Item = crate::DeriveJunction>>(
                &self,
                path: Iter,
            ) -> Option<Self> {
                self.0.derive(path).map(Self)
            }
        }
        impl core::fmt::Display for Public {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                use crate::Ss58Codec;
                f.write_fmt(format_args!("{0}", self.0.to_ss58check()))
            }
        }
        impl crate::serde::Serialize for Public {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> core::result::Result<S::Ok, S::Error>
            where
                S: crate::serde::Serializer,
            {
                use crate::Ss58Codec;
                serializer.serialize_str(&self.to_ss58check())
            }
        }
        impl<'de> crate::serde::Deserialize<'de> for Public {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: crate::serde::Deserializer<'de>,
            {
                use crate::{module_format_string_prelude::*, Ss58Codec};
                Public::from_ss58check(&String::deserialize(deserializer)?)
                    .map_err(|e| crate::serde::de::Error::custom({
                        let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                        res
                    }))
            }
        }
        impl AsRef<[u8]> for Public {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }
        impl AsMut<[u8]> for Public {
            fn as_mut(&mut self) -> &mut [u8] {
                self.0.as_mut()
            }
        }
        impl crate::ByteArray for Public {
            const LEN: usize = <super::Public>::LEN;
        }
        impl crate::Public for Public {}
        impl crate::AppPublic for Public {
            type Generic = super::Public;
        }
        impl<'a> TryFrom<&'a [u8]> for Public {
            type Error = ();
            fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
                <super::Public>::try_from(data).map(Into::into)
            }
        }
        impl Public {
            /// Convert into wrapped generic public key type.
            pub fn into_inner(self) -> super::Public {
                self.0
            }
        }
        /// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
        pub struct Signature(super::Signature);
        #[automatically_derived]
        impl ::core::hash::Hash for Signature {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Signature {
            #[inline]
            fn clone(&self) -> Signature {
                Signature(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Signature {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Signature {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<super::Signature>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Signature {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Signature {
            #[inline]
            fn eq(&self, other: &Signature) -> bool {
                self.0 == other.0
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for Signature {
                fn size_hint(&self) -> usize {
                    ::codec::Encode::size_hint(&&self.0)
                }
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    ::codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
                }
                fn encode(&self) -> ::codec::alloc::vec::Vec<::core::primitive::u8> {
                    ::codec::Encode::encode(&&self.0)
                }
                fn using_encoded<
                    R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R,
                >(&self, f: F) -> R {
                    ::codec::Encode::using_encoded(&&self.0, f)
                }
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for Signature {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for Signature {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    ::core::result::Result::Ok(
                        Signature({
                            let __codec_res_edqy = <super::Signature as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Signature.0`"),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                    __codec_res_edqy
                                }
                            }
                        }),
                    )
                }
            }
        };
        impl core::fmt::Debug for Signature {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Signature").field(&self.0).finish()
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for Signature {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "Signature",
                                "sp_application_crypto::ed25519::app",
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .docs(
                            &[
                                "A generic `AppPublic` wrapper type over $public crypto; this has no specific App.",
                            ],
                        )
                        .composite(
                            ::scale_info::build::Fields::unnamed()
                                .field(|f| {
                                    f.ty::<super::Signature>().type_name("super::Signature")
                                }),
                        )
                }
            }
        };
        impl crate::Wraps for Signature {
            type Inner = super::Signature;
        }
        impl From<super::Signature> for Signature {
            fn from(inner: super::Signature) -> Self {
                Self(inner)
            }
        }
        impl From<Signature> for super::Signature {
            fn from(outer: Signature) -> Self {
                outer.0
            }
        }
        impl AsRef<super::Signature> for Signature {
            fn as_ref(&self) -> &super::Signature {
                &self.0
            }
        }
        impl AsMut<super::Signature> for Signature {
            fn as_mut(&mut self) -> &mut super::Signature {
                &mut self.0
            }
        }
        impl crate::CryptoType for Signature {
            type Pair = Pair;
        }
        impl crate::AppCrypto for Signature {
            type Public = Public;
            type Pair = Pair;
            type Signature = Signature;
            const ID: crate::KeyTypeId = sp_core::testing::ED25519;
            const CRYPTO_ID: crate::CryptoTypeId = super::CRYPTO_ID;
        }
        impl crate::Deref for Signature {
            type Target = [u8];
            fn deref(&self) -> &Self::Target {
                self.0.as_ref()
            }
        }
        impl AsRef<[u8]> for Signature {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }
        impl crate::AppSignature for Signature {
            type Generic = super::Signature;
        }
        impl<'a> TryFrom<&'a [u8]> for Signature {
            type Error = ();
            fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
                <super::Signature>::try_from(data).map(Into::into)
            }
        }
        impl TryFrom<crate::Vec<u8>> for Signature {
            type Error = ();
            fn try_from(data: crate::Vec<u8>) -> Result<Self, Self::Error> {
                Self::try_from(&data[..])
            }
        }
        impl Signature {
            /// Convert into wrapped generic signature type.
            pub fn into_inner(self) -> super::Signature {
                self.0
            }
        }
        /// A generic `AppPublic` wrapper type over $pair crypto; this has no specific App.
        pub struct Pair(super::Pair);
        #[automatically_derived]
        impl ::core::clone::Clone for Pair {
            #[inline]
            fn clone(&self) -> Pair {
                Pair(::core::clone::Clone::clone(&self.0))
            }
        }
        impl crate::Wraps for Pair {
            type Inner = super::Pair;
        }
        impl From<super::Pair> for Pair {
            fn from(inner: super::Pair) -> Self {
                Self(inner)
            }
        }
        impl From<Pair> for super::Pair {
            fn from(outer: Pair) -> Self {
                outer.0
            }
        }
        impl AsRef<super::Pair> for Pair {
            fn as_ref(&self) -> &super::Pair {
                &self.0
            }
        }
        impl AsMut<super::Pair> for Pair {
            fn as_mut(&mut self) -> &mut super::Pair {
                &mut self.0
            }
        }
        impl crate::CryptoType for Pair {
            type Pair = Pair;
        }
        impl crate::Pair for Pair {
            type Public = Public;
            type Seed = <super::Pair as crate::Pair>::Seed;
            type Signature = Signature;
            fn generate_with_phrase(
                password: Option<&str>,
            ) -> (Self, String, Self::Seed) {
                let r = <super::Pair>::generate_with_phrase(password);
                (Self(r.0), r.1, r.2)
            }
            fn from_phrase(
                phrase: &str,
                password: Option<&str>,
            ) -> Result<(Self, Self::Seed), crate::SecretStringError> {
                <super::Pair>::from_phrase(phrase, password).map(|r| (Self(r.0), r.1))
            }
            fn derive<Iter: Iterator<Item = crate::DeriveJunction>>(
                &self,
                path: Iter,
                seed: Option<Self::Seed>,
            ) -> Result<(Self, Option<Self::Seed>), crate::DeriveError> {
                self.0.derive(path, seed).map(|x| (Self(x.0), x.1))
            }
            fn from_seed(seed: &Self::Seed) -> Self {
                Self(<super::Pair>::from_seed(seed))
            }
            fn from_seed_slice(seed: &[u8]) -> Result<Self, crate::SecretStringError> {
                <super::Pair>::from_seed_slice(seed).map(Self)
            }
            fn sign(&self, msg: &[u8]) -> Self::Signature {
                Signature(self.0.sign(msg))
            }
            fn verify<M: AsRef<[u8]>>(
                sig: &Self::Signature,
                message: M,
                pubkey: &Self::Public,
            ) -> bool {
                <super::Pair>::verify(&sig.0, message, pubkey.as_ref())
            }
            fn public(&self) -> Self::Public {
                Public(self.0.public())
            }
            fn to_raw_vec(&self) -> crate::Vec<u8> {
                self.0.to_raw_vec()
            }
        }
        impl crate::AppCrypto for Pair {
            type Public = Public;
            type Pair = Pair;
            type Signature = Signature;
            const ID: crate::KeyTypeId = sp_core::testing::ED25519;
            const CRYPTO_ID: crate::CryptoTypeId = super::CRYPTO_ID;
        }
        impl crate::AppPair for Pair {
            type Generic = super::Pair;
        }
        impl Pair {
            /// Convert into wrapped generic key pair type.
            pub fn into_inner(self) -> super::Pair {
                self.0
            }
        }
    }
    #[cfg(feature = "full_crypto")]
    pub use app::Pair as AppPair;
    pub use app::{Public as AppPublic, Signature as AppSignature};
    impl RuntimePublic for Public {
        type Signature = Signature;
        fn all(key_type: KeyTypeId) -> crate::Vec<Self> {
            sp_io::crypto::ed25519_public_keys(key_type)
        }
        fn generate_pair(key_type: KeyTypeId, seed: Option<Vec<u8>>) -> Self {
            sp_io::crypto::ed25519_generate(key_type, seed)
        }
        fn sign<M: AsRef<[u8]>>(
            &self,
            key_type: KeyTypeId,
            msg: &M,
        ) -> Option<Self::Signature> {
            sp_io::crypto::ed25519_sign(key_type, self, msg.as_ref())
        }
        fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
            sp_io::crypto::ed25519_verify(signature, msg.as_ref(), self)
        }
        fn to_raw_vec(&self) -> Vec<u8> {
            sp_core::crypto::ByteArray::to_raw_vec(self)
        }
    }
}
pub mod sr25519 {
    //! Sr25519 crypto types.
    use crate::{KeyTypeId, RuntimePublic};
    use sp_std::vec::Vec;
    pub use sp_core::sr25519::*;
    mod app {
        /// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
        #[codec(crate = crate::codec)]
        pub struct Public(super::Public);
        #[automatically_derived]
        impl ::core::clone::Clone for Public {
            #[inline]
            fn clone(&self) -> Public {
                Public(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Public {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Public {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<super::Public>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Public {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Public {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Public {
            #[inline]
            fn eq(&self, other: &Public) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Public {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Public,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Public {
            #[inline]
            fn cmp(&self, other: &Public) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl crate::codec::Encode for Public {
                fn size_hint(&self) -> usize {
                    crate::codec::Encode::size_hint(&&self.0)
                }
                fn encode_to<
                    __CodecOutputEdqy: crate::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    crate::codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
                }
                fn encode(
                    &self,
                ) -> crate::codec::alloc::vec::Vec<::core::primitive::u8> {
                    crate::codec::Encode::encode(&&self.0)
                }
                fn using_encoded<
                    R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R,
                >(&self, f: F) -> R {
                    crate::codec::Encode::using_encoded(&&self.0, f)
                }
            }
            #[automatically_derived]
            impl crate::codec::EncodeLike for Public {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl crate::codec::Decode for Public {
                fn decode<__CodecInputEdqy: crate::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, crate::codec::Error> {
                    ::core::result::Result::Ok(
                        Public({
                            let __codec_res_edqy = <super::Public as crate::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Public.0`"),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                    __codec_res_edqy
                                }
                            }
                        }),
                    )
                }
            }
        };
        impl core::fmt::Debug for Public {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Public").field(&self.0).finish()
            }
        }
        const _: () = {
            impl crate::codec::MaxEncodedLen for Public {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize.saturating_add(<super::Public>::max_encoded_len())
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for Public {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "Public",
                                "sp_application_crypto::sr25519::app",
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .docs(
                            &[
                                "A generic `AppPublic` wrapper type over $public crypto; this has no specific App.",
                            ],
                        )
                        .composite(
                            ::scale_info::build::Fields::unnamed()
                                .field(|f| {
                                    f.ty::<super::Public>().type_name("super::Public")
                                }),
                        )
                }
            }
        };
        impl crate::Wraps for Public {
            type Inner = super::Public;
        }
        impl From<super::Public> for Public {
            fn from(inner: super::Public) -> Self {
                Self(inner)
            }
        }
        impl From<Public> for super::Public {
            fn from(outer: Public) -> Self {
                outer.0
            }
        }
        impl AsRef<super::Public> for Public {
            fn as_ref(&self) -> &super::Public {
                &self.0
            }
        }
        impl AsMut<super::Public> for Public {
            fn as_mut(&mut self) -> &mut super::Public {
                &mut self.0
            }
        }
        impl crate::CryptoType for Public {
            type Pair = Pair;
        }
        impl crate::AppCrypto for Public {
            type Public = Public;
            type Pair = Pair;
            type Signature = Signature;
            const ID: crate::KeyTypeId = sp_core::testing::SR25519;
            const CRYPTO_ID: crate::CryptoTypeId = super::CRYPTO_ID;
        }
        impl crate::Derive for Public {
            fn derive<Iter: Iterator<Item = crate::DeriveJunction>>(
                &self,
                path: Iter,
            ) -> Option<Self> {
                self.0.derive(path).map(Self)
            }
        }
        impl core::fmt::Display for Public {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                use crate::Ss58Codec;
                f.write_fmt(format_args!("{0}", self.0.to_ss58check()))
            }
        }
        impl crate::serde::Serialize for Public {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> core::result::Result<S::Ok, S::Error>
            where
                S: crate::serde::Serializer,
            {
                use crate::Ss58Codec;
                serializer.serialize_str(&self.to_ss58check())
            }
        }
        impl<'de> crate::serde::Deserialize<'de> for Public {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: crate::serde::Deserializer<'de>,
            {
                use crate::{module_format_string_prelude::*, Ss58Codec};
                Public::from_ss58check(&String::deserialize(deserializer)?)
                    .map_err(|e| crate::serde::de::Error::custom({
                        let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                        res
                    }))
            }
        }
        impl AsRef<[u8]> for Public {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }
        impl AsMut<[u8]> for Public {
            fn as_mut(&mut self) -> &mut [u8] {
                self.0.as_mut()
            }
        }
        impl crate::ByteArray for Public {
            const LEN: usize = <super::Public>::LEN;
        }
        impl crate::Public for Public {}
        impl crate::AppPublic for Public {
            type Generic = super::Public;
        }
        impl<'a> TryFrom<&'a [u8]> for Public {
            type Error = ();
            fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
                <super::Public>::try_from(data).map(Into::into)
            }
        }
        impl Public {
            /// Convert into wrapped generic public key type.
            pub fn into_inner(self) -> super::Public {
                self.0
            }
        }
        /// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
        pub struct Signature(super::Signature);
        #[automatically_derived]
        impl ::core::hash::Hash for Signature {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Signature {
            #[inline]
            fn clone(&self) -> Signature {
                Signature(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Signature {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Signature {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<super::Signature>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Signature {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Signature {
            #[inline]
            fn eq(&self, other: &Signature) -> bool {
                self.0 == other.0
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for Signature {
                fn size_hint(&self) -> usize {
                    ::codec::Encode::size_hint(&&self.0)
                }
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    ::codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
                }
                fn encode(&self) -> ::codec::alloc::vec::Vec<::core::primitive::u8> {
                    ::codec::Encode::encode(&&self.0)
                }
                fn using_encoded<
                    R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R,
                >(&self, f: F) -> R {
                    ::codec::Encode::using_encoded(&&self.0, f)
                }
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for Signature {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for Signature {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    ::core::result::Result::Ok(
                        Signature({
                            let __codec_res_edqy = <super::Signature as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Signature.0`"),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                    __codec_res_edqy
                                }
                            }
                        }),
                    )
                }
            }
        };
        impl core::fmt::Debug for Signature {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Signature").field(&self.0).finish()
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for Signature {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "Signature",
                                "sp_application_crypto::sr25519::app",
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .docs(
                            &[
                                "A generic `AppPublic` wrapper type over $public crypto; this has no specific App.",
                            ],
                        )
                        .composite(
                            ::scale_info::build::Fields::unnamed()
                                .field(|f| {
                                    f.ty::<super::Signature>().type_name("super::Signature")
                                }),
                        )
                }
            }
        };
        impl crate::Wraps for Signature {
            type Inner = super::Signature;
        }
        impl From<super::Signature> for Signature {
            fn from(inner: super::Signature) -> Self {
                Self(inner)
            }
        }
        impl From<Signature> for super::Signature {
            fn from(outer: Signature) -> Self {
                outer.0
            }
        }
        impl AsRef<super::Signature> for Signature {
            fn as_ref(&self) -> &super::Signature {
                &self.0
            }
        }
        impl AsMut<super::Signature> for Signature {
            fn as_mut(&mut self) -> &mut super::Signature {
                &mut self.0
            }
        }
        impl crate::CryptoType for Signature {
            type Pair = Pair;
        }
        impl crate::AppCrypto for Signature {
            type Public = Public;
            type Pair = Pair;
            type Signature = Signature;
            const ID: crate::KeyTypeId = sp_core::testing::SR25519;
            const CRYPTO_ID: crate::CryptoTypeId = super::CRYPTO_ID;
        }
        impl crate::Deref for Signature {
            type Target = [u8];
            fn deref(&self) -> &Self::Target {
                self.0.as_ref()
            }
        }
        impl AsRef<[u8]> for Signature {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }
        impl crate::AppSignature for Signature {
            type Generic = super::Signature;
        }
        impl<'a> TryFrom<&'a [u8]> for Signature {
            type Error = ();
            fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
                <super::Signature>::try_from(data).map(Into::into)
            }
        }
        impl TryFrom<crate::Vec<u8>> for Signature {
            type Error = ();
            fn try_from(data: crate::Vec<u8>) -> Result<Self, Self::Error> {
                Self::try_from(&data[..])
            }
        }
        impl Signature {
            /// Convert into wrapped generic signature type.
            pub fn into_inner(self) -> super::Signature {
                self.0
            }
        }
        /// A generic `AppPublic` wrapper type over $pair crypto; this has no specific App.
        pub struct Pair(super::Pair);
        #[automatically_derived]
        impl ::core::clone::Clone for Pair {
            #[inline]
            fn clone(&self) -> Pair {
                Pair(::core::clone::Clone::clone(&self.0))
            }
        }
        impl crate::Wraps for Pair {
            type Inner = super::Pair;
        }
        impl From<super::Pair> for Pair {
            fn from(inner: super::Pair) -> Self {
                Self(inner)
            }
        }
        impl From<Pair> for super::Pair {
            fn from(outer: Pair) -> Self {
                outer.0
            }
        }
        impl AsRef<super::Pair> for Pair {
            fn as_ref(&self) -> &super::Pair {
                &self.0
            }
        }
        impl AsMut<super::Pair> for Pair {
            fn as_mut(&mut self) -> &mut super::Pair {
                &mut self.0
            }
        }
        impl crate::CryptoType for Pair {
            type Pair = Pair;
        }
        impl crate::Pair for Pair {
            type Public = Public;
            type Seed = <super::Pair as crate::Pair>::Seed;
            type Signature = Signature;
            fn generate_with_phrase(
                password: Option<&str>,
            ) -> (Self, String, Self::Seed) {
                let r = <super::Pair>::generate_with_phrase(password);
                (Self(r.0), r.1, r.2)
            }
            fn from_phrase(
                phrase: &str,
                password: Option<&str>,
            ) -> Result<(Self, Self::Seed), crate::SecretStringError> {
                <super::Pair>::from_phrase(phrase, password).map(|r| (Self(r.0), r.1))
            }
            fn derive<Iter: Iterator<Item = crate::DeriveJunction>>(
                &self,
                path: Iter,
                seed: Option<Self::Seed>,
            ) -> Result<(Self, Option<Self::Seed>), crate::DeriveError> {
                self.0.derive(path, seed).map(|x| (Self(x.0), x.1))
            }
            fn from_seed(seed: &Self::Seed) -> Self {
                Self(<super::Pair>::from_seed(seed))
            }
            fn from_seed_slice(seed: &[u8]) -> Result<Self, crate::SecretStringError> {
                <super::Pair>::from_seed_slice(seed).map(Self)
            }
            fn sign(&self, msg: &[u8]) -> Self::Signature {
                Signature(self.0.sign(msg))
            }
            fn verify<M: AsRef<[u8]>>(
                sig: &Self::Signature,
                message: M,
                pubkey: &Self::Public,
            ) -> bool {
                <super::Pair>::verify(&sig.0, message, pubkey.as_ref())
            }
            fn public(&self) -> Self::Public {
                Public(self.0.public())
            }
            fn to_raw_vec(&self) -> crate::Vec<u8> {
                self.0.to_raw_vec()
            }
        }
        impl crate::AppCrypto for Pair {
            type Public = Public;
            type Pair = Pair;
            type Signature = Signature;
            const ID: crate::KeyTypeId = sp_core::testing::SR25519;
            const CRYPTO_ID: crate::CryptoTypeId = super::CRYPTO_ID;
        }
        impl crate::AppPair for Pair {
            type Generic = super::Pair;
        }
        impl Pair {
            /// Convert into wrapped generic key pair type.
            pub fn into_inner(self) -> super::Pair {
                self.0
            }
        }
    }
    #[cfg(feature = "full_crypto")]
    pub use app::Pair as AppPair;
    pub use app::{Public as AppPublic, Signature as AppSignature};
    impl RuntimePublic for Public {
        type Signature = Signature;
        fn all(key_type: KeyTypeId) -> crate::Vec<Self> {
            sp_io::crypto::sr25519_public_keys(key_type)
        }
        fn generate_pair(key_type: KeyTypeId, seed: Option<Vec<u8>>) -> Self {
            sp_io::crypto::sr25519_generate(key_type, seed)
        }
        fn sign<M: AsRef<[u8]>>(
            &self,
            key_type: KeyTypeId,
            msg: &M,
        ) -> Option<Self::Signature> {
            sp_io::crypto::sr25519_sign(key_type, self, msg.as_ref())
        }
        fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
            sp_io::crypto::sr25519_verify(signature, msg.as_ref(), self)
        }
        fn to_raw_vec(&self) -> Vec<u8> {
            sp_core::crypto::ByteArray::to_raw_vec(self)
        }
    }
}
mod traits {
    use codec::Codec;
    use scale_info::TypeInfo;
    #[cfg(feature = "full_crypto")]
    use sp_core::crypto::Pair;
    use sp_core::crypto::{CryptoType, CryptoTypeId, IsWrappedBy, KeyTypeId, Public};
    use sp_std::{fmt::Debug, vec::Vec};
    /// Application-specific cryptographic object.
    ///
    /// Combines all the core types and constants that are defined by a particular
    /// cryptographic scheme when it is used in a specific application domain.
    ///
    /// Typically, the implementers of this trait are its associated types themselves.
    /// This provides a convenient way to access generic information about the scheme
    /// given any of the associated types.
    pub trait AppCrypto: 'static + Sized + CryptoType {
        /// Identifier for application-specific key type.
        const ID: KeyTypeId;
        /// Identifier of the crypto type of this application-specific key type.
        const CRYPTO_ID: CryptoTypeId;
        /// The corresponding public key type in this application scheme.
        type Public: AppPublic;
        /// The corresponding signature type in this application scheme.
        type Signature: AppSignature;
        /// The corresponding key pair type in this application scheme.
        #[cfg(feature = "full_crypto")]
        type Pair: AppPair;
    }
    /// Type which implements Hash in std, not when no-std (std variant).
    #[cfg(any(feature = "std", feature = "full_crypto"))]
    pub trait MaybeHash: sp_std::hash::Hash {}
    #[cfg(any(feature = "std", feature = "full_crypto"))]
    impl<T: sp_std::hash::Hash> MaybeHash for T {}
    /// Application-specific key pair.
    #[cfg(feature = "full_crypto")]
    pub trait AppPair: AppCrypto + Pair<
            Public = <Self as AppCrypto>::Public,
            Signature = <Self as AppCrypto>::Signature,
        > {
        /// The wrapped type which is just a plain instance of `Pair`.
        type Generic: IsWrappedBy<Self>
            + Pair<Public = <<Self as AppCrypto>::Public as AppPublic>::Generic>
            + Pair<
                Signature = <<Self as AppCrypto>::Signature as AppSignature>::Generic,
            >;
    }
    /// Application-specific public key.
    pub trait AppPublic: AppCrypto + Public + Debug + MaybeHash + Codec {
        /// The wrapped type which is just a plain instance of `Public`.
        type Generic: IsWrappedBy<Self> + Public + Debug + MaybeHash + Codec;
    }
    /// Application-specific signature.
    pub trait AppSignature: AppCrypto + Eq + PartialEq + Debug + Clone {
        /// The wrapped type which is just a plain instance of `Signature`.
        type Generic: IsWrappedBy<Self> + Eq + PartialEq + Debug;
    }
    /// Runtime interface for a public key.
    pub trait RuntimePublic: Sized {
        /// The signature that will be generated when signing with the corresponding private key.
        type Signature: Debug + Eq + PartialEq + Clone;
        /// Returns all public keys for the given key type in the keystore.
        fn all(key_type: KeyTypeId) -> crate::Vec<Self>;
        /// Generate a public/private pair for the given key type with an optional `seed` and
        /// store it in the keystore.
        ///
        /// The `seed` needs to be valid utf8.
        ///
        /// Returns the generated public key.
        fn generate_pair(key_type: KeyTypeId, seed: Option<Vec<u8>>) -> Self;
        /// Sign the given message with the corresponding private key of this public key.
        ///
        /// The private key will be requested from the keystore using the given key type.
        ///
        /// Returns the signature or `None` if the private key could not be found or some other error
        /// occurred.
        fn sign<M: AsRef<[u8]>>(
            &self,
            key_type: KeyTypeId,
            msg: &M,
        ) -> Option<Self::Signature>;
        /// Verify that the given signature matches the given message using this public key.
        fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool;
        /// Returns `Self` as raw vec.
        fn to_raw_vec(&self) -> Vec<u8>;
    }
    /// Runtime interface for an application's public key.
    pub trait RuntimeAppPublic: Sized {
        /// An identifier for this application-specific key type.
        const ID: KeyTypeId;
        /// The signature that will be generated when signing with the corresponding private key.
        type Signature: Debug + Eq + PartialEq + Clone + TypeInfo + Codec;
        /// Returns all public keys for this application in the keystore.
        fn all() -> crate::Vec<Self>;
        /// Generate a public/private pair with an optional `seed` and store it in the keystore.
        ///
        /// The `seed` needs to be valid utf8.
        ///
        /// Returns the generated public key.
        fn generate_pair(seed: Option<Vec<u8>>) -> Self;
        /// Sign the given message with the corresponding private key of this public key.
        ///
        /// The private key will be requested from the keystore.
        ///
        /// Returns the signature or `None` if the private key could not be found or some other error
        /// occurred.
        fn sign<M: AsRef<[u8]>>(&self, msg: &M) -> Option<Self::Signature>;
        /// Verify that the given signature matches the given message using this public key.
        fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool;
        /// Returns `Self` as raw vec.
        fn to_raw_vec(&self) -> Vec<u8>;
    }
    impl<T> RuntimeAppPublic for T
    where
        T: AppPublic + AsRef<<T as AppPublic>::Generic>,
        <T as AppPublic>::Generic: RuntimePublic,
        <T as AppCrypto>::Signature: TypeInfo + Codec
            + From<<<T as AppPublic>::Generic as RuntimePublic>::Signature>
            + AsRef<<<T as AppPublic>::Generic as RuntimePublic>::Signature>,
    {
        const ID: KeyTypeId = <T as AppCrypto>::ID;
        type Signature = <T as AppCrypto>::Signature;
        fn all() -> crate::Vec<Self> {
            <<T as AppPublic>::Generic as RuntimePublic>::all(Self::ID)
                .into_iter()
                .map(|p| p.into())
                .collect()
        }
        fn generate_pair(seed: Option<Vec<u8>>) -> Self {
            <<T as AppPublic>::Generic as RuntimePublic>::generate_pair(Self::ID, seed)
                .into()
        }
        fn sign<M: AsRef<[u8]>>(&self, msg: &M) -> Option<Self::Signature> {
            <<T as AppPublic>::Generic as RuntimePublic>::sign(
                    self.as_ref(),
                    Self::ID,
                    msg,
                )
                .map(|s| s.into())
        }
        fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
            <<T as AppPublic>::Generic as RuntimePublic>::verify(
                self.as_ref(),
                msg,
                signature.as_ref(),
            )
        }
        fn to_raw_vec(&self) -> Vec<u8> {
            <<T as AppPublic>::Generic as RuntimePublic>::to_raw_vec(self.as_ref())
        }
    }
    /// Something that is bound to a fixed [`RuntimeAppPublic`].
    pub trait BoundToRuntimeAppPublic {
        /// The [`RuntimeAppPublic`] this type is bound to.
        type Public: RuntimeAppPublic;
    }
    impl<T: RuntimeAppPublic> BoundToRuntimeAppPublic for T {
        type Public = Self;
    }
}
pub use traits::*;
#[doc(hidden)]
pub mod module_format_string_prelude {
    #[cfg(feature = "std")]
    pub use std::{format, string::String};
}
