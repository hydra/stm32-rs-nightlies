///Register `ITLINE15` reader
pub struct R(crate::R<ITLINE15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE15_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TIM2` reader - TIM2
pub type TIM2_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TIM2
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 15 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline15](index.html) module
pub struct ITLINE15_SPEC;
impl crate::RegisterSpec for ITLINE15_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline15::R](R) reader structure
impl crate::Readable for ITLINE15_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE15 to value 0
impl crate::Resettable for ITLINE15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
