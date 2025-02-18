///Register `DINR23` reader
pub struct R(crate::R<DINR23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR23_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN23` reader - Input data received from MDIO Master during write frames
pub type DIN23_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din23(&self) -> DIN23_R {
        DIN23_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 23
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr23](index.html) module
pub struct DINR23_SPEC;
impl crate::RegisterSpec for DINR23_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr23::R](R) reader structure
impl crate::Readable for DINR23_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR23 to value 0
impl crate::Resettable for DINR23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
