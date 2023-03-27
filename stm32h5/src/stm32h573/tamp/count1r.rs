///Register `COUNT1R` reader
pub struct R(crate::R<COUNT1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT1R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `COUNT` reader - This register is read-only only and is incremented by one when a write access is done to this register. This register cannot roll-over and is frozen when reaching the maximum value.
pub type COUNT_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - This register is read-only only and is incremented by one when a write access is done to this register. This register cannot roll-over and is frozen when reaching the maximum value.
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
///TAMP monotonic counter 1 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [count1r](index.html) module
pub struct COUNT1R_SPEC;
impl crate::RegisterSpec for COUNT1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [count1r::R](R) reader structure
impl crate::Readable for COUNT1R_SPEC {
    type Reader = R;
}
///`reset()` method sets COUNT1R to value 0
impl crate::Resettable for COUNT1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
