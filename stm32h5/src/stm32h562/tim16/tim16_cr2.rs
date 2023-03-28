///Register `TIM16_CR2` reader
pub struct R(crate::R<TIM16_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM16_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM16_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM16_CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM16_CR2` writer
pub struct W(crate::W<TIM16_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM16_CR2_SPEC>;
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
impl From<crate::W<TIM16_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM16_CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
pub type CCPC_R = crate::BitReader<bool>;
///Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
pub type CCPC_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM16_CR2_SPEC, bool, O>;
///Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
pub type CCUS_R = crate::BitReader<bool>;
///Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
pub type CCUS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM16_CR2_SPEC, bool, O>;
///Field `CCDS` reader - Capture/compare DMA selection
pub type CCDS_R = crate::BitReader<bool>;
///Field `CCDS` writer - Capture/compare DMA selection
pub type CCDS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM16_CR2_SPEC, bool, O>;
///Field `OIS1` reader - Output Idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register).
pub type OIS1_R = crate::BitReader<bool>;
///Field `OIS1` writer - Output Idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register).
pub type OIS1_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM16_CR2_SPEC, bool, O>;
///Field `OIS1N` reader - Output Idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register).
pub type OIS1N_R = crate::BitReader<bool>;
///Field `OIS1N` writer - Output Idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register).
pub type OIS1N_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM16_CR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Output Idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register).
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output Idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register).
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<0> {
        CCPC_W::new(self)
    }
    ///Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<2> {
        CCUS_W::new(self)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<3> {
        CCDS_W::new(self)
    }
    ///Bit 8 - Output Idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register).
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<8> {
        OIS1_W::new(self)
    }
    ///Bit 9 - Output Idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register).
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<9> {
        OIS1N_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM16 control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim16_cr2](index.html) module
pub struct TIM16_CR2_SPEC;
impl crate::RegisterSpec for TIM16_CR2_SPEC {
    type Ux = u16;
}
///`read()` method returns [tim16_cr2::R](R) reader structure
impl crate::Readable for TIM16_CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim16_cr2::W](W) writer structure
impl crate::Writable for TIM16_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM16_CR2 to value 0
impl crate::Resettable for TIM16_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
