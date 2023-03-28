///Register `IER1` reader
pub struct R(crate::R<IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER1` writer
pub struct W(crate::W<IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER1_SPEC>;
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
impl From<crate::W<IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TOHSTXIE` reader - TOHSTXIE
pub type TOHSTXIE_R = crate::BitReader<bool>;
///Field `TOHSTXIE` writer - TOHSTXIE
pub type TOHSTXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `TOLPRXIE` reader - TOLPRXIE
pub type TOLPRXIE_R = crate::BitReader<bool>;
///Field `TOLPRXIE` writer - TOLPRXIE
pub type TOLPRXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `ECCSEIE` reader - ECCSEIE
pub type ECCSEIE_R = crate::BitReader<bool>;
///Field `ECCSEIE` writer - ECCSEIE
pub type ECCSEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `ECCMEIE` reader - ECCMEIE
pub type ECCMEIE_R = crate::BitReader<bool>;
///Field `ECCMEIE` writer - ECCMEIE
pub type ECCMEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `CRCEIE` reader - CRCEIE
pub type CRCEIE_R = crate::BitReader<bool>;
///Field `CRCEIE` writer - CRCEIE
pub type CRCEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `PSEIE` reader - PSEIE
pub type PSEIE_R = crate::BitReader<bool>;
///Field `PSEIE` writer - PSEIE
pub type PSEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `EOTPEIE` reader - EOTPEIE
pub type EOTPEIE_R = crate::BitReader<bool>;
///Field `EOTPEIE` writer - EOTPEIE
pub type EOTPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `LPWREIE` reader - LPWREIE
pub type LPWREIE_R = crate::BitReader<bool>;
///Field `LPWREIE` writer - LPWREIE
pub type LPWREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `GCWREIE` reader - GCWREIE
pub type GCWREIE_R = crate::BitReader<bool>;
///Field `GCWREIE` writer - GCWREIE
pub type GCWREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `GPWREIE` reader - GPWREIE
pub type GPWREIE_R = crate::BitReader<bool>;
///Field `GPWREIE` writer - GPWREIE
pub type GPWREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `GPTXEIE` reader - GPTXEIE
pub type GPTXEIE_R = crate::BitReader<bool>;
///Field `GPTXEIE` writer - GPTXEIE
pub type GPTXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `GPRDEIE` reader - GPRDEIE
pub type GPRDEIE_R = crate::BitReader<bool>;
///Field `GPRDEIE` writer - GPRDEIE
pub type GPRDEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `GPRXEIE` reader - GPRXEIE
pub type GPRXEIE_R = crate::BitReader<bool>;
///Field `GPRXEIE` writer - GPRXEIE
pub type GPRXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
impl R {
    ///Bit 0 - TOHSTXIE
    #[inline(always)]
    pub fn tohstxie(&self) -> TOHSTXIE_R {
        TOHSTXIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TOLPRXIE
    #[inline(always)]
    pub fn tolprxie(&self) -> TOLPRXIE_R {
        TOLPRXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ECCSEIE
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECCMEIE
    #[inline(always)]
    pub fn eccmeie(&self) -> ECCMEIE_R {
        ECCMEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRCEIE
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PSEIE
    #[inline(always)]
    pub fn pseie(&self) -> PSEIE_R {
        PSEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EOTPEIE
    #[inline(always)]
    pub fn eotpeie(&self) -> EOTPEIE_R {
        EOTPEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LPWREIE
    #[inline(always)]
    pub fn lpwreie(&self) -> LPWREIE_R {
        LPWREIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GCWREIE
    #[inline(always)]
    pub fn gcwreie(&self) -> GCWREIE_R {
        GCWREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPWREIE
    #[inline(always)]
    pub fn gpwreie(&self) -> GPWREIE_R {
        GPWREIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPTXEIE
    #[inline(always)]
    pub fn gptxeie(&self) -> GPTXEIE_R {
        GPTXEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GPRDEIE
    #[inline(always)]
    pub fn gprdeie(&self) -> GPRDEIE_R {
        GPRDEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GPRXEIE
    #[inline(always)]
    pub fn gprxeie(&self) -> GPRXEIE_R {
        GPRXEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TOHSTXIE
    #[inline(always)]
    #[must_use]
    pub fn tohstxie(&mut self) -> TOHSTXIE_W<0> {
        TOHSTXIE_W::new(self)
    }
    ///Bit 1 - TOLPRXIE
    #[inline(always)]
    #[must_use]
    pub fn tolprxie(&mut self) -> TOLPRXIE_W<1> {
        TOLPRXIE_W::new(self)
    }
    ///Bit 2 - ECCSEIE
    #[inline(always)]
    #[must_use]
    pub fn eccseie(&mut self) -> ECCSEIE_W<2> {
        ECCSEIE_W::new(self)
    }
    ///Bit 3 - ECCMEIE
    #[inline(always)]
    #[must_use]
    pub fn eccmeie(&mut self) -> ECCMEIE_W<3> {
        ECCMEIE_W::new(self)
    }
    ///Bit 4 - CRCEIE
    #[inline(always)]
    #[must_use]
    pub fn crceie(&mut self) -> CRCEIE_W<4> {
        CRCEIE_W::new(self)
    }
    ///Bit 5 - PSEIE
    #[inline(always)]
    #[must_use]
    pub fn pseie(&mut self) -> PSEIE_W<5> {
        PSEIE_W::new(self)
    }
    ///Bit 6 - EOTPEIE
    #[inline(always)]
    #[must_use]
    pub fn eotpeie(&mut self) -> EOTPEIE_W<6> {
        EOTPEIE_W::new(self)
    }
    ///Bit 7 - LPWREIE
    #[inline(always)]
    #[must_use]
    pub fn lpwreie(&mut self) -> LPWREIE_W<7> {
        LPWREIE_W::new(self)
    }
    ///Bit 8 - GCWREIE
    #[inline(always)]
    #[must_use]
    pub fn gcwreie(&mut self) -> GCWREIE_W<8> {
        GCWREIE_W::new(self)
    }
    ///Bit 9 - GPWREIE
    #[inline(always)]
    #[must_use]
    pub fn gpwreie(&mut self) -> GPWREIE_W<9> {
        GPWREIE_W::new(self)
    }
    ///Bit 10 - GPTXEIE
    #[inline(always)]
    #[must_use]
    pub fn gptxeie(&mut self) -> GPTXEIE_W<10> {
        GPTXEIE_W::new(self)
    }
    ///Bit 11 - GPRDEIE
    #[inline(always)]
    #[must_use]
    pub fn gprdeie(&mut self) -> GPRDEIE_W<11> {
        GPRDEIE_W::new(self)
    }
    ///Bit 12 - GPRXEIE
    #[inline(always)]
    #[must_use]
    pub fn gprxeie(&mut self) -> GPRXEIE_W<12> {
        GPRXEIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host interrupt enable register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier1](index.html) module
pub struct IER1_SPEC;
impl crate::RegisterSpec for IER1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier1::R](R) reader structure
impl crate::Readable for IER1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier1::W](W) writer structure
impl crate::Writable for IER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER1 to value 0
impl crate::Resettable for IER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
