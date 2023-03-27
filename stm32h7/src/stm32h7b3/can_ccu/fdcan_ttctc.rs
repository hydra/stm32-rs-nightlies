///Register `FDCAN_TTCTC` reader
pub struct R(crate::R<FDCAN_TTCTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTCTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTCTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTCTC_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CT` reader - Cycle Time
pub type CT_R = crate::FieldReader<u16, u16>;
///Field `CC` reader - Cycle Count
pub type CC_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:15 - Cycle Time
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:21 - Cycle Count
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
///FDCAN TT Cycle Time and Count Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_ttctc](index.html) module
pub struct FDCAN_TTCTC_SPEC;
impl crate::RegisterSpec for FDCAN_TTCTC_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_ttctc::R](R) reader structure
impl crate::Readable for FDCAN_TTCTC_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TTCTC to value 0
impl crate::Resettable for FDCAN_TTCTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
