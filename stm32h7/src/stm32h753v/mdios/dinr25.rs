///Register `DINR25` reader
pub struct R(crate::R<DINR25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR25_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN25` reader - Input data received from MDIO Master during write frames
pub type DIN25_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din25(&self) -> DIN25_R {
        DIN25_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 25
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr25](index.html) module
pub struct DINR25_SPEC;
impl crate::RegisterSpec for DINR25_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr25::R](R) reader structure
impl crate::Readable for DINR25_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR25 to value 0
impl crate::Resettable for DINR25_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
