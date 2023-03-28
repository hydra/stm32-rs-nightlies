///Register `DINR21` reader
pub struct R(crate::R<DINR21_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR21_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR21_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR21_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN21` reader - Input data received from MDIO Master during write frames
pub type DIN21_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din21(&self) -> DIN21_R {
        DIN21_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 21
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr21](index.html) module
pub struct DINR21_SPEC;
impl crate::RegisterSpec for DINR21_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr21::R](R) reader structure
impl crate::Readable for DINR21_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR21 to value 0
impl crate::Resettable for DINR21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
