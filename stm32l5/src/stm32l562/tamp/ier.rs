///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TAMP1IE` reader - TAMP1IE
pub type TAMP1IE_R = crate::BitReader<bool>;
///Field `TAMP1IE` writer - TAMP1IE
pub type TAMP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TAMP2IE` reader - TAMP2IE
pub type TAMP2IE_R = crate::BitReader<bool>;
///Field `TAMP2IE` writer - TAMP2IE
pub type TAMP2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TAMP3IE` reader - TAMP3IE
pub type TAMP3IE_R = crate::BitReader<bool>;
///Field `TAMP3IE` writer - TAMP3IE
pub type TAMP3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TAMP4IE` reader - TAMP4IE
pub type TAMP4IE_R = crate::BitReader<bool>;
///Field `TAMP4IE` writer - TAMP4IE
pub type TAMP4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TAMP5IE` reader - TAMP5IE
pub type TAMP5IE_R = crate::BitReader<bool>;
///Field `TAMP5IE` writer - TAMP5IE
pub type TAMP5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TAMP6IE` reader - TAMP6IE
pub type TAMP6IE_R = crate::BitReader<bool>;
///Field `TAMP6IE` writer - TAMP6IE
pub type TAMP6IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TAMP7IE` reader - TAMP7IE
pub type TAMP7IE_R = crate::BitReader<bool>;
///Field `TAMP7IE` writer - TAMP7IE
pub type TAMP7IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TAMP8IE` reader - TAMP8IE
pub type TAMP8IE_R = crate::BitReader<bool>;
///Field `TAMP8IE` writer - TAMP8IE
pub type TAMP8IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP1IE` reader - ITAMP1IE
pub type ITAMP1IE_R = crate::BitReader<bool>;
///Field `ITAMP1IE` writer - ITAMP1IE
pub type ITAMP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP2IE` reader - ITAMP2IE
pub type ITAMP2IE_R = crate::BitReader<bool>;
///Field `ITAMP2IE` writer - ITAMP2IE
pub type ITAMP2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP3IE` reader - ITAMP3IE
pub type ITAMP3IE_R = crate::BitReader<bool>;
///Field `ITAMP3IE` writer - ITAMP3IE
pub type ITAMP3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP5IE` reader - ITAMP5IE
pub type ITAMP5IE_R = crate::BitReader<bool>;
///Field `ITAMP5IE` writer - ITAMP5IE
pub type ITAMP5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP8IE` reader - ITAMP8IE
pub type ITAMP8IE_R = crate::BitReader<bool>;
///Field `ITAMP8IE` writer - ITAMP8IE
pub type ITAMP8IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    ///Bit 0 - TAMP1IE
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2IE
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3IE
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TAMP4IE
    #[inline(always)]
    pub fn tamp4ie(&self) -> TAMP4IE_R {
        TAMP4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TAMP5IE
    #[inline(always)]
    pub fn tamp5ie(&self) -> TAMP5IE_R {
        TAMP5IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TAMP6IE
    #[inline(always)]
    pub fn tamp6ie(&self) -> TAMP6IE_R {
        TAMP6IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMP7IE
    #[inline(always)]
    pub fn tamp7ie(&self) -> TAMP7IE_R {
        TAMP7IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TAMP8IE
    #[inline(always)]
    pub fn tamp8ie(&self) -> TAMP8IE_R {
        TAMP8IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - ITAMP1IE
    #[inline(always)]
    pub fn itamp1ie(&self) -> ITAMP1IE_R {
        ITAMP1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ITAMP2IE
    #[inline(always)]
    pub fn itamp2ie(&self) -> ITAMP2IE_R {
        ITAMP2IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ITAMP3IE
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ITAMP5IE
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 23 - ITAMP8IE
    #[inline(always)]
    pub fn itamp8ie(&self) -> ITAMP8IE_R {
        ITAMP8IE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TAMP1IE
    #[inline(always)]
    #[must_use]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<0> {
        TAMP1IE_W::new(self)
    }
    ///Bit 1 - TAMP2IE
    #[inline(always)]
    #[must_use]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<1> {
        TAMP2IE_W::new(self)
    }
    ///Bit 2 - TAMP3IE
    #[inline(always)]
    #[must_use]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W<2> {
        TAMP3IE_W::new(self)
    }
    ///Bit 3 - TAMP4IE
    #[inline(always)]
    #[must_use]
    pub fn tamp4ie(&mut self) -> TAMP4IE_W<3> {
        TAMP4IE_W::new(self)
    }
    ///Bit 4 - TAMP5IE
    #[inline(always)]
    #[must_use]
    pub fn tamp5ie(&mut self) -> TAMP5IE_W<4> {
        TAMP5IE_W::new(self)
    }
    ///Bit 5 - TAMP6IE
    #[inline(always)]
    #[must_use]
    pub fn tamp6ie(&mut self) -> TAMP6IE_W<5> {
        TAMP6IE_W::new(self)
    }
    ///Bit 6 - TAMP7IE
    #[inline(always)]
    #[must_use]
    pub fn tamp7ie(&mut self) -> TAMP7IE_W<6> {
        TAMP7IE_W::new(self)
    }
    ///Bit 7 - TAMP8IE
    #[inline(always)]
    #[must_use]
    pub fn tamp8ie(&mut self) -> TAMP8IE_W<7> {
        TAMP8IE_W::new(self)
    }
    ///Bit 16 - ITAMP1IE
    #[inline(always)]
    #[must_use]
    pub fn itamp1ie(&mut self) -> ITAMP1IE_W<16> {
        ITAMP1IE_W::new(self)
    }
    ///Bit 17 - ITAMP2IE
    #[inline(always)]
    #[must_use]
    pub fn itamp2ie(&mut self) -> ITAMP2IE_W<17> {
        ITAMP2IE_W::new(self)
    }
    ///Bit 18 - ITAMP3IE
    #[inline(always)]
    #[must_use]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W<18> {
        ITAMP3IE_W::new(self)
    }
    ///Bit 20 - ITAMP5IE
    #[inline(always)]
    #[must_use]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W<20> {
        ITAMP5IE_W::new(self)
    }
    ///Bit 23 - ITAMP8IE
    #[inline(always)]
    #[must_use]
    pub fn itamp8ie(&mut self) -> ITAMP8IE_W<23> {
        ITAMP8IE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
