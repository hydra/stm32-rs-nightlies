///Register `L1CKCR` reader
pub struct R(crate::R<L1CKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1CKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1CKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1CKCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `L1CKCR` writer
pub struct W(crate::W<L1CKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1CKCR_SPEC>;
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
impl From<crate::W<L1CKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1CKCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CKBLUE` reader - Color Key Blue value
pub type CKBLUE_R = crate::FieldReader<u8, u8>;
///Field `CKBLUE` writer - Color Key Blue value
pub type CKBLUE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, L1CKCR_SPEC, u8, u8, 8, O>;
///Field `CKGREEN` reader - Color Key Green value
pub type CKGREEN_R = crate::FieldReader<u8, u8>;
///Field `CKGREEN` writer - Color Key Green value
pub type CKGREEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, L1CKCR_SPEC, u8, u8, 8, O>;
///Field `CKRED` reader - Color Key Red value
pub type CKRED_R = crate::FieldReader<u8, u8>;
///Field `CKRED` writer - Color Key Red value
pub type CKRED_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, L1CKCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Color Key Blue value
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Color Key Green value
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Color Key Red value
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Color Key Blue value
    #[inline(always)]
    #[must_use]
    pub fn ckblue(&mut self) -> CKBLUE_W<0> {
        CKBLUE_W::new(self)
    }
    ///Bits 8:15 - Color Key Green value
    #[inline(always)]
    #[must_use]
    pub fn ckgreen(&mut self) -> CKGREEN_W<8> {
        CKGREEN_W::new(self)
    }
    ///Bits 16:23 - Color Key Red value
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
///LTDC Layer Color Keying Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1ckcr](index.html) module
pub struct L1CKCR_SPEC;
impl crate::RegisterSpec for L1CKCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [l1ckcr::R](R) reader structure
impl crate::Readable for L1CKCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [l1ckcr::W](W) writer structure
impl crate::Writable for L1CKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets L1CKCR to value 0
impl crate::Resettable for L1CKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
