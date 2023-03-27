///Register `ALRMBSSR` reader
pub struct R(crate::R<ALRMBSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRMBSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRMBSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRMBSSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ALRMBSSR` writer
pub struct W(crate::W<ALRMBSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRMBSSR_SPEC>;
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
impl From<crate::W<ALRMBSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRMBSSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SS` reader - Sub seconds value
pub type SS_R = crate::FieldReader<u16, u16>;
///Field `SS` writer - Sub seconds value
pub type SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMBSSR_SPEC, u16, u16, 15, O>;
///Field `MASKSS` reader - Mask the most-significant bits starting at this bit
pub type MASKSS_R = crate::FieldReader<u8, u8>;
///Field `MASKSS` writer - Mask the most-significant bits starting at this bit
pub type MASKSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMBSSR_SPEC, u8, u8, 6, O>;
///Field `SSCLR` reader - SSCLR
pub type SSCLR_R = crate::BitReader<bool>;
///Field `SSCLR` writer - SSCLR
pub type SSCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMBSSR_SPEC, bool, O>;
impl R {
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 24:29 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 31 - SSCLR
    #[inline(always)]
    pub fn ssclr(&self) -> SSCLR_R {
        SSCLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<0> {
        SS_W::new(self)
    }
    ///Bits 24:29 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    #[must_use]
    pub fn maskss(&mut self) -> MASKSS_W<24> {
        MASKSS_W::new(self)
    }
    ///Bit 31 - SSCLR
    #[inline(always)]
    #[must_use]
    pub fn ssclr(&mut self) -> SSCLR_W<31> {
        SSCLR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///alarm B sub second register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [alrmbssr](index.html) module
pub struct ALRMBSSR_SPEC;
impl crate::RegisterSpec for ALRMBSSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [alrmbssr::R](R) reader structure
impl crate::Readable for ALRMBSSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [alrmbssr::W](W) writer structure
impl crate::Writable for ALRMBSSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ALRMBSSR to value 0
impl crate::Resettable for ALRMBSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
