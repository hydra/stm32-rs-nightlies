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
///Field `FRNUM` reader - FRNUM
pub type FRNUM_R = crate::FieldReader<u16, u16>;
///Field `FTREM` reader - FTREM
pub type FTREM_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - FRNUM
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - FTREM
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.
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
