///Register `MMCRFCECR` reader
pub struct R(crate::R<MMCRFCECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRFCECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRFCECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRFCECR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RFCFC` reader - Received frames CRC error counter
pub type RFCFC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Received frames CRC error counter
    #[inline(always)]
    pub fn rfcfc(&self) -> RFCFC_R {
        RFCFC_R::new(self.bits)
    }
}
///Ethernet MMC received frames with CRC error counter register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmcrfcecr](index.html) module
pub struct MMCRFCECR_SPEC;
impl crate::RegisterSpec for MMCRFCECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmcrfcecr::R](R) reader structure
impl crate::Readable for MMCRFCECR_SPEC {
    type Reader = R;
}
///`reset()` method sets MMCRFCECR to value 0
impl crate::Resettable for MMCRFCECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
