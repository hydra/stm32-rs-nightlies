///Register `JDATA13R` reader
pub struct R(crate::R<JDATA13R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDATA13R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDATA13R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDATA13R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `JDATA1` reader - Injected group conversion data for SDADC1
pub type JDATA1_R = crate::FieldReader<u16, u16>;
///Field `JDATA3` reader - Injected group conversion data for SDADC3
pub type JDATA3_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Injected group conversion data for SDADC1
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Injected group conversion data for SDADC3
    #[inline(always)]
    pub fn jdata3(&self) -> JDATA3_R {
        JDATA3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///SDADC1 and SDADC3 injected data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jdata13r](index.html) module
pub struct JDATA13R_SPEC;
impl crate::RegisterSpec for JDATA13R_SPEC {
    type Ux = u32;
}
///`read()` method returns [jdata13r::R](R) reader structure
impl crate::Readable for JDATA13R_SPEC {
    type Reader = R;
}
///`reset()` method sets JDATA13R to value 0
impl crate::Resettable for JDATA13R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
