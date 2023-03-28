///Register `DOUT` reader
pub struct R(crate::R<DOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DATAOUT` reader - Data output
pub type DATAOUT_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Data output
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new(self.bits)
    }
}
///data output register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dout](index.html) module
pub struct DOUT_SPEC;
impl crate::RegisterSpec for DOUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [dout::R](R) reader structure
impl crate::Readable for DOUT_SPEC {
    type Reader = R;
}
///`reset()` method sets DOUT to value 0
impl crate::Resettable for DOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
