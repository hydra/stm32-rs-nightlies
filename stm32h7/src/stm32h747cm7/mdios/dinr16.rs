///Register `DINR16` reader
pub struct R(crate::R<DINR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR16_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN16` reader - Input data received from MDIO Master during write frames
pub type DIN16_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din16(&self) -> DIN16_R {
        DIN16_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 16
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr16](index.html) module
pub struct DINR16_SPEC;
impl crate::RegisterSpec for DINR16_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr16::R](R) reader structure
impl crate::Readable for DINR16_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR16 to value 0
impl crate::Resettable for DINR16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
