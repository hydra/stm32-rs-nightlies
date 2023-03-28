///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TAMP1E` reader - Tamper detection on TAMP_IN1 enable
pub type TAMP1E_R = crate::BitReader<bool>;
///Field `TAMP1E` writer - Tamper detection on TAMP_IN1 enable
pub type TAMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `TAMP2E` reader - Tamper detection on TAMP_IN2 enable
pub type TAMP2E_R = crate::BitReader<bool>;
///Field `TAMP2E` writer - Tamper detection on TAMP_IN2 enable
pub type TAMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP1E` reader - Internal tamper 1 enable
pub type ITAMP1E_R = crate::BitReader<bool>;
///Field `ITAMP1E` writer - Internal tamper 1 enable
pub type ITAMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP2E` reader - Internal tamper 2 enable
pub type ITAMP2E_R = crate::BitReader<bool>;
///Field `ITAMP2E` writer - Internal tamper 2 enable
pub type ITAMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP3E` reader - Internal tamper 3 enable
pub type ITAMP3E_R = crate::BitReader<bool>;
///Field `ITAMP3E` writer - Internal tamper 3 enable
pub type ITAMP3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP4E` reader - Internal tamper 4 enable
pub type ITAMP4E_R = crate::BitReader<bool>;
///Field `ITAMP4E` writer - Internal tamper 4 enable
pub type ITAMP4E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP5E` reader - Internal tamper 5 enable
pub type ITAMP5E_R = crate::BitReader<bool>;
///Field `ITAMP5E` writer - Internal tamper 5 enable
pub type ITAMP5E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP6E` reader - Internal tamper 6 enable
pub type ITAMP6E_R = crate::BitReader<bool>;
///Field `ITAMP6E` writer - Internal tamper 6 enable
pub type ITAMP6E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP7E` reader - Internal tamper 7 enable
pub type ITAMP7E_R = crate::BitReader<bool>;
///Field `ITAMP7E` writer - Internal tamper 7 enable
pub type ITAMP7E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP8E` reader - Internal tamper 8 enable
pub type ITAMP8E_R = crate::BitReader<bool>;
///Field `ITAMP8E` writer - Internal tamper 8 enable
pub type ITAMP8E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP9E` reader - Internal tamper 9 enable
pub type ITAMP9E_R = crate::BitReader<bool>;
///Field `ITAMP9E` writer - Internal tamper 9 enable
pub type ITAMP9E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP11E` reader - Internal tamper 11 enable
pub type ITAMP11E_R = crate::BitReader<bool>;
///Field `ITAMP11E` writer - Internal tamper 11 enable
pub type ITAMP11E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP12E` reader - Internal tamper 12 enable
pub type ITAMP12E_R = crate::BitReader<bool>;
///Field `ITAMP12E` writer - Internal tamper 12 enable
pub type ITAMP12E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP13E` reader - Internal tamper 13 enable
pub type ITAMP13E_R = crate::BitReader<bool>;
///Field `ITAMP13E` writer - Internal tamper 13 enable
pub type ITAMP13E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `ITAMP15E` reader - Internal tamper 15 enable
pub type ITAMP15E_R = crate::BitReader<bool>;
///Field `ITAMP15E` writer - Internal tamper 15 enable
pub type ITAMP15E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - Tamper detection on TAMP_IN1 enable
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tamper detection on TAMP_IN2 enable
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - Internal tamper 1 enable
    #[inline(always)]
    pub fn itamp1e(&self) -> ITAMP1E_R {
        ITAMP1E_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Internal tamper 2 enable
    #[inline(always)]
    pub fn itamp2e(&self) -> ITAMP2E_R {
        ITAMP2E_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Internal tamper 3 enable
    #[inline(always)]
    pub fn itamp3e(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Internal tamper 4 enable
    #[inline(always)]
    pub fn itamp4e(&self) -> ITAMP4E_R {
        ITAMP4E_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Internal tamper 5 enable
    #[inline(always)]
    pub fn itamp5e(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Internal tamper 6 enable
    #[inline(always)]
    pub fn itamp6e(&self) -> ITAMP6E_R {
        ITAMP6E_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Internal tamper 7 enable
    #[inline(always)]
    pub fn itamp7e(&self) -> ITAMP7E_R {
        ITAMP7E_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Internal tamper 8 enable
    #[inline(always)]
    pub fn itamp8e(&self) -> ITAMP8E_R {
        ITAMP8E_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Internal tamper 9 enable
    #[inline(always)]
    pub fn itamp9e(&self) -> ITAMP9E_R {
        ITAMP9E_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Internal tamper 11 enable
    #[inline(always)]
    pub fn itamp11e(&self) -> ITAMP11E_R {
        ITAMP11E_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Internal tamper 12 enable
    #[inline(always)]
    pub fn itamp12e(&self) -> ITAMP12E_R {
        ITAMP12E_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Internal tamper 13 enable
    #[inline(always)]
    pub fn itamp13e(&self) -> ITAMP13E_R {
        ITAMP13E_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Internal tamper 15 enable
    #[inline(always)]
    pub fn itamp15e(&self) -> ITAMP15E_R {
        ITAMP15E_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Tamper detection on TAMP_IN1 enable
    #[inline(always)]
    #[must_use]
    pub fn tamp1e(&mut self) -> TAMP1E_W<0> {
        TAMP1E_W::new(self)
    }
    ///Bit 1 - Tamper detection on TAMP_IN2 enable
    #[inline(always)]
    #[must_use]
    pub fn tamp2e(&mut self) -> TAMP2E_W<1> {
        TAMP2E_W::new(self)
    }
    ///Bit 16 - Internal tamper 1 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp1e(&mut self) -> ITAMP1E_W<16> {
        ITAMP1E_W::new(self)
    }
    ///Bit 17 - Internal tamper 2 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp2e(&mut self) -> ITAMP2E_W<17> {
        ITAMP2E_W::new(self)
    }
    ///Bit 18 - Internal tamper 3 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp3e(&mut self) -> ITAMP3E_W<18> {
        ITAMP3E_W::new(self)
    }
    ///Bit 19 - Internal tamper 4 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp4e(&mut self) -> ITAMP4E_W<19> {
        ITAMP4E_W::new(self)
    }
    ///Bit 20 - Internal tamper 5 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp5e(&mut self) -> ITAMP5E_W<20> {
        ITAMP5E_W::new(self)
    }
    ///Bit 21 - Internal tamper 6 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp6e(&mut self) -> ITAMP6E_W<21> {
        ITAMP6E_W::new(self)
    }
    ///Bit 22 - Internal tamper 7 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp7e(&mut self) -> ITAMP7E_W<22> {
        ITAMP7E_W::new(self)
    }
    ///Bit 23 - Internal tamper 8 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp8e(&mut self) -> ITAMP8E_W<23> {
        ITAMP8E_W::new(self)
    }
    ///Bit 24 - Internal tamper 9 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp9e(&mut self) -> ITAMP9E_W<24> {
        ITAMP9E_W::new(self)
    }
    ///Bit 26 - Internal tamper 11 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp11e(&mut self) -> ITAMP11E_W<26> {
        ITAMP11E_W::new(self)
    }
    ///Bit 27 - Internal tamper 12 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp12e(&mut self) -> ITAMP12E_W<27> {
        ITAMP12E_W::new(self)
    }
    ///Bit 28 - Internal tamper 13 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp13e(&mut self) -> ITAMP13E_W<28> {
        ITAMP13E_W::new(self)
    }
    ///Bit 30 - Internal tamper 15 enable
    #[inline(always)]
    #[must_use]
    pub fn itamp15e(&mut self) -> ITAMP15E_W<30> {
        ITAMP15E_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
