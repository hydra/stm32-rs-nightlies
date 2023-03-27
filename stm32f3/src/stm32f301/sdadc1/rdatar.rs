///Register `RDATAR` reader
pub struct R(crate::R<RDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RDATA` reader - Regular channel conversion data
pub type RDATA_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Regular channel conversion data
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
///data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdatar](index.html) module
pub struct RDATAR_SPEC;
impl crate::RegisterSpec for RDATAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rdatar::R](R) reader structure
impl crate::Readable for RDATAR_SPEC {
    type Reader = R;
}
///`reset()` method sets RDATAR to value 0
impl crate::Resettable for RDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
