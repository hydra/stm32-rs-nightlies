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
///Field `TFFL` reader - Tx FIFO Free Level
pub type TFFL_R = crate::FieldReader<u8, u8>;
///Field `TFGI` reader - TFGI
pub type TFGI_R = crate::FieldReader<u8, u8>;
///Field `TFQPI` reader - Tx FIFO/Queue Put Index
pub type TFQPI_R = crate::FieldReader<u8, u8>;
///Field `TFQF` reader - Tx FIFO/Queue Full
pub type TFQF_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:5 - Tx FIFO Free Level
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:12 - TFGI
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Tx FIFO/Queue Put Index
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 21 - Tx FIFO/Queue Full
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
///FDCAN Tx FIFO/Queue Status Register
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
