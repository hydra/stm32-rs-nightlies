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
pub type OF0_R = crate::BitReader<bool>;
///Field `OF1` reader - Trigger overrun event flag
pub type OF1_R = crate::BitReader<bool>;
///Field `OF2` reader - Trigger overrun event flag
pub type OF2_R = crate::BitReader<bool>;
///Field `OF3` reader - Trigger overrun event flag
pub type OF3_R = crate::BitReader<bool>;
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
