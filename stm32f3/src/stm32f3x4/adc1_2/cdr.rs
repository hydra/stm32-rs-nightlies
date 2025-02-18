///Register `CDR` reader
pub struct R(crate::R<CDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RDATA_MST` reader - Regular data of the master ADC
pub type RDATA_MST_R = crate::FieldReader<u16, u16>;
///Field `RDATA_SLV` reader - Regular data of the slave ADC
pub type RDATA_SLV_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Regular data of the master ADC
    #[inline(always)]
    pub fn rdata_mst(&self) -> RDATA_MST_R {
        RDATA_MST_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Regular data of the slave ADC
    #[inline(always)]
    pub fn rdata_slv(&self) -> RDATA_SLV_R {
        RDATA_SLV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///ADC common regular data register for dual mode
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cdr](index.html) module
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cdr::R](R) reader structure
impl crate::Readable for CDR_SPEC {
    type Reader = R;
}
///`reset()` method sets CDR to value 0
impl crate::Resettable for CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
