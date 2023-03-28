///Register `LCVCIDR` reader
pub struct R(crate::R<LCVCIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCVCIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCVCIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCVCIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `VCID` reader - Virtual Channel ID
pub type VCID_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:1 - Virtual Channel ID
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
///DSI Host LTDC Current VCID Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lcvcidr](index.html) module
pub struct LCVCIDR_SPEC;
impl crate::RegisterSpec for LCVCIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lcvcidr::R](R) reader structure
impl crate::Readable for LCVCIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets LCVCIDR to value 0
impl crate::Resettable for LCVCIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
