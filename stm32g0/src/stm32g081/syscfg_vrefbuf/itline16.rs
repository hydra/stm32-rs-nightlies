///Register `ITLINE16` reader
pub struct R(crate::R<ITLINE16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE16_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TIM3` reader - TIM3
pub type TIM3_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TIM3
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 16 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline16](index.html) module
pub struct ITLINE16_SPEC;
impl crate::RegisterSpec for ITLINE16_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline16::R](R) reader structure
impl crate::Readable for ITLINE16_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE16 to value 0
impl crate::Resettable for ITLINE16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
