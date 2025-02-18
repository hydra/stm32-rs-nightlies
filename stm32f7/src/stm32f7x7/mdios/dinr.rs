///Register `DINR%s` reader
pub struct R(crate::R<DINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN` reader - Input data received from MDIO Master during write frames
pub type DIN_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register %s
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr](index.html) module
pub struct DINR_SPEC;
impl crate::RegisterSpec for DINR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr::R](R) reader structure
impl crate::Readable for DINR_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR%s to value 0
impl crate::Resettable for DINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
