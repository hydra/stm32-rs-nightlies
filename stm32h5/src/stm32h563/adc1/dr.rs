///Register `DR` reader
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RDATA` reader - Regular data converted These bits are read-only. They contain the conversion result from the last converted regular channel. The data are left- or right-aligned as described in .
pub type RDATA_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Regular data converted These bits are read-only. They contain the conversion result from the last converted regular channel. The data are left- or right-aligned as described in .
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
///ADC regular data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](index.html) module
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dr::R](R) reader structure
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
