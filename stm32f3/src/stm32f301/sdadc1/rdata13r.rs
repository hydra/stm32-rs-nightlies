///Register `RDATA13R` reader
pub struct R(crate::R<RDATA13R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATA13R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATA13R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATA13R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RDATA1` reader - Regular conversion data for SDADC1
pub type RDATA1_R = crate::FieldReader<u16, u16>;
///Field `RDATA3` reader - Regular conversion data for SDADC3
pub type RDATA3_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Regular conversion data for SDADC1
    #[inline(always)]
    pub fn rdata1(&self) -> RDATA1_R {
        RDATA1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Regular conversion data for SDADC3
    #[inline(always)]
    pub fn rdata3(&self) -> RDATA3_R {
        RDATA3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///SDADC1 and SDADC3 regular data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdata13r](index.html) module
pub struct RDATA13R_SPEC;
impl crate::RegisterSpec for RDATA13R_SPEC {
    type Ux = u32;
}
///`read()` method returns [rdata13r::R](R) reader structure
impl crate::Readable for RDATA13R_SPEC {
    type Reader = R;
}
///`reset()` method sets RDATA13R to value 0
impl crate::Resettable for RDATA13R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
