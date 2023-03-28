///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SOF0` reader - Synchronization overrun event flag
pub type SOF0_R = crate::BitReader<SOF0_A>;
///Synchronization overrun event flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOF0_A {
    ///0: No synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ
    NoSyncEvent = 0,
    ///1: Synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ
    SyncEvent = 1,
}
impl From<SOF0_A> for bool {
    #[inline(always)]
    fn from(variant: SOF0_A) -> Self {
        variant as u8 != 0
    }
}
impl SOF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SOF0_A {
        match self.bits {
            false => SOF0_A::NoSyncEvent,
            true => SOF0_A::SyncEvent,
        }
    }
    ///Checks if the value of the field is `NoSyncEvent`
    #[inline(always)]
    pub fn is_no_sync_event(&self) -> bool {
        *self == SOF0_A::NoSyncEvent
    }
    ///Checks if the value of the field is `SyncEvent`
    #[inline(always)]
    pub fn is_sync_event(&self) -> bool {
        *self == SOF0_A::SyncEvent
    }
}
///Field `SOF1` reader - Synchronization overrun event flag
pub use SOF0_R as SOF1_R;
///Field `SOF2` reader - Synchronization overrun event flag
pub use SOF0_R as SOF2_R;
///Field `SOF3` reader - Synchronization overrun event flag
pub use SOF0_R as SOF3_R;
///Field `SOF4` reader - Synchronization overrun event flag
pub use SOF0_R as SOF4_R;
///Field `SOF5` reader - Synchronization overrun event flag
pub use SOF0_R as SOF5_R;
///Field `SOF6` reader - Synchronization overrun event flag
pub use SOF0_R as SOF6_R;
///Field `SOF7` reader - Synchronization overrun event flag
pub use SOF0_R as SOF7_R;
///Field `SOF8` reader - Synchronization overrun event flag
pub use SOF0_R as SOF8_R;
///Field `SOF9` reader - Synchronization overrun event flag
pub use SOF0_R as SOF9_R;
///Field `SOF10` reader - Synchronization overrun event flag
pub use SOF0_R as SOF10_R;
///Field `SOF11` reader - Synchronization overrun event flag
pub use SOF0_R as SOF11_R;
///Field `SOF12` reader - Synchronization overrun event flag
pub use SOF0_R as SOF12_R;
///Field `SOF13` reader - Synchronization overrun event flag
pub use SOF0_R as SOF13_R;
impl R {
    ///Bit 0 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof2(&self) -> SOF2_R {
        SOF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof3(&self) -> SOF3_R {
        SOF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof4(&self) -> SOF4_R {
        SOF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof5(&self) -> SOF5_R {
        SOF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof6(&self) -> SOF6_R {
        SOF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof7(&self) -> SOF7_R {
        SOF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof8(&self) -> SOF8_R {
        SOF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof9(&self) -> SOF9_R {
        SOF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof10(&self) -> SOF10_R {
        SOF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof11(&self) -> SOF11_R {
        SOF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof12(&self) -> SOF12_R {
        SOF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof13(&self) -> SOF13_R {
        SOF13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
///channel status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
