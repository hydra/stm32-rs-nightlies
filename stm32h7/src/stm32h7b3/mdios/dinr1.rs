///Register `DINR1` reader
pub struct R(crate::R<DINR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN1` reader - Input data received from MDIO Master during write frames
pub type DIN1_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din1(&self) -> DIN1_R {
        DIN1_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr1](index.html) module
pub struct DINR1_SPEC;
impl crate::RegisterSpec for DINR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr1::R](R) reader structure
impl crate::Readable for DINR1_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR1 to value 0
impl crate::Resettable for DINR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
