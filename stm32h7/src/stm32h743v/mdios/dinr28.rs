///Register `DINR28` reader
pub struct R(crate::R<DINR28_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR28_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR28_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR28_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN28` reader - Input data received from MDIO Master during write frames
pub type DIN28_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din28(&self) -> DIN28_R {
        DIN28_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 28
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr28](index.html) module
pub struct DINR28_SPEC;
impl crate::RegisterSpec for DINR28_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr28::R](R) reader structure
impl crate::Readable for DINR28_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR28 to value 0
impl crate::Resettable for DINR28_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
