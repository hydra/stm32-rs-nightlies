///Register `DINR9` reader
pub struct R(crate::R<DINR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR9_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN9` reader - Input data received from MDIO Master during write frames
pub type DIN9_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din9(&self) -> DIN9_R {
        DIN9_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 9
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr9](index.html) module
pub struct DINR9_SPEC;
impl crate::RegisterSpec for DINR9_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr9::R](R) reader structure
impl crate::Readable for DINR9_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR9 to value 0
impl crate::Resettable for DINR9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
