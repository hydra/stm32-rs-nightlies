///Register `DINR4` reader
pub struct R(crate::R<DINR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN4` reader - Input data received from MDIO Master during write frames
pub type DIN4_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din4(&self) -> DIN4_R {
        DIN4_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 4
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr4](index.html) module
pub struct DINR4_SPEC;
impl crate::RegisterSpec for DINR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr4::R](R) reader structure
impl crate::Readable for DINR4_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR4 to value 0
impl crate::Resettable for DINR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
