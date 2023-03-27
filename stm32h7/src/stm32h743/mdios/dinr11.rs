///Register `DINR11` reader
pub struct R(crate::R<DINR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR11_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN11` reader - Input data received from MDIO Master during write frames
pub type DIN11_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din11(&self) -> DIN11_R {
        DIN11_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 11
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr11](index.html) module
pub struct DINR11_SPEC;
impl crate::RegisterSpec for DINR11_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr11::R](R) reader structure
impl crate::Readable for DINR11_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR11 to value 0
impl crate::Resettable for DINR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
