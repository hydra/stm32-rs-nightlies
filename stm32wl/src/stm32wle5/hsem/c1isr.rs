///Register `C1ISR` reader
pub struct R(crate::R<C1ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ISF0` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub type ISF0_R = crate::BitReader<ISF0_A>;
///Interrupt(N) semaphore n status bit before enable (mask)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF0_A {
    ///0: No interrupt pending
    NotPending = 0,
    ///1: Interrupt pending
    Pending = 1,
}
impl From<ISF0_A> for bool {
    #[inline(always)]
    fn from(variant: ISF0_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ISF0_A {
        match self.bits {
            false => ISF0_A::NotPending,
            true => ISF0_A::Pending,
        }
    }
    ///Checks if the value of the field is `NotPending`
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ISF0_A::NotPending
    }
    ///Checks if the value of the field is `Pending`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ISF0_A::Pending
    }
}
///Field `ISF1` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF1_R;
///Field `ISF2` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF2_R;
///Field `ISF3` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF3_R;
///Field `ISF4` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF4_R;
///Field `ISF5` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF5_R;
///Field `ISF6` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF6_R;
///Field `ISF7` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF7_R;
///Field `ISF8` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF8_R;
///Field `ISF9` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF9_R;
///Field `ISF10` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF10_R;
///Field `ISF11` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF11_R;
///Field `ISF12` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF12_R;
///Field `ISF13` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF13_R;
///Field `ISF14` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF14_R;
///Field `ISF15` reader - Interrupt(N) semaphore n status bit before enable (mask)
pub use ISF0_R as ISF15_R;
impl R {
    ///Bit 0 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf0(&self) -> ISF0_R {
        ISF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf1(&self) -> ISF1_R {
        ISF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf2(&self) -> ISF2_R {
        ISF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf3(&self) -> ISF3_R {
        ISF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf4(&self) -> ISF4_R {
        ISF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf5(&self) -> ISF5_R {
        ISF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf6(&self) -> ISF6_R {
        ISF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf7(&self) -> ISF7_R {
        ISF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf8(&self) -> ISF8_R {
        ISF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf9(&self) -> ISF9_R {
        ISF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf10(&self) -> ISF10_R {
        ISF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf11(&self) -> ISF11_R {
        ISF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf12(&self) -> ISF12_R {
        ISF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf13(&self) -> ISF13_R {
        ISF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf14(&self) -> ISF14_R {
        ISF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt(N) semaphore n status bit before enable (mask)
    #[inline(always)]
    pub fn isf15(&self) -> ISF15_R {
        ISF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///HSEM Interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1isr](index.html) module
pub struct C1ISR_SPEC;
impl crate::RegisterSpec for C1ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1isr::R](R) reader structure
impl crate::Readable for C1ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets C1ISR to value 0
impl crate::Resettable for C1ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
