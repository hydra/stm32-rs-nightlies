///Register `TXFQS` reader
pub struct R(crate::R<TXFQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFQS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TFFL` reader - TFFL
pub type TFFL_R = crate::FieldReader<u8, u8>;
///Field `TFGI` reader - TFGI
pub type TFGI_R = crate::FieldReader<u8, u8>;
///Field `TFQPI` reader - TFQPI
pub type TFQPI_R = crate::FieldReader<u8, u8>;
///Field `TFQF` reader - TFQF
pub type TFQF_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:2 - TFFL
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:9 - TFGI
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - TFQPI
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 21 - TFQF
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
///The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated).
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txfqs](index.html) module
pub struct TXFQS_SPEC;
impl crate::RegisterSpec for TXFQS_SPEC {
    type Ux = u32;
}
///`read()` method returns [txfqs::R](R) reader structure
impl crate::Readable for TXFQS_SPEC {
    type Reader = R;
}
///`reset()` method sets TXFQS to value 0
impl crate::Resettable for TXFQS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
