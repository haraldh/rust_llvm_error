// SPDX-License-Identifier: Apache-2.0

//! Host <-> Shim Communication

extern "C" {
    /// Extern
    pub static _ENARX_SALLYPORT_START: Block;
}

use crate::addr::{ShimPhysUnencryptedAddr, ShimVirtAddr};
use crate::spin::RwLocked;
use core::convert::TryFrom;
use primordial::Address;
use spinning::Lazy;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Block {
    buf: [u8; 100],
}

/// The static HostCall Mutex
pub static HOST_CALL_ALLOC: Lazy<RwLocked<HostCallAllocator>> = Lazy::new(|| {
    let block_mut: *mut Block = unsafe {
        let address = Address::<u64, Block>::from(&_ENARX_SALLYPORT_START);
        let shim_virt = ShimVirtAddr::<Block>::try_from(address).unwrap();

        ShimPhysUnencryptedAddr::<Block>::try_from(shim_virt)
            .unwrap()
            .into_mut() as *mut _
    };

    let mut hostcall_allocator = HostCallAllocator([None, None]);

    for i in 0..2 {
        (hostcall_allocator.0)[i].replace(unsafe { &mut *block_mut.add(i) });
    }

    RwLocked::<HostCallAllocator>::new(hostcall_allocator)
});

/// Allocator for all `sallyport::Block`
pub struct HostCallAllocator([Option<&'static mut Block>; 2]);
