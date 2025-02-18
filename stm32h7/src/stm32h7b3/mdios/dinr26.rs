///Register `DINR26` reader
pub struct R(crate::R<DINR26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR26_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN26` reader - Input data received from MDIO Master during write frames
pub type DIN26_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din26(&self) -> DIN26_R {
        DIN26_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 26
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr26](index.html) module
pub struct DINR26_SPEC;
impl crate::RegisterSpec for DINR26_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr26::R](R) reader structure
impl crate::Readable for DINR26_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR26 to value 0
impl crate::Resettable for DINR26_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
