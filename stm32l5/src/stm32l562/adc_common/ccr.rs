///Register `CCR` reader
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR` writer
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DUAL` reader - DUAL
pub type DUAL_R = crate::FieldReader<u8, u8>;
///Field `DUAL` writer - DUAL
pub type DUAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 5, O>;
///Field `DELAY` reader - DELAY
pub type DELAY_R = crate::FieldReader<u8, u8>;
///Field `DELAY` writer - DELAY
pub type DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 3, O>;
///Field `DMACFG` reader - DMACFG
pub type DMACFG_R = crate::BitReader<bool>;
///Field `DMACFG` writer - DMACFG
pub type DMACFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
///Field `MDMA` reader - MDMA
pub type MDMA_R = crate::FieldReader<u8, u8>;
///Field `MDMA` writer - MDMA
pub type MDMA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
///Field `CKMODE` reader - ADC clock mode
pub type CKMODE_R = crate::FieldReader<u8, u8>;
///Field `CKMODE` writer - ADC clock mode
pub type CKMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
///Field `PRESC` reader - ADC prescaler
pub type PRESC_R = crate::FieldReader<u8, u8>;
///Field `PRESC` writer - ADC prescaler
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 4, O>;
///Field `VREFEN` reader - VREFINT enable
pub type VREFEN_R = crate::BitReader<bool>;
///Field `VREFEN` writer - VREFINT enable
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
///Field `CH17SEL` reader - CH17SEL
pub type CH17SEL_R = crate::BitReader<bool>;
///Field `CH17SEL` writer - CH17SEL
pub type CH17SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
///Field `CH18SEL` reader - CH18SEL
pub type CH18SEL_R = crate::BitReader<bool>;
///Field `CH18SEL` writer - CH18SEL
pub type CH18SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
impl R {
    ///Bits 0:4 - DUAL
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:10 - DELAY
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 13 - DMACFG
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - MDMA
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CH17SEL
    #[inline(always)]
    pub fn ch17sel(&self) -> CH17SEL_R {
        CH17SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CH18SEL
    #[inline(always)]
    pub fn ch18sel(&self) -> CH18SEL_R {
        CH18SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - DUAL
    #[inline(always)]
    #[must_use]
    pub fn dual(&mut self) -> DUAL_W<0> {
        DUAL_W::new(self)
    }
    ///Bits 8:10 - DELAY
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<8> {
        DELAY_W::new(self)
    }
    ///Bit 13 - DMACFG
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<13> {
        DMACFG_W::new(self)
    }
    ///Bits 14:15 - MDMA
    #[inline(always)]
    #[must_use]
    pub fn mdma(&mut self) -> MDMA_W<14> {
        MDMA_W::new(self)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<16> {
        CKMODE_W::new(self)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<18> {
        PRESC_W::new(self)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<22> {
        VREFEN_W::new(self)
    }
    ///Bit 23 - CH17SEL
    #[inline(always)]
    #[must_use]
    pub fn ch17sel(&mut self) -> CH17SEL_W<23> {
        CH17SEL_W::new(self)
    }
    ///Bit 24 - CH18SEL
    #[inline(always)]
    #[must_use]
    pub fn ch18sel(&mut self) -> CH18SEL_W<24> {
        CH18SEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC common control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](index.html) module
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr::R](R) reader structure
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr::W](W) writer structure
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
