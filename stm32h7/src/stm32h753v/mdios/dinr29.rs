///Register `DINR29` reader
pub struct R(crate::R<DINR29_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR29_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR29_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR29_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN29` reader - Input data received from MDIO Master during write frames
pub type DIN29_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din29(&self) -> DIN29_R {
        DIN29_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 29
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr29](index.html) module
pub struct DINR29_SPEC;
impl crate::RegisterSpec for DINR29_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr29::R](R) reader structure
impl crate::Readable for DINR29_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR29 to value 0
impl crate::Resettable for DINR29_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
