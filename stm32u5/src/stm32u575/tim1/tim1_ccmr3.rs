///Register `TIM1_CCMR3` reader
pub struct R(crate::R<TIM1_CCMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_CCMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_CCMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_CCMR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM1_CCMR3` writer
pub struct W(crate::W<TIM1_CCMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_CCMR3_SPEC>;
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
impl From<crate::W<TIM1_CCMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_CCMR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OC5FE` reader - Output compare 5 fast enable
pub type OC5FE_R = crate::BitReader<bool>;
///Field `OC5FE` writer - Output compare 5 fast enable
pub type OC5FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCMR3_SPEC, bool, O>;
///Field `OC5PE` reader - Output compare 5 preload enable
pub type OC5PE_R = crate::BitReader<bool>;
///Field `OC5PE` writer - Output compare 5 preload enable
pub type OC5PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCMR3_SPEC, bool, O>;
///Field `OC5M1` reader - Output compare 5 mode
pub type OC5M1_R = crate::FieldReader<u8, u8>;
///Field `OC5M1` writer - Output compare 5 mode
pub type OC5M1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM1_CCMR3_SPEC, u8, u8, 3, O>;
///Field `OC5CE` reader - Output compare 5 clear enable
pub type OC5CE_R = crate::BitReader<bool>;
///Field `OC5CE` writer - Output compare 5 clear enable
pub type OC5CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCMR3_SPEC, bool, O>;
///Field `OC6FE` reader - Output compare 6 fast enable
pub type OC6FE_R = crate::BitReader<bool>;
///Field `OC6FE` writer - Output compare 6 fast enable
pub type OC6FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCMR3_SPEC, bool, O>;
///Field `OC6PE` reader - Output compare 6 preload enable
pub type OC6PE_R = crate::BitReader<bool>;
///Field `OC6PE` writer - Output compare 6 preload enable
pub type OC6PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCMR3_SPEC, bool, O>;
///Field `OC6M1` reader - Output compare 6 mode
pub type OC6M1_R = crate::FieldReader<u8, u8>;
///Field `OC6M1` writer - Output compare 6 mode
pub type OC6M1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM1_CCMR3_SPEC, u8, u8, 3, O>;
///Field `OC6CE` reader - Output compare 6 clear enable
pub type OC6CE_R = crate::BitReader<bool>;
///Field `OC6CE` writer - Output compare 6 clear enable
pub type OC6CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCMR3_SPEC, bool, O>;
///Field `OC5M2` reader - Output compare 5 mode
pub type OC5M2_R = crate::BitReader<bool>;
///Field `OC5M2` writer - Output compare 5 mode
pub type OC5M2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCMR3_SPEC, bool, O>;
///Field `OC6M2` reader - Output compare 6 mode
pub type OC6M2_R = crate::BitReader<bool>;
///Field `OC6M2` writer - Output compare 6 mode
pub type OC6M2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCMR3_SPEC, bool, O>;
impl R {
    ///Bit 2 - Output compare 5 fast enable
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output compare 5 preload enable
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Output compare 5 mode
    #[inline(always)]
    pub fn oc5m1(&self) -> OC5M1_R {
        OC5M1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 5 clear enable
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - Output compare 6 fast enable
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output compare 6 preload enable
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Output compare 6 mode
    #[inline(always)]
    pub fn oc6m1(&self) -> OC6M1_R {
        OC6M1_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 6 clear enable
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Output compare 5 mode
    #[inline(always)]
    pub fn oc5m2(&self) -> OC5M2_R {
        OC5M2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output compare 6 mode
    #[inline(always)]
    pub fn oc6m2(&self) -> OC6M2_R {
        OC6M2_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Output compare 5 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc5fe(&mut self) -> OC5FE_W<2> {
        OC5FE_W::new(self)
    }
    ///Bit 3 - Output compare 5 preload enable
    #[inline(always)]
    #[must_use]
    pub fn oc5pe(&mut self) -> OC5PE_W<3> {
        OC5PE_W::new(self)
    }
    ///Bits 4:6 - Output compare 5 mode
    #[inline(always)]
    #[must_use]
    pub fn oc5m1(&mut self) -> OC5M1_W<4> {
        OC5M1_W::new(self)
    }
    ///Bit 7 - Output compare 5 clear enable
    #[inline(always)]
    #[must_use]
    pub fn oc5ce(&mut self) -> OC5CE_W<7> {
        OC5CE_W::new(self)
    }
    ///Bit 10 - Output compare 6 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc6fe(&mut self) -> OC6FE_W<10> {
        OC6FE_W::new(self)
    }
    ///Bit 11 - Output compare 6 preload enable
    #[inline(always)]
    #[must_use]
    pub fn oc6pe(&mut self) -> OC6PE_W<11> {
        OC6PE_W::new(self)
    }
    ///Bits 12:14 - Output compare 6 mode
    #[inline(always)]
    #[must_use]
    pub fn oc6m1(&mut self) -> OC6M1_W<12> {
        OC6M1_W::new(self)
    }
    ///Bit 15 - Output compare 6 clear enable
    #[inline(always)]
    #[must_use]
    pub fn oc6ce(&mut self) -> OC6CE_W<15> {
        OC6CE_W::new(self)
    }
    ///Bit 16 - Output compare 5 mode
    #[inline(always)]
    #[must_use]
    pub fn oc5m2(&mut self) -> OC5M2_W<16> {
        OC5M2_W::new(self)
    }
    ///Bit 24 - Output compare 6 mode
    #[inline(always)]
    #[must_use]
    pub fn oc6m2(&mut self) -> OC6M2_W<24> {
        OC6M2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM1 capture/compare mode register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim1_ccmr3](index.html) module
pub struct TIM1_CCMR3_SPEC;
impl crate::RegisterSpec for TIM1_CCMR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim1_ccmr3::R](R) reader structure
impl crate::Readable for TIM1_CCMR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim1_ccmr3::W](W) writer structure
impl crate::Writable for TIM1_CCMR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM1_CCMR3 to value 0
impl crate::Resettable for TIM1_CCMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
