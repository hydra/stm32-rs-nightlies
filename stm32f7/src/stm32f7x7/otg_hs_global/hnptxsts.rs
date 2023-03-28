///Register `HNPTXSTS` reader
pub struct R(crate::R<HNPTXSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HNPTXSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HNPTXSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HNPTXSTS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NPTXFSAV` reader - Nonperiodic TxFIFO space available
pub type NPTXFSAV_R = crate::FieldReader<u16, u16>;
///Field `NPTQXSAV` reader - Nonperiodic transmit request queue space available
pub type NPTQXSAV_R = crate::FieldReader<u8, u8>;
///Field `NPTXQTOP` reader - Top of the nonperiodic transmit request queue
pub type NPTXQTOP_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:15 - Nonperiodic TxFIFO space available
    #[inline(always)]
    pub fn nptxfsav(&self) -> NPTXFSAV_R {
        NPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Nonperiodic transmit request queue space available
    #[inline(always)]
    pub fn nptqxsav(&self) -> NPTQXSAV_R {
        NPTQXSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:30 - Top of the nonperiodic transmit request queue
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
///OTG_HS nonperiodic transmit FIFO/queue status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hnptxsts](index.html) module
pub struct HNPTXSTS_SPEC;
impl crate::RegisterSpec for HNPTXSTS_SPEC {
    type Ux = u32;
}
///`read()` method returns [hnptxsts::R](R) reader structure
impl crate::Readable for HNPTXSTS_SPEC {
    type Reader = R;
}
///`reset()` method sets HNPTXSTS to value 0x0008_0200
impl crate::Resettable for HNPTXSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0200;
}
