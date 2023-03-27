///Register `ITLINE19` reader
pub struct R(crate::R<ITLINE19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE19_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TIM14` reader - TIM14
pub type TIM14_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TIM14
    #[inline(always)]
    pub fn tim14(&self) -> TIM14_R {
        TIM14_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 19 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline19](index.html) module
pub struct ITLINE19_SPEC;
impl crate::RegisterSpec for ITLINE19_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline19::R](R) reader structure
impl crate::Readable for ITLINE19_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE19 to value 0
impl crate::Resettable for ITLINE19_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
