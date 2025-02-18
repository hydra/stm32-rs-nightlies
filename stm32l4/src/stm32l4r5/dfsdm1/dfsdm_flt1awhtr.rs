///Register `DFSDM_FLT1AWHTR` reader
pub struct R(crate::R<DFSDM_FLT1AWHTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT1AWHTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT1AWHTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT1AWHTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DFSDM_FLT1AWHTR` writer
pub struct W(crate::W<DFSDM_FLT1AWHTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_FLT1AWHTR_SPEC>;
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
impl From<crate::W<DFSDM_FLT1AWHTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_FLT1AWHTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BKAWH` reader - Break signal assignment to analog watchdog high threshold event
pub type BKAWH_R = crate::FieldReader<u8, u8>;
///Field `BKAWH` writer - Break signal assignment to analog watchdog high threshold event
pub type BKAWH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_FLT1AWHTR_SPEC, u8, u8, 4, O>;
///Field `AWHT` reader - Analog watchdog high threshold
pub type AWHT_R = crate::FieldReader<u32, u32>;
///Field `AWHT` writer - Analog watchdog high threshold
pub type AWHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFSDM_FLT1AWHTR_SPEC, u32, u32, 24, O>;
impl R {
    ///Bits 0:3 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:31 - Analog watchdog high threshold
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:3 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    #[must_use]
    pub fn bkawh(&mut self) -> BKAWH_W<0> {
        BKAWH_W::new(self)
    }
    ///Bits 8:31 - Analog watchdog high threshold
    #[inline(always)]
    #[must_use]
    pub fn awht(&mut self) -> AWHT_W<8> {
        AWHT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1awhtr](index.html) module
pub struct DFSDM_FLT1AWHTR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT1AWHTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm_flt1awhtr::R](R) reader structure
impl crate::Readable for DFSDM_FLT1AWHTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dfsdm_flt1awhtr::W](W) writer structure
impl crate::Writable for DFSDM_FLT1AWHTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DFSDM_FLT1AWHTR to value 0
impl crate::Resettable for DFSDM_FLT1AWHTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
