///Register `SYSCFG_ITLINE16` reader
pub struct R(crate::R<SYSCFG_ITLINE16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE16_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TIM3` reader - Timer 3 interrupt request pending
pub type TIM3_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Timer 3 interrupt request pending
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new((self.bits & 1) != 0)
    }
}
///SYSCFG interrupt line 16 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [syscfg_itline16](index.html) module
pub struct SYSCFG_ITLINE16_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE16_SPEC {
    type Ux = u32;
}
///`read()` method returns [syscfg_itline16::R](R) reader structure
impl crate::Readable for SYSCFG_ITLINE16_SPEC {
    type Reader = R;
}
///`reset()` method sets SYSCFG_ITLINE16 to value 0
impl crate::Resettable for SYSCFG_ITLINE16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
