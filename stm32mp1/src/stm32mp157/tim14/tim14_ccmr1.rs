///Register `TIM14_CCMR1` reader
pub struct R(crate::R<TIM14_CCMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM14_CCMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM14_CCMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM14_CCMR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM14_CCMR1` writer
pub struct W(crate::W<TIM14_CCMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM14_CCMR1_SPEC>;
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
impl From<crate::W<TIM14_CCMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM14_CCMR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC1S` reader - CC1S
pub type CC1S_R = crate::FieldReader<u8, u8>;
///Field `CC1S` writer - CC1S
pub type CC1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM14_CCMR1_SPEC, u8, u8, 2, O>;
///Field `OC1FE` reader - OC1FE
pub type OC1FE_R = crate::BitReader<bool>;
///Field `OC1FE` writer - OC1FE
pub type OC1FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM14_CCMR1_SPEC, bool, O>;
///Field `OC1PE` reader - OC1PE
pub type OC1PE_R = crate::BitReader<bool>;
///Field `OC1PE` writer - OC1PE
pub type OC1PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM14_CCMR1_SPEC, bool, O>;
///Field `OC1M` reader - OC1M
pub type OC1M_R = crate::FieldReader<u8, u8>;
///Field `OC1M` writer - OC1M
pub type OC1M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM14_CCMR1_SPEC, u8, u8, 3, O>;
///Field `OC1M3` reader - OC1M3
pub type OC1M3_R = crate::BitReader<bool>;
///Field `OC1M3` writer - OC1M3
pub type OC1M3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM14_CCMR1_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - CC1S
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - OC1FE
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OC1PE
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC1M
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 16 - OC1M3
    #[inline(always)]
    pub fn oc1m3(&self) -> OC1M3_R {
        OC1M3_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - CC1S
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    ///Bit 2 - OC1FE
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<2> {
        OC1FE_W::new(self)
    }
    ///Bit 3 - OC1PE
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<3> {
        OC1PE_W::new(self)
    }
    ///Bits 4:6 - OC1M
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OC1M_W<4> {
        OC1M_W::new(self)
    }
    ///Bit 16 - OC1M3
    #[inline(always)]
    #[must_use]
    pub fn oc1m3(&mut self) -> OC1M3_W<16> {
        OC1M3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim14_ccmr1](index.html) module
pub struct TIM14_CCMR1_SPEC;
impl crate::RegisterSpec for TIM14_CCMR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim14_ccmr1::R](R) reader structure
impl crate::Readable for TIM14_CCMR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim14_ccmr1::W](W) writer structure
impl crate::Writable for TIM14_CCMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM14_CCMR1 to value 0
impl crate::Resettable for TIM14_CCMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
