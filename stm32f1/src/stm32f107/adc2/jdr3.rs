///Register `JDR3` reader
pub struct R(crate::R<JDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDR3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `JDATA` reader - Injected data
pub type JDATA_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Injected data
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
///injected data register x
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jdr3](index.html) module
pub struct JDR3_SPEC;
impl crate::RegisterSpec for JDR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [jdr3::R](R) reader structure
impl crate::Readable for JDR3_SPEC {
    type Reader = R;
}
///`reset()` method sets JDR3 to value 0
impl crate::Resettable for JDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
