///Register `FLT1EXMIN` reader
pub struct R(crate::R<FLT1EXMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT1EXMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT1EXMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT1EXMIN_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EXMINCH` reader - Extremes detector minimum data channel
pub type EXMINCH_R = crate::FieldReader<u8, u8>;
///Field `EXMIN` reader - EXMIN
pub type EXMIN_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:2 - Extremes detector minimum data channel
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:31 - EXMIN
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
///Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flt1exmin](index.html) module
pub struct FLT1EXMIN_SPEC;
impl crate::RegisterSpec for FLT1EXMIN_SPEC {
    type Ux = u32;
}
///`read()` method returns [flt1exmin::R](R) reader structure
impl crate::Readable for FLT1EXMIN_SPEC {
    type Reader = R;
}
///`reset()` method sets FLT1EXMIN to value 0x7fff_ff00
impl crate::Resettable for FLT1EXMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fff_ff00;
}
