use ::core::{cell::UnsafeCell, marker::PhantomData, mem::MaybeUninit};

#[repr(transparent)]
pub struct ManuallyInit<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    _marker: PhantomData<T>,
}

impl<T> ManuallyInit<T> {
    /// Creates a new uninitialized cell.
    #[inline]
    #[must_use]
    pub const fn new() -> ManuallyInit<T> {
        ManuallyInit {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            _marker: PhantomData,
        }
    }

    // /// Creates a new initialized cell with the given value.
    // #[inline]
    // #[must_use]
    // pub const fn new_with(value: T) -> ManuallyInit<T> {
    //     ManuallyInit {
    //         value: UnsafeCell::new(MaybeUninit::new(value)),
    //         _marker: PhantomData,
    //     }
    // }

    /// Initializes the cell with the given value.
    ///
    /// # Safety
    /// The cell must be uninitialized when calling this method.
    #[inline]
    pub const unsafe fn init(&self, value: T) { unsafe { (&mut *self.value.get()).write(value) }; }

    /// Gets the reference to the underlying value.
    ///
    /// # Safety
    /// The cell must be initialized when calling this method.
    #[inline]
    pub const unsafe fn get(&self) -> &T { unsafe { (&*self.value.get()).assume_init_ref() } }

    // /// Gets the mutable reference to the underlying value.
    // ///
    // /// # Safety
    // /// The cell must be initialized when calling this method.
    // #[inline]
    // pub const unsafe fn get_mut(&mut self) -> &mut T {
    //     unsafe { (&mut *self.value.get()).assume_init_mut() }
    // }

    // /// Consumes the cell, returning the wrapped value.
    // ///
    // /// # Safety
    // /// The cell must be initialized when calling this method.
    // #[inline]
    // pub const unsafe fn into_inner(self) -> T {
    //     unsafe { self.value.into_inner().assume_init() }
    // }

    // /// Takes the value out of the cell, leaving it uninitialized.
    // ///
    // /// # Safety
    // /// The cell must be initialized when calling this method.
    // #[inline]
    // pub const unsafe fn take(&mut self) -> T {
    //     unsafe { (&mut *self.value.get()).assume_init_read() }
    // }
}

unsafe impl<T: Send> Send for ManuallyInit<T> {}
unsafe impl<T: Sync> Sync for ManuallyInit<T> {}

impl<T> ::core::ops::Deref for ManuallyInit<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target { unsafe { self.get() } }
}
