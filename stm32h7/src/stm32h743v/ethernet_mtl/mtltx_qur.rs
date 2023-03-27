///Register `MTLTxQUR` reader
pub struct R(crate::R<MTLTX_QUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLTX_QUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLTX_QUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLTX_QUR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `UFFRMCNT` reader - Underflow Packet Counter
pub type UFFRMCNT_R = crate::FieldReader<u16, u16>;
///Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter
pub type UFCNTOVF_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:10 - Underflow Packet Counter
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 11 - Overflow Bit for Underflow Packet Counter
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
///Tx queue underflow register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mtltx_qur](index.html) module
pub struct MTLTX_QUR_SPEC;
impl crate::RegisterSpec for MTLTX_QUR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mtltx_qur::R](R) reader structure
impl crate::Readable for MTLTX_QUR_SPEC {
    type Reader = R;
}
///`reset()` method sets MTLTxQUR to value 0
impl crate::Resettable for MTLTX_QUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
