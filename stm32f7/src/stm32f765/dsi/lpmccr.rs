///Register `LPMCCR` reader
pub struct R(crate::R<LPMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `VLPSIZE` reader - VACT Largest Packet Size
pub type VLPSIZE_R = crate::FieldReader<u8, u8>;
///Field `LPSIZE` reader - Largest Packet Size
pub type LPSIZE_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - VACT Largest Packet Size
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Largest Packet Size
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
///DSI Host Low-Power mode Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpmccr](index.html) module
pub struct LPMCCR_SPEC;
impl crate::RegisterSpec for LPMCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpmccr::R](R) reader structure
impl crate::Readable for LPMCCR_SPEC {
    type Reader = R;
}
///`reset()` method sets LPMCCR to value 0
impl crate::Resettable for LPMCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
