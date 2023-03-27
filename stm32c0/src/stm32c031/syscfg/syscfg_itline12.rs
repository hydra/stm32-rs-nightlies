///Register `SYSCFG_ITLINE12` reader
pub struct R(crate::R<SYSCFG_ITLINE12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE12_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ADC` reader - ADC interrupt request pending
pub type ADC_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - ADC interrupt request pending
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 1) != 0)
    }
}
///SYSCFG interrupt line 12 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [syscfg_itline12](index.html) module
pub struct SYSCFG_ITLINE12_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE12_SPEC {
    type Ux = u32;
}
///`read()` method returns [syscfg_itline12::R](R) reader structure
impl crate::Readable for SYSCFG_ITLINE12_SPEC {
    type Reader = R;
}
///`reset()` method sets SYSCFG_ITLINE12 to value 0
impl crate::Resettable for SYSCFG_ITLINE12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
