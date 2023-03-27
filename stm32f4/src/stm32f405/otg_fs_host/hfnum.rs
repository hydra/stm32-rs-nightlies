///Register `HFNUM` reader
pub struct R(crate::R<HFNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFNUM_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FRNUM` reader - Frame number
pub type FRNUM_R = crate::FieldReader<u16, u16>;
///Field `FTREM` reader - Frame time remaining
pub type FTREM_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Frame number
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Frame time remaining
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hfnum](index.html) module
pub struct HFNUM_SPEC;
impl crate::RegisterSpec for HFNUM_SPEC {
    type Ux = u32;
}
///`read()` method returns [hfnum::R](R) reader structure
impl crate::Readable for HFNUM_SPEC {
    type Reader = R;
}
///`reset()` method sets HFNUM to value 0x3fff
impl crate::Resettable for HFNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}
