///Register `PWR_BDSR` reader
pub struct R(crate::R<PWR_BDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_BDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_BDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_BDSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `VBATH` reader - Backup domain voltage level monitoring versus high threshold
pub type VBATH_R = crate::BitReader<bool>;
///Field `TEMPL` reader - Temperature level monitoring versus low threshold
pub type TEMPL_R = crate::BitReader<bool>;
///Field `TEMPH` reader - Temperature level monitoring versus high threshold
pub type TEMPH_R = crate::BitReader<bool>;
impl R {
    ///Bit 1 - Backup domain voltage level monitoring versus high threshold
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Temperature level monitoring versus low threshold
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Temperature level monitoring versus high threshold
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 3) & 1) != 0)
    }
}
///PWR Backup domain status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_bdsr](index.html) module
pub struct PWR_BDSR_SPEC;
impl crate::RegisterSpec for PWR_BDSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_bdsr::R](R) reader structure
impl crate::Readable for PWR_BDSR_SPEC {
    type Reader = R;
}
///`reset()` method sets PWR_BDSR to value 0
impl crate::Resettable for PWR_BDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
