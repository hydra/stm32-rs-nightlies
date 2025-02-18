///Register `TTCPT` reader
pub struct R(crate::R<TTCPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTCPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTCPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTCPT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CCV` reader - Cycle Count Value
pub type CCV_R = crate::FieldReader<u8, u8>;
///Field `SWV` reader - Stop Watch Value
pub type SWV_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:5 - Cycle Count Value
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 16:31 - Stop Watch Value
    #[inline(always)]
    pub fn swv(&self) -> SWV_R {
        SWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///FDCAN TT Capture Time Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ttcpt](index.html) module
pub struct TTCPT_SPEC;
impl crate::RegisterSpec for TTCPT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ttcpt::R](R) reader structure
impl crate::Readable for TTCPT_SPEC {
    type Reader = R;
}
///`reset()` method sets TTCPT to value 0
impl crate::Resettable for TTCPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
