///Register `LTDC_L2CKCR` reader
pub struct R(crate::R<LTDC_L2CKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L2CKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L2CKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L2CKCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LTDC_L2CKCR` writer
pub struct W(crate::W<LTDC_L2CKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L2CKCR_SPEC>;
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
impl From<crate::W<LTDC_L2CKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L2CKCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CKBLUE` reader - CKBLUE
pub type CKBLUE_R = crate::FieldReader<u8, u8>;
///Field `CKBLUE` writer - CKBLUE
pub type CKBLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L2CKCR_SPEC, u8, u8, 8, O>;
///Field `CKGREEN` reader - CKGREEN
pub type CKGREEN_R = crate::FieldReader<u8, u8>;
///Field `CKGREEN` writer - CKGREEN
pub type CKGREEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L2CKCR_SPEC, u8, u8, 8, O>;
///Field `CKRED` reader - CKRED
pub type CKRED_R = crate::FieldReader<u8, u8>;
///Field `CKRED` writer - CKRED
pub type CKRED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L2CKCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - CKBLUE
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - CKGREEN
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - CKRED
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - CKBLUE
    #[inline(always)]
    #[must_use]
    pub fn ckblue(&mut self) -> CKBLUE_W<0> {
        CKBLUE_W::new(self)
    }
    ///Bits 8:15 - CKGREEN
    #[inline(always)]
    #[must_use]
    pub fn ckgreen(&mut self) -> CKGREEN_W<8> {
        CKGREEN_W::new(self)
    }
    ///Bits 16:23 - CKRED
    #[inline(always)]
    #[must_use]
    pub fn ckred(&mut self) -> CKRED_W<16> {
        CKRED_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register defines the color key value (RGB), that is used by the color keying.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ltdc_l2ckcr](index.html) module
pub struct LTDC_L2CKCR_SPEC;
impl crate::RegisterSpec for LTDC_L2CKCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ltdc_l2ckcr::R](R) reader structure
impl crate::Readable for LTDC_L2CKCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ltdc_l2ckcr::W](W) writer structure
impl crate::Writable for LTDC_L2CKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LTDC_L2CKCR to value 0
impl crate::Resettable for LTDC_L2CKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
