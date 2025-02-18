///Register `DINR20` reader
pub struct R(crate::R<DINR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR20_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN20` reader - Input data received from MDIO Master during write frames
pub type DIN20_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din20(&self) -> DIN20_R {
        DIN20_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 20
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr20](index.html) module
pub struct DINR20_SPEC;
impl crate::RegisterSpec for DINR20_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr20::R](R) reader structure
impl crate::Readable for DINR20_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR20 to value 0
impl crate::Resettable for DINR20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
