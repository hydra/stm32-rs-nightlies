///Register `FDCAN_TTLGT` reader
pub struct R(crate::R<FDCAN_TTLGT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTLGT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTLGT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTLGT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LT` reader - Local Time
pub type LT_R = crate::FieldReader<u16, u16>;
///Field `GT` reader - Global Time
pub type GT_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Local Time
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Global Time
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///FDCAN TT Local and Global Time Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_ttlgt](index.html) module
pub struct FDCAN_TTLGT_SPEC;
impl crate::RegisterSpec for FDCAN_TTLGT_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_ttlgt::R](R) reader structure
impl crate::Readable for FDCAN_TTLGT_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TTLGT to value 0
impl crate::Resettable for FDCAN_TTLGT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
