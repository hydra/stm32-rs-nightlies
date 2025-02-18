///Register `ITLINE20` reader
pub struct R(crate::R<ITLINE20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE20_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TIM15` reader - TIM15
pub type TIM15_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TIM15
    #[inline(always)]
    pub fn tim15(&self) -> TIM15_R {
        TIM15_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 20 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline20](index.html) module
pub struct ITLINE20_SPEC;
impl crate::RegisterSpec for ITLINE20_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline20::R](R) reader structure
impl crate::Readable for ITLINE20_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE20 to value 0
impl crate::Resettable for ITLINE20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
