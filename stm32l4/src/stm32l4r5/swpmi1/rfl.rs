///Register `RFL` reader
pub struct R(crate::R<RFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFL_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RFL` reader - Receive frame length
pub type RFL_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:4 - Receive frame length
    #[inline(always)]
    pub fn rfl(&self) -> RFL_R {
        RFL_R::new((self.bits & 0x1f) as u8)
    }
}
///SWPMI Receive Frame Length register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rfl](index.html) module
pub struct RFL_SPEC;
impl crate::RegisterSpec for RFL_SPEC {
    type Ux = u32;
}
///`read()` method returns [rfl::R](R) reader structure
impl crate::Readable for RFL_SPEC {
    type Reader = R;
}
///`reset()` method sets RFL to value 0
impl crate::Resettable for RFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
