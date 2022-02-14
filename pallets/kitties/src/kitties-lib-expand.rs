#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;
///
///			The module that hosts all the
///			[FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame)
///			types needed to add this pallet to a
///			[runtime](https://substrate.dev/docs/en/knowledgebase/runtime/).
///
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use frame_support::{
        traits::{Randomness, Currency},
    };
    pub struct Kitty(pub [u8; 16]);
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for Kitty {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                _parity_scale_codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
            }
            fn encode(&self) -> _parity_scale_codec::alloc::vec::Vec<u8> {
                _parity_scale_codec::Encode::encode(&&self.0)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                _parity_scale_codec::Encode::using_encoded(&&self.0, f)
            }
        }
        impl _parity_scale_codec::EncodeLike for Kitty {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for Kitty {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(Kitty({
                    let __codec_res_edqy =
                        <[u8; 16] as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `Kitty.0`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                }))
            }
        }
    };
    type KittyIndex = u32;
    /// Configure the pallet by specifying the parameters and types on which it depends.
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
    }
    ///
    ///			The [pallet](https://substrate.dev/docs/en/knowledgebase/runtime/pallets) implementing
    ///			the on-chain logic.
    ///
    pub struct Pallet<T>(frame_support::sp_std::marker::PhantomData<(T)>);
    const _: () = {
        impl<T> core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        impl<T> core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        impl<T> core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    #[allow(type_alias_bounds)]
    pub type KittiesCount<T> = StorageValue<_GeneratedPrefixForStorageKittiesCount<T>, u32>;
    #[allow(type_alias_bounds)]
    pub type Kitties<T> = StorageMap<
        _GeneratedPrefixForStorageKitties<T>,
        Blake2_128Concat,
        KittyIndex,
        Option<Kitty>,
        ValueQuery,
    >;
    #[allow(type_alias_bounds)]
    pub type Owner<T: Config> = StorageMap<
        _GeneratedPrefixForStorageOwner<T>,
        Blake2_128Concat,
        KittyIndex,
        Option<T::AccountId>,
        ValueQuery,
    >;
    ///
    ///			The [event](https://substrate.dev/docs/en/knowledgebase/runtime/events) emitted
    ///			by this pallet.
    ///
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        KittyCreate(T::AccountId, KittyIndex),
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
    }
    const _: () = {
        impl<T: Config> core::clone::Clone for Event<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::KittyCreate(ref _0, ref _1) => Self::KittyCreate(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Event<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Event<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::KittyCreate(_0, _1), Self::KittyCreate(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::KittyCreate { .. }, Self::__Ignore { .. }) => false,
                    (Self::__Ignore { .. }, Self::KittyCreate { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::fmt::Debug for Event<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::KittyCreate(ref _0, ref _1) => fmt
                        .debug_tuple("Event::KittyCreate")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Event::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                }
            }
        }
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Config> _parity_scale_codec::Encode for Event<T>
        where
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
        {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::KittyCreate(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> _parity_scale_codec::EncodeLike for Event<T>
        where
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
        {
        }
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Config> _parity_scale_codec::Decode for Event<T>
        where
            T::AccountId: _parity_scale_codec::Decode,
            T::AccountId: _parity_scale_codec::Decode,
        {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::KittyCreate(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyCreate.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <KittyIndex as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyCreate.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    _ => ::core::result::Result::Err(
                        "Could not decode `Event`, variant doesn\'t exist".into(),
                    ),
                }
            }
        }
    };
    ///
    ///			Custom [dispatch errors](https://substrate.dev/docs/en/knowledgebase/runtime/errors)
    ///			of this pallet.
    ///
    pub enum Error<T> {
        #[doc(hidden)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        KittiesCountOverflow,
    }
    impl<T: Config> Pallet<T> {
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let kitty_id = match Self::kitties_count() {
                Some(id) => {
                    {
                        if !(id != KittyIndex::maxValue()) {
                            {
                                return Err(Error::<T>::KittiesCountOverflow.into());
                            };
                        }
                    };
                    id
                }
                None => 1,
            };
            let dna = Self::random_value(&who);
            Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
            Owner::<T>::insert(kitty_id, Some(who.clone()));
            KittiesCount::<T>::put(kitty_id + 1);
            Self::deposit_event(Event::KittyCreate(who, kitty_id));
            Ok(())
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn random_value(&sender: &T::AccountId) -> [u8; 16] {
            let pay_load = (
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            pay_load.using_encode(blake2_128)
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn module_constants_metadata(
        ) -> &'static [frame_support::dispatch::ModuleConstantMetadata] {
            &[]
        }
    }
    impl<T: Config> frame_support::error::ModuleErrorMetadata for Pallet<T> {
        fn metadata() -> &'static [frame_support::error::ErrorMetadata] {
            <Error<T> as frame_support::error::ModuleErrorMetadata>::metadata()
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetPalletVersion for Pallet<T> {
        fn current_version() -> frame_support::traits::PalletVersion {
            frame_support::traits::PalletVersion {
                major: 3u16,
                minor: 0u8,
                patch: 0u8,
            }
        }
        fn storage_version() -> Option<frame_support::traits::PalletVersion> {
            let key = frame_support::traits::PalletVersion::storage_key::<
                <T as frame_system::Config>::PalletInfo,
                Self,
            >()
            .expect("Every active pallet has a name in the runtime; qed");
            frame_support::storage::unhashed::get(&key)
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            frame_support::traits::PalletVersion {
                major: 3u16,
                minor: 0u8,
                patch: 0u8,
            }
            .put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
    }
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains one variant per dispatchable that can be called by an extrinsic.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T,)>,
            frame_support::Never,
        ),
        create(),
    }
    const _: () = {
        impl<T: Config> core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Call::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::create() => fmt.debug_tuple("Call::create").finish(),
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                    Self::create() => Self::create(),
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::create(), Self::create()) => true,
                    (Self::__Ignore { .. }, Self::create { .. }) => false,
                    (Self::create { .. }, Self::__Ignore { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        #[allow(non_camel_case_types)]
        impl<T: Config> _parity_scale_codec::Encode for Call<T> {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::create() => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> _parity_scale_codec::EncodeLike for Call<T> {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        #[allow(non_camel_case_types)]
        impl<T: Config> _parity_scale_codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::create())
                    }
                    _ => ::core::result::Result::Err(
                        "Could not decode `Call`, variant doesn\'t exist".into(),
                    ),
                }
            }
        }
    };
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::create() => {
                    let __pallet_base_weight = 0;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<()>>::weigh_data(
                        &__pallet_base_weight,
                        (),
                    );
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<()>>::classify_dispatch(
                            &__pallet_base_weight,
                            (),
                        );
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<()>>::pays_fee(
                        &__pallet_base_weight,
                        (),
                    );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__Ignore cannot be used",) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::create(..) => "create",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem cannot be used.",) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["create"]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type Origin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::Origin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Self::create() => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "create",
                                    "pallet_kitties::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/kitties/src/lib.rs"),
                                    Some(11u32),
                                    Some("pallet_kitties::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::create(origin)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    {
                        {
                            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                                &["internal error: entered unreachable code: "],
                                &match (&"__PhantomItem cannot be used.",) {
                                    _args => [::core::fmt::ArgumentV1::new(
                                        _args.0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ))
                        }
                    };
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type Call = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        #[allow(dead_code)]
        pub fn call_functions() -> &'static [frame_support::dispatch::FunctionMetadata] {
            &[frame_support::dispatch::FunctionMetadata {
                name: frame_support::dispatch::DecodeDifferent::Encode("create"),
                arguments: frame_support::dispatch::DecodeDifferent::Encode(&[]),
                documentation: frame_support::dispatch::DecodeDifferent::Encode(&[]),
            }]
        }
    }
    impl<T: Config> frame_support::sp_std::fmt::Debug for Error<T> {
        fn fmt(
            &self,
            f: &mut frame_support::sp_std::fmt::Formatter<'_>,
        ) -> frame_support::sp_std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        pub fn as_u8(&self) -> u8 {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"`__Ignore` can never be constructed",) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
                Self::NoneValue => 0usize as u8,
                Self::KittiesCountOverflow => 1usize as u8,
            }
        }
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"`__Ignore` can never be constructed",) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
                Self::NoneValue => "NoneValue",
                Self::KittiesCountOverflow => "KittiesCountOverflow",
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str {
            err.as_str()
        }
    }
    impl<T: Config> From<Error<T>> for frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            let index = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: index :: < Pallet < T > > () . expect ("Every active module has an index in the runtime; qed") as u8 ;
            frame_support::sp_runtime::DispatchError::Module {
                index,
                error: err.as_u8(),
                message: Some(err.as_str()),
            }
        }
    }
    impl<T: Config> frame_support::error::ModuleErrorMetadata for Error<T> {
        fn metadata() -> &'static [frame_support::error::ErrorMetadata] {
            &[
                frame_support::error::ErrorMetadata {
                    name: frame_support::error::DecodeDifferent::Encode("NoneValue"),
                    documentation: frame_support::error::DecodeDifferent::Encode(&[
                        " Error names should be descriptive.",
                    ]),
                },
                frame_support::error::ErrorMetadata {
                    name: frame_support::error::DecodeDifferent::Encode("KittiesCountOverflow"),
                    documentation: frame_support::error::DecodeDifferent::Encode(&[
                        " Errors should have helpful documentation associated with them.",
                    ]),
                },
            ]
        }
    }
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_1 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::Event as From<Event<T>>>::from(event);
            let event =
                <<T as Config>::Event as Into<<T as frame_system::Config>::Event>>::into(event);
            <frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Event<T> {
        #[allow(dead_code)]
        #[doc(hidden)]
        pub fn metadata() -> &'static [frame_support::event::EventMetadata] {
            & [frame_support :: event :: EventMetadata { name : frame_support :: event :: DecodeDifferent :: Encode ("KittyCreate") , arguments : frame_support :: event :: DecodeDifferent :: Encode (& ["AccountId" , "KittyIndex"]) , documentation : frame_support :: event :: DecodeDifferent :: Encode (& [" Event documentation should end with an array that provides descriptive names for event" , " parameters. [something, who]"]) , }]
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::metadata::StorageMetadata {
            frame_support :: metadata :: StorageMetadata { prefix : frame_support :: metadata :: DecodeDifferent :: Encode (< < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("Every active pallet has a name in the runtime; qed")) , entries : frame_support :: metadata :: DecodeDifferent :: Encode (& [frame_support :: metadata :: StorageEntryMetadata { name : frame_support :: metadata :: DecodeDifferent :: Encode (< KittiesCount < T > as frame_support :: storage :: types :: StorageValueMetadata > :: NAME) , modifier : < KittiesCount < T > as frame_support :: storage :: types :: StorageValueMetadata > :: MODIFIER , ty : frame_support :: metadata :: StorageEntryType :: Plain (frame_support :: metadata :: DecodeDifferent :: Encode ("u32")) , default : frame_support :: metadata :: DecodeDifferent :: Encode (< KittiesCount < T > as frame_support :: storage :: types :: StorageValueMetadata > :: DEFAULT) , documentation : frame_support :: metadata :: DecodeDifferent :: Encode (& []) , } , frame_support :: metadata :: StorageEntryMetadata { name : frame_support :: metadata :: DecodeDifferent :: Encode (< Kitties < T > as frame_support :: storage :: types :: StorageMapMetadata > :: NAME) , modifier : < Kitties < T > as frame_support :: storage :: types :: StorageMapMetadata > :: MODIFIER , ty : frame_support :: metadata :: StorageEntryType :: Map { hasher : < Kitties < T > as frame_support :: storage :: types :: StorageMapMetadata > :: HASHER , key : frame_support :: metadata :: DecodeDifferent :: Encode ("KittyIndex") , value : frame_support :: metadata :: DecodeDifferent :: Encode ("Option<Kitty>") , unused : false , } , default : frame_support :: metadata :: DecodeDifferent :: Encode (< Kitties < T > as frame_support :: storage :: types :: StorageMapMetadata > :: DEFAULT) , documentation : frame_support :: metadata :: DecodeDifferent :: Encode (& []) , } , frame_support :: metadata :: StorageEntryMetadata { name : frame_support :: metadata :: DecodeDifferent :: Encode (< Owner < T > as frame_support :: storage :: types :: StorageMapMetadata > :: NAME) , modifier : < Owner < T > as frame_support :: storage :: types :: StorageMapMetadata > :: MODIFIER , ty : frame_support :: metadata :: StorageEntryType :: Map { hasher : < Owner < T > as frame_support :: storage :: types :: StorageMapMetadata > :: HASHER , key : frame_support :: metadata :: DecodeDifferent :: Encode ("KittyIndex") , value : frame_support :: metadata :: DecodeDifferent :: Encode ("Option<T::AccountId>") , unused : false , } , default : frame_support :: metadata :: DecodeDifferent :: Encode (< Owner < T > as frame_support :: storage :: types :: StorageMapMetadata > :: DEFAULT) , documentation : frame_support :: metadata :: DecodeDifferent :: Encode (& []) , }]) , }
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn kitties_count() -> Option<u32> {
            <KittiesCount<T> as frame_support::storage::StorageValue<u32>>::get()
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn kitties<KArg>(k: KArg) -> Option<Kitty>
        where
            KArg: frame_support::codec::EncodeLike<KittyIndex>,
        {
            <Kitties<T> as frame_support::storage::StorageMap<KittyIndex, Option<Kitty>>>::get(k)
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn owner<KArg>(k: KArg) -> Option<T::AccountId>
        where
            KArg: frame_support::codec::EncodeLike<KittyIndex>,
        {
            <Owner<T> as frame_support::storage::StorageMap<KittyIndex, Option<T::AccountId>>>::get(
                k,
            )
        }
    }
    pub struct _GeneratedPrefixForStorageKittiesCount<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
        for _GeneratedPrefixForStorageKittiesCount<T>
    {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "KittiesCount";
    }
    pub struct _GeneratedPrefixForStorageKitties<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for _GeneratedPrefixForStorageKitties<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "Kitties";
    }
    pub struct _GeneratedPrefixForStorageOwner<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for _GeneratedPrefixForStorageOwner<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "Owner";
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_2 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    pub(super) trait Store {
        type KittiesCount;
        type Kitties;
        type Owner;
    }
    impl<T: Config> Store for Pallet<T> {
        type KittiesCount = KittiesCount<T>;
        type Kitties = Kitties<T>;
        type Owner = Owner<T>;
    }
    impl<T: Config> frame_support::traits::Hooks<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
    }
    impl<T: Config> frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/kitties/src/lib.rs"),
                            Some(11u32),
                            Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_finalize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_idle(
            n: <T as frame_system::Config>::BlockNumber,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_idle (n , remaining_weight)
        }
    }
    impl<T: Config> frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_initialize(
            n: <T as frame_system::Config>::BlockNumber,
        ) -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/kitties/src/lib.rs"),
                            Some(11u32),
                            Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_initialize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/kitties/src/lib.rs"),
                            Some(11u32),
                            Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let new_storage_version = frame_support::traits::PalletVersion {
                major: 3u16,
                minor: 0u8,
                patch: 0u8,
            };
            let pallet_name = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Self > () . unwrap_or ("<unknown pallet name>") ;
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "\u{2705} no migration for ",
                                ", setting storage version to ",
                            ],
                            &match (&pallet_name, &new_storage_version) {
                                _args => [
                                    ::core::fmt::ArgumentV1::new(
                                        _args.0,
                                        ::core::fmt::Display::fmt,
                                    ),
                                    ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ),
                        lvl,
                        &(
                            frame_support::LOG_TARGET,
                            "pallet_kitties::pallet",
                            "pallets/kitties/src/lib.rs",
                            11u32,
                        ),
                    );
                }
            };
            let result = <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::on_runtime_upgrade();
            new_storage_version.put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
            let additional_write =
                <<T as frame_system::Config>::DbWeight as frame_support::traits::Get<_>>::get()
                    .writes(1);
            result.saturating_add(additional_write)
        }
    }
    impl<T: Config> frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: offchain_worker (n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: integrity_test ()
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_3 as is_genesis_config_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_4 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_5 as is_validate_unsigned_part_defined;
    }
}
