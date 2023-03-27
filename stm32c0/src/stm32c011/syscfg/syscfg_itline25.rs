///Register `SYSCFG_ITLINE25` reader
pub struct R(crate::R<SYSCFG_ITLINE25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE25_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SPI1` reader - SPI1 interrupt request pending
pub type SPI1_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - SPI1 interrupt request pending
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new((self.bits & 1) != 0)
    }
}
///SYSCFG interrupt line 25 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [syscfg_itline25](index.html) module
pub struct SYSCFG_ITLINE25_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE25_SPEC {
    type Ux = u32;
}
///`read()` method returns [syscfg_itline25::R](R) reader structure
impl crate::Readable for SYSCFG_ITLINE25_SPEC {
    type Reader = R;
}
///`reset()` method sets SYSCFG_ITLINE25 to value 0
impl crate::Resettable for SYSCFG_ITLINE25_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
