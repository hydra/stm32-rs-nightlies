///Register `LPGPIO_IDR` reader
pub struct R(crate::R<LPGPIO_IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPGPIO_IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPGPIO_IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPGPIO_IDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IDy` reader - IDy
pub type IDY_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - IDy
    #[inline(always)]
    pub fn idy(&self) -> IDY_R {
        IDY_R::new((self.bits & 0xffff) as u16)
    }
}
///LPGPIO port input data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpgpio_idr](index.html) module
pub struct LPGPIO_IDR_SPEC;
impl crate::RegisterSpec for LPGPIO_IDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpgpio_idr::R](R) reader structure
impl crate::Readable for LPGPIO_IDR_SPEC {
    type Reader = R;
}
///`reset()` method sets LPGPIO_IDR to value 0
impl crate::Resettable for LPGPIO_IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
