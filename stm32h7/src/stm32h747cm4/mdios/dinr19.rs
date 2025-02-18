///Register `DINR19` reader
pub struct R(crate::R<DINR19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR19_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN19` reader - Input data received from MDIO Master during write frames
pub type DIN19_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din19(&self) -> DIN19_R {
        DIN19_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 19
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr19](index.html) module
pub struct DINR19_SPEC;
impl crate::RegisterSpec for DINR19_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr19::R](R) reader structure
impl crate::Readable for DINR19_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR19 to value 0
impl crate::Resettable for DINR19_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
