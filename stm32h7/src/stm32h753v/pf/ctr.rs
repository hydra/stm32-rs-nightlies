///Register `CTR` reader
pub struct R(crate::R<CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `_IminLine` reader - IminLine
pub type _IMIN_LINE_R = crate::FieldReader<u8, u8>;
///Field `DMinLine` reader - DMinLine
pub type DMIN_LINE_R = crate::FieldReader<u8, u8>;
///Field `ERG` reader - ERG
pub type ERG_R = crate::FieldReader<u8, u8>;
///Field `CWG` reader - CWG
pub type CWG_R = crate::FieldReader<u8, u8>;
///Field `Format` reader - Format
pub type FORMAT_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - IminLine
    #[inline(always)]
    pub fn _imin_line(&self) -> _IMIN_LINE_R {
        _IMIN_LINE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 16:19 - DMinLine
    #[inline(always)]
    pub fn dmin_line(&self) -> DMIN_LINE_R {
        DMIN_LINE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - ERG
    #[inline(always)]
    pub fn erg(&self) -> ERG_R {
        ERG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - CWG
    #[inline(always)]
    pub fn cwg(&self) -> CWG_R {
        CWG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 29:31 - Format
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 29) & 7) as u8)
    }
}
///Cache Type register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ctr](index.html) module
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ctr::R](R) reader structure
impl crate::Readable for CTR_SPEC {
    type Reader = R;
}
///`reset()` method sets CTR to value 0x8303_c003
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8303_c003;
}
