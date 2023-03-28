///Register `PCSCNTR` reader
pub struct R(crate::R<PCSCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSCNTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCSCNTR` writer
pub struct W(crate::W<PCSCNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSCNTR_SPEC>;
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
impl From<crate::W<PCSCNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSCNTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSCOUNT` reader - CSCOUNT
pub type CSCOUNT_R = crate::FieldReader<u16, u16>;
///Field `CSCOUNT` writer - CSCOUNT
pub type CSCOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCSCNTR_SPEC, u16, u16, 16, O>;
///Field `CNTB1EN` reader - CNTB1EN
pub type CNTB1EN_R = crate::BitReader<bool>;
///Field `CNTB1EN` writer - CNTB1EN
pub type CNTB1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCSCNTR_SPEC, bool, O>;
///Field `CNTB2EN` reader - CNTB2EN
pub type CNTB2EN_R = crate::BitReader<bool>;
///Field `CNTB2EN` writer - CNTB2EN
pub type CNTB2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCSCNTR_SPEC, bool, O>;
///Field `CNTB3EN` reader - CNTB3EN
pub type CNTB3EN_R = crate::BitReader<bool>;
///Field `CNTB3EN` writer - CNTB3EN
pub type CNTB3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCSCNTR_SPEC, bool, O>;
///Field `CNTB4EN` reader - CNTB4EN
pub type CNTB4EN_R = crate::BitReader<bool>;
///Field `CNTB4EN` writer - CNTB4EN
pub type CNTB4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCSCNTR_SPEC, bool, O>;
impl R {
    ///Bits 0:15 - CSCOUNT
    #[inline(always)]
    pub fn cscount(&self) -> CSCOUNT_R {
        CSCOUNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - CNTB1EN
    #[inline(always)]
    pub fn cntb1en(&self) -> CNTB1EN_R {
        CNTB1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CNTB2EN
    #[inline(always)]
    pub fn cntb2en(&self) -> CNTB2EN_R {
        CNTB2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CNTB3EN
    #[inline(always)]
    pub fn cntb3en(&self) -> CNTB3EN_R {
        CNTB3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CNTB4EN
    #[inline(always)]
    pub fn cntb4en(&self) -> CNTB4EN_R {
        CNTB4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - CSCOUNT
    #[inline(always)]
    #[must_use]
    pub fn cscount(&mut self) -> CSCOUNT_W<0> {
        CSCOUNT_W::new(self)
    }
    ///Bit 16 - CNTB1EN
    #[inline(always)]
    #[must_use]
    pub fn cntb1en(&mut self) -> CNTB1EN_W<16> {
        CNTB1EN_W::new(self)
    }
    ///Bit 17 - CNTB2EN
    #[inline(always)]
    #[must_use]
    pub fn cntb2en(&mut self) -> CNTB2EN_W<17> {
        CNTB2EN_W::new(self)
    }
    ///Bit 18 - CNTB3EN
    #[inline(always)]
    #[must_use]
    pub fn cntb3en(&mut self) -> CNTB3EN_W<18> {
        CNTB3EN_W::new(self)
    }
    ///Bit 19 - CNTB4EN
    #[inline(always)]
    #[must_use]
    pub fn cntb4en(&mut self) -> CNTB4EN_W<19> {
        CNTB4EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PSRAM chip select counter register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcscntr](index.html) module
pub struct PCSCNTR_SPEC;
impl crate::RegisterSpec for PCSCNTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcscntr::R](R) reader structure
impl crate::Readable for PCSCNTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcscntr::W](W) writer structure
impl crate::Writable for PCSCNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PCSCNTR to value 0
impl crate::Resettable for PCSCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
