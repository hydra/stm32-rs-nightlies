///Register `HDPEXTR` reader
pub struct R(crate::R<HDPEXTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDPEXTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDPEXTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDPEXTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HDPEXTR` writer
pub struct W(crate::W<HDPEXTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDPEXTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HDPEXTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDPEXTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HDP1_EXT` reader - HDP area extension in 8�Kbytes sectors in Bank1. Extension is added after the HDP1_END sector (included).
pub type HDP1_EXT_R = crate::FieldReader<u8, u8>;
///Field `HDP1_EXT` writer - HDP area extension in 8�Kbytes sectors in Bank1. Extension is added after the HDP1_END sector (included).
pub type HDP1_EXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HDPEXTR_SPEC, u8, u8, 7, O>;
///Field `HDP2_EXT` reader - HDP area extension in 8�Kbytes sectors in bank 2. Extension is added after the HDP2_END sector (included).
pub type HDP2_EXT_R = crate::FieldReader<u8, u8>;
///Field `HDP2_EXT` writer - HDP area extension in 8�Kbytes sectors in bank 2. Extension is added after the HDP2_END sector (included).
pub type HDP2_EXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HDPEXTR_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:6 - HDP area extension in 8�Kbytes sectors in Bank1. Extension is added after the HDP1_END sector (included).
    #[inline(always)]
    pub fn hdp1_ext(&self) -> HDP1_EXT_R {
        HDP1_EXT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - HDP area extension in 8�Kbytes sectors in bank 2. Extension is added after the HDP2_END sector (included).
    #[inline(always)]
    pub fn hdp2_ext(&self) -> HDP2_EXT_R {
        HDP2_EXT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - HDP area extension in 8�Kbytes sectors in Bank1. Extension is added after the HDP1_END sector (included).
    #[inline(always)]
    #[must_use]
    pub fn hdp1_ext(&mut self) -> HDP1_EXT_W<0> {
        HDP1_EXT_W::new(self)
    }
    ///Bits 16:22 - HDP area extension in 8�Kbytes sectors in bank 2. Extension is added after the HDP2_END sector (included).
    #[inline(always)]
    #[must_use]
    pub fn hdp2_ext(&mut self) -> HDP2_EXT_W<16> {
        HDP2_EXT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH HDP extension register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hdpextr](index.html) module
pub struct HDPEXTR_SPEC;
impl crate::RegisterSpec for HDPEXTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hdpextr::R](R) reader structure
impl crate::Readable for HDPEXTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hdpextr::W](W) writer structure
impl crate::Writable for HDPEXTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HDPEXTR to value 0
impl crate::Resettable for HDPEXTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
