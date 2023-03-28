///Register `PWR_CSR1` reader
pub struct R(crate::R<PWR_CSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CSR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PVDO` reader - PVDO
pub type PVDO_R = crate::BitReader<bool>;
///Field `AVDO` reader - AVDO
pub type AVDO_R = crate::BitReader<bool>;
impl R {
    ///Bit 4 - PVDO
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - AVDO
    #[inline(always)]
    pub fn avdo(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 16) & 1) != 0)
    }
}
///Reset on any system reset.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_csr1](index.html) module
pub struct PWR_CSR1_SPEC;
impl crate::RegisterSpec for PWR_CSR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_csr1::R](R) reader structure
impl crate::Readable for PWR_CSR1_SPEC {
    type Reader = R;
}
///`reset()` method sets PWR_CSR1 to value 0
impl crate::Resettable for PWR_CSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
