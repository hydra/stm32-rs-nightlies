///Register `IWDG_IDR` reader
pub struct R(crate::R<IWDG_IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDG_IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDG_IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDG_IDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
///IWDG identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iwdg_idr](index.html) module
pub struct IWDG_IDR_SPEC;
impl crate::RegisterSpec for IWDG_IDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [iwdg_idr::R](R) reader structure
impl crate::Readable for IWDG_IDR_SPEC {
    type Reader = R;
}
///`reset()` method sets IWDG_IDR to value 0x0012_0041
impl crate::Resettable for IWDG_IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0012_0041;
}
