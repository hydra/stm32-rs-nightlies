///Register `RGSR` reader
pub struct R(crate::R<RGSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `OF0` reader - Trigger overrun event flag
pub type OF0_R = crate::BitReader<OF0_A>;
///Trigger overrun event flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OF0_A {
    ///0: No new trigger event occured on DMA request generator channel x, before the request counter underrun
    NoTrigger = 0,
    ///1: New trigger event occured on DMA request generator channel x, before the request counter underrun
    Trigger = 1,
}
impl From<OF0_A> for bool {
    #[inline(always)]
    fn from(variant: OF0_A) -> Self {
        variant as u8 != 0
    }
}
impl OF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OF0_A {
        match self.bits {
            false => OF0_A::NoTrigger,
            true => OF0_A::Trigger,
        }
    }
    ///Checks if the value of the field is `NoTrigger`
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == OF0_A::NoTrigger
    }
    ///Checks if the value of the field is `Trigger`
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == OF0_A::Trigger
    }
}
///Field `OF1` reader - Trigger overrun event flag
pub use OF0_R as OF1_R;
///Field `OF2` reader - Trigger overrun event flag
pub use OF0_R as OF2_R;
///Field `OF3` reader - Trigger overrun event flag
pub use OF0_R as OF3_R;
impl R {
    ///Bit 0 - Trigger overrun event flag
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Trigger overrun event flag
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Trigger overrun event flag
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Trigger overrun event flag
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
///request generator interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rgsr](index.html) module
pub struct RGSR_SPEC;
impl crate::RegisterSpec for RGSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rgsr::R](R) reader structure
impl crate::Readable for RGSR_SPEC {
    type Reader = R;
}
///`reset()` method sets RGSR to value 0
impl crate::Resettable for RGSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
