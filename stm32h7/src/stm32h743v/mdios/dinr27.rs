///Register `DINR27` reader
pub struct R(crate::R<DINR27_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR27_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR27_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR27_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN27` reader - Input data received from MDIO Master during write frames
pub type DIN27_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din27(&self) -> DIN27_R {
        DIN27_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 27
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr27](index.html) module
pub struct DINR27_SPEC;
impl crate::RegisterSpec for DINR27_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr27::R](R) reader structure
impl crate::Readable for DINR27_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR27 to value 0
impl crate::Resettable for DINR27_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
