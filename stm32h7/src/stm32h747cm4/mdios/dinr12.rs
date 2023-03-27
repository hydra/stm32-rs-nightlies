///Register `DINR12` reader
pub struct R(crate::R<DINR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR12_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN12` reader - Input data received from MDIO Master during write frames
pub type DIN12_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din12(&self) -> DIN12_R {
        DIN12_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 12
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr12](index.html) module
pub struct DINR12_SPEC;
impl crate::RegisterSpec for DINR12_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr12::R](R) reader structure
impl crate::Readable for DINR12_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR12 to value 0
impl crate::Resettable for DINR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
