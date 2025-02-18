///Register `TZC_INT_STATUS` reader
pub struct R(crate::R<TZC_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `STATUS` reader - STATUS
pub type STATUS_R = crate::FieldReader<u8, u8>;
///Field `OVERRUN` reader - OVERRUN
pub type OVERRUN_R = crate::FieldReader<u8, u8>;
///Field `OVERLAP` reader - OVERLAP
pub type OVERLAP_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:1 - STATUS
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - OVERRUN
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - OVERLAP
    #[inline(always)]
    pub fn overlap(&self) -> OVERLAP_R {
        OVERLAP_R::new(((self.bits >> 16) & 3) as u8)
    }
}
///Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_int_status](index.html) module
pub struct TZC_INT_STATUS_SPEC;
impl crate::RegisterSpec for TZC_INT_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_int_status::R](R) reader structure
impl crate::Readable for TZC_INT_STATUS_SPEC {
    type Reader = R;
}
///`reset()` method sets TZC_INT_STATUS to value 0
impl crate::Resettable for TZC_INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
