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
///Field `TAMP1IE` reader - Tamper 1 interrupt enable
pub type TAMP1IE_R = crate::BitReader<bool>;
///Field `TAMP1IE` writer - Tamper 1 interrupt enable
pub type TAMP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TAMP2IE` reader - Tamper 2 interrupt enable
pub type TAMP2IE_R = crate::BitReader<bool>;
///Field `TAMP2IE` writer - Tamper 2 interrupt enable
pub type TAMP2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP1IE` reader - Internal tamper 1 interrupt enable
pub type ITAMP1IE_R = crate::BitReader<bool>;
///Field `ITAMP1IE` writer - Internal tamper 1 interrupt enable
pub type ITAMP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP2IE` reader - Internal tamper 2 interrupt enable
pub type ITAMP2IE_R = crate::BitReader<bool>;
///Field `ITAMP2IE` writer - Internal tamper 2 interrupt enable
pub type ITAMP2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP3IE` reader - Internal tamper 3 interrupt enable
pub type ITAMP3IE_R = crate::BitReader<bool>;
///Field `ITAMP3IE` writer - Internal tamper 3 interrupt enable
pub type ITAMP3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP4IE` reader - Internal tamper 4 interrupt enable
pub type ITAMP4IE_R = crate::BitReader<bool>;
///Field `ITAMP4IE` writer - Internal tamper 4 interrupt enable
pub type ITAMP4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP5IE` reader - Internal tamper 5 interrupt enable
pub type ITAMP5IE_R = crate::BitReader<bool>;
///Field `ITAMP5IE` writer - Internal tamper 5 interrupt enable
pub type ITAMP5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP6IE` reader - Internal tamper 6 interrupt enable
pub type ITAMP6IE_R = crate::BitReader<bool>;
///Field `ITAMP6IE` writer - Internal tamper 6 interrupt enable
pub type ITAMP6IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP7IE` reader - Internal tamper 7 interrupt enable
pub type ITAMP7IE_R = crate::BitReader<bool>;
///Field `ITAMP7IE` writer - Internal tamper 7 interrupt enable
pub type ITAMP7IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP8IE` reader - Internal tamper 8 interrupt enable
pub type ITAMP8IE_R = crate::BitReader<bool>;
///Field `ITAMP8IE` writer - Internal tamper 8 interrupt enable
pub type ITAMP8IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP9IE` reader - Internal tamper 9 interrupt enable
pub type ITAMP9IE_R = crate::BitReader<bool>;
///Field `ITAMP9IE` writer - Internal tamper 9 interrupt enable
pub type ITAMP9IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP11IE` reader - Internal tamper 11 interrupt enable
pub type ITAMP11IE_R = crate::BitReader<bool>;
///Field `ITAMP11IE` writer - Internal tamper 11 interrupt enable
pub type ITAMP11IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP12IE` reader - Internal tamper 12 interrupt enable
pub type ITAMP12IE_R = crate::BitReader<bool>;
///Field `ITAMP12IE` writer - Internal tamper 12 interrupt enable
pub type ITAMP12IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP13IE` reader - Internal tamper 13 interrupt enable
pub type ITAMP13IE_R = crate::BitReader<bool>;
///Field `ITAMP13IE` writer - Internal tamper 13 interrupt enable
pub type ITAMP13IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `ITAMP15IE` reader - Internal tamper 15 interrupt enable
pub type ITAMP15IE_R = crate::BitReader<bool>;
///Field `ITAMP15IE` writer - Internal tamper 15 interrupt enable
pub type ITAMP15IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    ///Bit 0 - Tamper 1 interrupt enable
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tamper 2 interrupt enable
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - Internal tamper 1 interrupt enable
    #[inline(always)]
    pub fn itamp1ie(&self) -> ITAMP1IE_R {
        ITAMP1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Internal tamper 2 interrupt enable
    #[inline(always)]
    pub fn itamp2ie(&self) -> ITAMP2IE_R {
        ITAMP2IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Internal tamper 3 interrupt enable
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Internal tamper 4 interrupt enable
    #[inline(always)]
    pub fn itamp4ie(&self) -> ITAMP4IE_R {
        ITAMP4IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Internal tamper 5 interrupt enable
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Internal tamper 6 interrupt enable
    #[inline(always)]
    pub fn itamp6ie(&self) -> ITAMP6IE_R {
        ITAMP6IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Internal tamper 7 interrupt enable
    #[inline(always)]
    pub fn itamp7ie(&self) -> ITAMP7IE_R {
        ITAMP7IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Internal tamper 8 interrupt enable
    #[inline(always)]
    pub fn itamp8ie(&self) -> ITAMP8IE_R {
        ITAMP8IE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Internal tamper 9 interrupt enable
    #[inline(always)]
    pub fn itamp9ie(&self) -> ITAMP9IE_R {
        ITAMP9IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Internal tamper 11 interrupt enable
    #[inline(always)]
    pub fn itamp11ie(&self) -> ITAMP11IE_R {
        ITAMP11IE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Internal tamper 12 interrupt enable
    #[inline(always)]
    pub fn itamp12ie(&self) -> ITAMP12IE_R {
        ITAMP12IE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Internal tamper 13 interrupt enable
    #[inline(always)]
    pub fn itamp13ie(&self) -> ITAMP13IE_R {
        ITAMP13IE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Internal tamper 15 interrupt enable
    #[inline(always)]
    pub fn itamp15ie(&self) -> ITAMP15IE_R {
        ITAMP15IE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Tamper 1 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<0> {
        TAMP1IE_W::new(self)
    }
    ///Bit 1 - Tamper 2 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<1> {
        TAMP2IE_W::new(self)
    }
    ///Bit 16 - Internal tamper 1 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp1ie(&mut self) -> ITAMP1IE_W<16> {
        ITAMP1IE_W::new(self)
    }
    ///Bit 17 - Internal tamper 2 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp2ie(&mut self) -> ITAMP2IE_W<17> {
        ITAMP2IE_W::new(self)
    }
    ///Bit 18 - Internal tamper 3 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W<18> {
        ITAMP3IE_W::new(self)
    }
    ///Bit 19 - Internal tamper 4 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp4ie(&mut self) -> ITAMP4IE_W<19> {
        ITAMP4IE_W::new(self)
    }
    ///Bit 20 - Internal tamper 5 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W<20> {
        ITAMP5IE_W::new(self)
    }
    ///Bit 21 - Internal tamper 6 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp6ie(&mut self) -> ITAMP6IE_W<21> {
        ITAMP6IE_W::new(self)
    }
    ///Bit 22 - Internal tamper 7 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp7ie(&mut self) -> ITAMP7IE_W<22> {
        ITAMP7IE_W::new(self)
    }
    ///Bit 23 - Internal tamper 8 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp8ie(&mut self) -> ITAMP8IE_W<23> {
        ITAMP8IE_W::new(self)
    }
    ///Bit 24 - Internal tamper 9 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp9ie(&mut self) -> ITAMP9IE_W<24> {
        ITAMP9IE_W::new(self)
    }
    ///Bit 26 - Internal tamper 11 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp11ie(&mut self) -> ITAMP11IE_W<26> {
        ITAMP11IE_W::new(self)
    }
    ///Bit 27 - Internal tamper 12 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp12ie(&mut self) -> ITAMP12IE_W<27> {
        ITAMP12IE_W::new(self)
    }
    ///Bit 28 - Internal tamper 13 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp13ie(&mut self) -> ITAMP13IE_W<28> {
        ITAMP13IE_W::new(self)
    }
    ///Bit 30 - Internal tamper 15 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itamp15ie(&mut self) -> ITAMP15IE_W<30> {
        ITAMP15IE_W::new(self)
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
