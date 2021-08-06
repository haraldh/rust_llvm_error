// SPDX-License-Identifier: Apache-2.0

//! Some basic address operations

extern "C" {
    /// Extern
    pub static _ENARX_SALLYPORT_START: u8;
    /// Extern
    pub static _ENARX_SALLYPORT_END: u8;
}

use core::convert::TryFrom;
use primordial::Address;

/// Address in the host virtual address space
pub struct HostVirtAddr<U>(Address<u64, U>);

impl<U> HostVirtAddr<U> {
    /// Create a new HostVirtAddr
    ///
    /// # Safety
    /// The caller has to ensure, that the address is in the hosts virtual address space
    pub unsafe fn new(val: Address<u64, U>) -> Self {
        Self(val)
    }
}

impl<U> From<ShimPhysUnencryptedAddr<U>> for HostVirtAddr<U> {
    #[inline(always)]
    fn from(val: ShimPhysUnencryptedAddr<U>) -> Self {
        HostVirtAddr(val.0)
    }
}

/// Address in the shim virtual address space
#[derive(Clone)]
pub struct ShimVirtAddr<U>(Address<u64, U>);

impl<U> TryFrom<Address<u64, U>> for ShimVirtAddr<U> {
    type Error = ();

    #[inline(always)]
    fn try_from(value: Address<u64, U>) -> Result<Self, Self::Error> {
        let value = value.raw();

        Ok(Self(unsafe { Address::unchecked(value) }))
    }
}

/// Address in the shim virtual address space
pub struct ShimPhysUnencryptedAddr<U>(Address<u64, U>);

impl<U> ShimPhysUnencryptedAddr<U> {
    /// Get the raw address
    pub fn raw(self) -> Address<u64, U> {
        self.0
    }

    /// convert to mutable reference
    pub fn into_mut<'a>(self) -> &'a mut U {
        unsafe { &mut *(self.0.raw() as *mut U) }
    }
}

impl<U> TryFrom<ShimVirtAddr<U>> for ShimPhysUnencryptedAddr<U> {
    type Error = ();

    #[inline(always)]
    fn try_from(value: ShimVirtAddr<U>) -> Result<Self, Self::Error> {
        let value = value.0.raw();

        if value >= unsafe { &_ENARX_SALLYPORT_END as *const _ as u64 } {
            return Err(());
        }
        if value < unsafe { &_ENARX_SALLYPORT_START as *const _ as u64 } {
            return Err(());
        }

        let value = value.checked_sub(0xFFFF_FF80_0000_0000).ok_or(())?;

        Ok(Self(unsafe { Address::unchecked(value) }))
    }
}
