///Register `EEFDR2` reader
pub struct R(crate::R<EEFDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFDR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EEFDR2` writer
pub struct W(crate::W<EEFDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFDR2_SPEC>;
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
impl From<crate::W<EEFDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFDR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EE6LTCH` reader - External Event 6 latch
pub type EE6LTCH_R = crate::BitReader<bool>;
///Field `EE6LTCH` writer - External Event 6 latch
pub type EE6LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFDR2_SPEC, bool, O>;
///Field `EE6FLTR` reader - External Event 6 filter
pub type EE6FLTR_R = crate::FieldReader<u8, u8>;
///Field `EE6FLTR` writer - External Event 6 filter
pub type EE6FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFDR2_SPEC, u8, u8, 4, O>;
///Field `EE7LTCH` reader - External Event 7 latch
pub type EE7LTCH_R = crate::BitReader<bool>;
///Field `EE7LTCH` writer - External Event 7 latch
pub type EE7LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFDR2_SPEC, bool, O>;
///Field `EE7FLTR` reader - External Event 7 filter
pub type EE7FLTR_R = crate::FieldReader<u8, u8>;
///Field `EE7FLTR` writer - External Event 7 filter
pub type EE7FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFDR2_SPEC, u8, u8, 4, O>;
///Field `EE8LTCH` reader - External Event 8 latch
pub type EE8LTCH_R = crate::BitReader<bool>;
///Field `EE8LTCH` writer - External Event 8 latch
pub type EE8LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFDR2_SPEC, bool, O>;
///Field `EE8FLTR` reader - External Event 8 filter
pub type EE8FLTR_R = crate::FieldReader<u8, u8>;
///Field `EE8FLTR` writer - External Event 8 filter
pub type EE8FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFDR2_SPEC, u8, u8, 4, O>;
///Field `EE9LTCH` reader - External Event 9 latch
pub type EE9LTCH_R = crate::BitReader<bool>;
///Field `EE9LTCH` writer - External Event 9 latch
pub type EE9LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFDR2_SPEC, bool, O>;
///Field `EE9FLTR` reader - External Event 9 filter
pub type EE9FLTR_R = crate::FieldReader<u8, u8>;
///Field `EE9FLTR` writer - External Event 9 filter
pub type EE9FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFDR2_SPEC, u8, u8, 4, O>;
///Field `EE10LTCH` reader - External Event 10 latch
pub type EE10LTCH_R = crate::BitReader<bool>;
///Field `EE10LTCH` writer - External Event 10 latch
pub type EE10LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFDR2_SPEC, bool, O>;
///Field `EE10FLTR` reader - External Event 10 filter
pub type EE10FLTR_R = crate::FieldReader<u8, u8>;
///Field `EE10FLTR` writer - External Event 10 filter
pub type EE10FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFDR2_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - External Event 6 latch
    #[inline(always)]
    pub fn ee6ltch(&self) -> EE6LTCH_R {
        EE6LTCH_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - External Event 6 filter
    #[inline(always)]
    pub fn ee6fltr(&self) -> EE6FLTR_R {
        EE6FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 6 - External Event 7 latch
    #[inline(always)]
    pub fn ee7ltch(&self) -> EE7LTCH_R {
        EE7LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:10 - External Event 7 filter
    #[inline(always)]
    pub fn ee7fltr(&self) -> EE7FLTR_R {
        EE7FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bit 12 - External Event 8 latch
    #[inline(always)]
    pub fn ee8ltch(&self) -> EE8LTCH_R {
        EE8LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:16 - External Event 8 filter
    #[inline(always)]
    pub fn ee8fltr(&self) -> EE8FLTR_R {
        EE8FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bit 18 - External Event 9 latch
    #[inline(always)]
    pub fn ee9ltch(&self) -> EE9LTCH_R {
        EE9LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:22 - External Event 9 filter
    #[inline(always)]
    pub fn ee9fltr(&self) -> EE9FLTR_R {
        EE9FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bit 24 - External Event 10 latch
    #[inline(always)]
    pub fn ee10ltch(&self) -> EE10LTCH_R {
        EE10LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:28 - External Event 10 filter
    #[inline(always)]
    pub fn ee10fltr(&self) -> EE10FLTR_R {
        EE10FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - External Event 6 latch
    #[inline(always)]
    #[must_use]
    pub fn ee6ltch(&mut self) -> EE6LTCH_W<0> {
        EE6LTCH_W::new(self)
    }
    ///Bits 1:4 - External Event 6 filter
    #[inline(always)]
    #[must_use]
    pub fn ee6fltr(&mut self) -> EE6FLTR_W<1> {
        EE6FLTR_W::new(self)
    }
    ///Bit 6 - External Event 7 latch
    #[inline(always)]
    #[must_use]
    pub fn ee7ltch(&mut self) -> EE7LTCH_W<6> {
        EE7LTCH_W::new(self)
    }
    ///Bits 7:10 - External Event 7 filter
    #[inline(always)]
    #[must_use]
    pub fn ee7fltr(&mut self) -> EE7FLTR_W<7> {
        EE7FLTR_W::new(self)
    }
    ///Bit 12 - External Event 8 latch
    #[inline(always)]
    #[must_use]
    pub fn ee8ltch(&mut self) -> EE8LTCH_W<12> {
        EE8LTCH_W::new(self)
    }
    ///Bits 13:16 - External Event 8 filter
    #[inline(always)]
    #[must_use]
    pub fn ee8fltr(&mut self) -> EE8FLTR_W<13> {
        EE8FLTR_W::new(self)
    }
    ///Bit 18 - External Event 9 latch
    #[inline(always)]
    #[must_use]
    pub fn ee9ltch(&mut self) -> EE9LTCH_W<18> {
        EE9LTCH_W::new(self)
    }
    ///Bits 19:22 - External Event 9 filter
    #[inline(always)]
    #[must_use]
    pub fn ee9fltr(&mut self) -> EE9FLTR_W<19> {
        EE9FLTR_W::new(self)
    }
    ///Bit 24 - External Event 10 latch
    #[inline(always)]
    #[must_use]
    pub fn ee10ltch(&mut self) -> EE10LTCH_W<24> {
        EE10LTCH_W::new(self)
    }
    ///Bits 25:28 - External Event 10 filter
    #[inline(always)]
    #[must_use]
    pub fn ee10fltr(&mut self) -> EE10FLTR_W<25> {
        EE10FLTR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx External Event Filtering Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefdr2](index.html) module
pub struct EEFDR2_SPEC;
impl crate::RegisterSpec for EEFDR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [eefdr2::R](R) reader structure
impl crate::Readable for EEFDR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eefdr2::W](W) writer structure
impl crate::Writable for EEFDR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EEFDR2 to value 0
impl crate::Resettable for EEFDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
