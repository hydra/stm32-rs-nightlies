///Register `SYSCFG_ITLINE4` reader
pub struct R(crate::R<SYSCFG_ITLINE4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RCC` reader - Reset and clock control interrupt request pending
pub type RCC_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Reset and clock control interrupt request pending
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new((self.bits & 1) != 0)
    }
}
///SYSCFG interrupt line 4 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [syscfg_itline4](index.html) module
pub struct SYSCFG_ITLINE4_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE4_SPEC {
    type Ux = u32;
}
///`read()` method returns [syscfg_itline4::R](R) reader structure
impl crate::Readable for SYSCFG_ITLINE4_SPEC {
    type Reader = R;
}
///`reset()` method sets SYSCFG_ITLINE4 to value 0
impl crate::Resettable for SYSCFG_ITLINE4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
