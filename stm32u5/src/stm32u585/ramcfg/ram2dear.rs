///Register `RAM2DEAR` reader
pub struct R(crate::R<RAM2DEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM2DEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM2DEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM2DEAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EDEA` reader - EDEA
pub type EDEA_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - EDEA
    #[inline(always)]
    pub fn edea(&self) -> EDEA_R {
        EDEA_R::new(self.bits)
    }
}
///RAMCFG RAM x ECC double error address register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram2dear](index.html) module
pub struct RAM2DEAR_SPEC;
impl crate::RegisterSpec for RAM2DEAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ram2dear::R](R) reader structure
impl crate::Readable for RAM2DEAR_SPEC {
    type Reader = R;
}
///`reset()` method sets RAM2DEAR to value 0
impl crate::Resettable for RAM2DEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
