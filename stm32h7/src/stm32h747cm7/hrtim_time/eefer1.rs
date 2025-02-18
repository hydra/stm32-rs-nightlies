///Register `EEFER1` reader
pub struct R(crate::R<EEFER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFER1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EEFER1` writer
pub struct W(crate::W<EEFER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFER1_SPEC>;
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
impl From<crate::W<EEFER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFER1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EE1LTCH` reader - External Event 1 latch
pub type EE1LTCH_R = crate::BitReader<bool>;
///Field `EE1LTCH` writer - External Event 1 latch
pub type EE1LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFER1_SPEC, bool, O>;
///Field `EE1FLTR` reader - External Event 1 filter
pub type EE1FLTR_R = crate::FieldReader<u8, u8>;
///Field `EE1FLTR` writer - External Event 1 filter
pub type EE1FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFER1_SPEC, u8, u8, 4, O>;
///Field `EE2LTCH` reader - External Event 2 latch
pub type EE2LTCH_R = crate::BitReader<bool>;
///Field `EE2LTCH` writer - External Event 2 latch
pub type EE2LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFER1_SPEC, bool, O>;
///Field `EE2FLTR` reader - External Event 2 filter
pub type EE2FLTR_R = crate::FieldReader<u8, u8>;
///Field `EE2FLTR` writer - External Event 2 filter
pub type EE2FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFER1_SPEC, u8, u8, 4, O>;
///Field `EE3LTCH` reader - External Event 3 latch
pub type EE3LTCH_R = crate::BitReader<bool>;
///Field `EE3LTCH` writer - External Event 3 latch
pub type EE3LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFER1_SPEC, bool, O>;
///Field `EE3FLTR` reader - External Event 3 filter
pub type EE3FLTR_R = crate::FieldReader<u8, u8>;
///Field `EE3FLTR` writer - External Event 3 filter
pub type EE3FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFER1_SPEC, u8, u8, 4, O>;
///Field `EE4LTCH` reader - External Event 4 latch
pub type EE4LTCH_R = crate::BitReader<bool>;
///Field `EE4LTCH` writer - External Event 4 latch
pub type EE4LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFER1_SPEC, bool, O>;
///Field `EE4FLTR` reader - External Event 4 filter
pub type EE4FLTR_R = crate::FieldReader<u8, u8>;
///Field `EE4FLTR` writer - External Event 4 filter
pub type EE4FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFER1_SPEC, u8, u8, 4, O>;
///Field `EE5LTCH` reader - External Event 5 latch
pub type EE5LTCH_R = crate::BitReader<bool>;
///Field `EE5LTCH` writer - External Event 5 latch
pub type EE5LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFER1_SPEC, bool, O>;
///Field `EE5FLTR` reader - External Event 5 filter
pub type EE5FLTR_R = crate::FieldReader<u8, u8>;
///Field `EE5FLTR` writer - External Event 5 filter
pub type EE5FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFER1_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - External Event 1 latch
    #[inline(always)]
    pub fn ee1ltch(&self) -> EE1LTCH_R {
        EE1LTCH_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - External Event 1 filter
    #[inline(always)]
    pub fn ee1fltr(&self) -> EE1FLTR_R {
        EE1FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 6 - External Event 2 latch
    #[inline(always)]
    pub fn ee2ltch(&self) -> EE2LTCH_R {
        EE2LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:10 - External Event 2 filter
    #[inline(always)]
    pub fn ee2fltr(&self) -> EE2FLTR_R {
        EE2FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bit 12 - External Event 3 latch
    #[inline(always)]
    pub fn ee3ltch(&self) -> EE3LTCH_R {
        EE3LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:16 - External Event 3 filter
    #[inline(always)]
    pub fn ee3fltr(&self) -> EE3FLTR_R {
        EE3FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bit 18 - External Event 4 latch
    #[inline(always)]
    pub fn ee4ltch(&self) -> EE4LTCH_R {
        EE4LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:22 - External Event 4 filter
    #[inline(always)]
    pub fn ee4fltr(&self) -> EE4FLTR_R {
        EE4FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bit 24 - External Event 5 latch
    #[inline(always)]
    pub fn ee5ltch(&self) -> EE5LTCH_R {
        EE5LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:28 - External Event 5 filter
    #[inline(always)]
    pub fn ee5fltr(&self) -> EE5FLTR_R {
        EE5FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - External Event 1 latch
    #[inline(always)]
    #[must_use]
    pub fn ee1ltch(&mut self) -> EE1LTCH_W<0> {
        EE1LTCH_W::new(self)
    }
    ///Bits 1:4 - External Event 1 filter
    #[inline(always)]
    #[must_use]
    pub fn ee1fltr(&mut self) -> EE1FLTR_W<1> {
        EE1FLTR_W::new(self)
    }
    ///Bit 6 - External Event 2 latch
    #[inline(always)]
    #[must_use]
    pub fn ee2ltch(&mut self) -> EE2LTCH_W<6> {
        EE2LTCH_W::new(self)
    }
    ///Bits 7:10 - External Event 2 filter
    #[inline(always)]
    #[must_use]
    pub fn ee2fltr(&mut self) -> EE2FLTR_W<7> {
        EE2FLTR_W::new(self)
    }
    ///Bit 12 - External Event 3 latch
    #[inline(always)]
    #[must_use]
    pub fn ee3ltch(&mut self) -> EE3LTCH_W<12> {
        EE3LTCH_W::new(self)
    }
    ///Bits 13:16 - External Event 3 filter
    #[inline(always)]
    #[must_use]
    pub fn ee3fltr(&mut self) -> EE3FLTR_W<13> {
        EE3FLTR_W::new(self)
    }
    ///Bit 18 - External Event 4 latch
    #[inline(always)]
    #[must_use]
    pub fn ee4ltch(&mut self) -> EE4LTCH_W<18> {
        EE4LTCH_W::new(self)
    }
    ///Bits 19:22 - External Event 4 filter
    #[inline(always)]
    #[must_use]
    pub fn ee4fltr(&mut self) -> EE4FLTR_W<19> {
        EE4FLTR_W::new(self)
    }
    ///Bit 24 - External Event 5 latch
    #[inline(always)]
    #[must_use]
    pub fn ee5ltch(&mut self) -> EE5LTCH_W<24> {
        EE5LTCH_W::new(self)
    }
    ///Bits 25:28 - External Event 5 filter
    #[inline(always)]
    #[must_use]
    pub fn ee5fltr(&mut self) -> EE5FLTR_W<25> {
        EE5FLTR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx External Event Filtering Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefer1](index.html) module
pub struct EEFER1_SPEC;
impl crate::RegisterSpec for EEFER1_SPEC {
    type Ux = u32;
}
///`read()` method returns [eefer1::R](R) reader structure
impl crate::Readable for EEFER1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eefer1::W](W) writer structure
impl crate::Writable for EEFER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EEFER1 to value 0
impl crate::Resettable for EEFER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
