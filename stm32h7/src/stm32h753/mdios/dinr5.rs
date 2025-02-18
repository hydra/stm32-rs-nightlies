///Register `DINR5` reader
pub struct R(crate::R<DINR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR5_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN5` reader - Input data received from MDIO Master during write frames
pub type DIN5_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din5(&self) -> DIN5_R {
        DIN5_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 5
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr5](index.html) module
pub struct DINR5_SPEC;
impl crate::RegisterSpec for DINR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr5::R](R) reader structure
impl crate::Readable for DINR5_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR5 to value 0
impl crate::Resettable for DINR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
