///Register `CCMR2_output` reader
pub struct R(crate::R<CCMR2_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR2_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR2_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR2_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR2_output` writer
pub struct W(crate::W<CCMR2_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR2_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR2_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR2_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC3S` reader - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER).
pub type CC3S_R = crate::FieldReader<u8, u8>;
///Field `CC3S` writer - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER).
pub type CC3S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR2_OUTPUT_SPEC, u8, u8, 2, O>;
///Field `OC3FE` reader - Output compare 3 fast enable Refer to OC1FE description.
pub type OC3FE_R = crate::BitReader<bool>;
///Field `OC3FE` writer - Output compare 3 fast enable Refer to OC1FE description.
pub type OC3FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
///Field `OC3PE` reader - Output compare 3 preload enable Refer to OC1PE description.
pub type OC3PE_R = crate::BitReader<bool>;
///Field `OC3PE` writer - Output compare 3 preload enable Refer to OC1PE description.
pub type OC3PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
///Field `OC3M1` reader - Output compare 3 mode Refer to OC1M\[3:0\]
///description.
pub type OC3M1_R = crate::FieldReader<u8, u8>;
///Field `OC3M1` writer - Output compare 3 mode Refer to OC1M\[3:0\]
///description.
pub type OC3M1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR2_OUTPUT_SPEC, u8, u8, 3, O>;
///Field `OC3CE` reader - Output compare 3 clear enable Refer to OC1CE description.
pub type OC3CE_R = crate::BitReader<bool>;
///Field `OC3CE` writer - Output compare 3 clear enable Refer to OC1CE description.
pub type OC3CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
///Field `CC4S` reader - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER).
pub type CC4S_R = crate::FieldReader<u8, u8>;
///Field `CC4S` writer - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER).
pub type CC4S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR2_OUTPUT_SPEC, u8, u8, 2, O>;
///Field `OC4FE` reader - Output compare 4 fast enable Refer to OC1FE description.
pub type OC4FE_R = crate::BitReader<bool>;
///Field `OC4FE` writer - Output compare 4 fast enable Refer to OC1FE description.
pub type OC4FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
///Field `OC4PE` reader - Output compare 4 preload enable Refer to OC1PE description.
pub type OC4PE_R = crate::BitReader<bool>;
///Field `OC4PE` writer - Output compare 4 preload enable Refer to OC1PE description.
pub type OC4PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
///Field `OC4M1` reader - Output compare 4 mode Refer to OC3M\[3:0\]
///description.
pub type OC4M1_R = crate::FieldReader<u8, u8>;
///Field `OC4M1` writer - Output compare 4 mode Refer to OC3M\[3:0\]
///description.
pub type OC4M1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR2_OUTPUT_SPEC, u8, u8, 3, O>;
///Field `OC4CE` reader - Output compare 4 clear enable Refer to OC1CE description.
pub type OC4CE_R = crate::BitReader<bool>;
///Field `OC4CE` writer - Output compare 4 clear enable Refer to OC1CE description.
pub type OC4CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
///Field `OC3M2` reader - Output compare 3 mode Refer to OC1M\[3:0\]
///description.
pub type OC3M2_R = crate::BitReader<bool>;
///Field `OC3M2` writer - Output compare 3 mode Refer to OC1M\[3:0\]
///description.
pub type OC3M2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
///Field `OC4M2` reader - Output compare 4 mode Refer to OC3M\[3:0\]
///description.
pub type OC4M2_R = crate::BitReader<bool>;
///Field `OC4M2` writer - Output compare 4 mode Refer to OC3M\[3:0\]
///description.
pub type OC4M2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER).
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output compare 3 fast enable Refer to OC1FE description.
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output compare 3 preload enable Refer to OC1PE description.
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Output compare 3 mode Refer to OC1M\[3:0\]
    ///description.
    #[inline(always)]
    pub fn oc3m1(&self) -> OC3M1_R {
        OC3M1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 3 clear enable Refer to OC1CE description.
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER).
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Output compare 4 fast enable Refer to OC1FE description.
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output compare 4 preload enable Refer to OC1PE description.
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Output compare 4 mode Refer to OC3M\[3:0\]
    ///description.
    #[inline(always)]
    pub fn oc4m1(&self) -> OC4M1_R {
        OC4M1_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 4 clear enable Refer to OC1CE description.
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Output compare 3 mode Refer to OC1M\[3:0\]
    ///description.
    #[inline(always)]
    pub fn oc3m2(&self) -> OC3M2_R {
        OC3M2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output compare 4 mode Refer to OC3M\[3:0\]
    ///description.
    #[inline(always)]
    pub fn oc4m2(&self) -> OC4M2_R {
        OC4M2_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER).
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CC3S_W<0> {
        CC3S_W::new(self)
    }
    ///Bit 2 - Output compare 3 fast enable Refer to OC1FE description.
    #[inline(always)]
    #[must_use]
    pub fn oc3fe(&mut self) -> OC3FE_W<2> {
        OC3FE_W::new(self)
    }
    ///Bit 3 - Output compare 3 preload enable Refer to OC1PE description.
    #[inline(always)]
    #[must_use]
    pub fn oc3pe(&mut self) -> OC3PE_W<3> {
        OC3PE_W::new(self)
    }
    ///Bits 4:6 - Output compare 3 mode Refer to OC1M\[3:0\]
    ///description.
    #[inline(always)]
    #[must_use]
    pub fn oc3m1(&mut self) -> OC3M1_W<4> {
        OC3M1_W::new(self)
    }
    ///Bit 7 - Output compare 3 clear enable Refer to OC1CE description.
    #[inline(always)]
    #[must_use]
    pub fn oc3ce(&mut self) -> OC3CE_W<7> {
        OC3CE_W::new(self)
    }
    ///Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER).
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CC4S_W<8> {
        CC4S_W::new(self)
    }
    ///Bit 10 - Output compare 4 fast enable Refer to OC1FE description.
    #[inline(always)]
    #[must_use]
    pub fn oc4fe(&mut self) -> OC4FE_W<10> {
        OC4FE_W::new(self)
    }
    ///Bit 11 - Output compare 4 preload enable Refer to OC1PE description.
    #[inline(always)]
    #[must_use]
    pub fn oc4pe(&mut self) -> OC4PE_W<11> {
        OC4PE_W::new(self)
    }
    ///Bits 12:14 - Output compare 4 mode Refer to OC3M\[3:0\]
    ///description.
    #[inline(always)]
    #[must_use]
    pub fn oc4m1(&mut self) -> OC4M1_W<12> {
        OC4M1_W::new(self)
    }
    ///Bit 15 - Output compare 4 clear enable Refer to OC1CE description.
    #[inline(always)]
    #[must_use]
    pub fn oc4ce(&mut self) -> OC4CE_W<15> {
        OC4CE_W::new(self)
    }
    ///Bit 16 - Output compare 3 mode Refer to OC1M\[3:0\]
    ///description.
    #[inline(always)]
    #[must_use]
    pub fn oc3m2(&mut self) -> OC3M2_W<16> {
        OC3M2_W::new(self)
    }
    ///Bit 24 - Output compare 4 mode Refer to OC3M\[3:0\]
    ///description.
    #[inline(always)]
    #[must_use]
    pub fn oc4m2(&mut self) -> OC4M2_W<24> {
        OC4M2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM1 capture/compare mode register 2 \[alternate\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr2_output](index.html) module
pub struct CCMR2_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR2_OUTPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr2_output::R](R) reader structure
impl crate::Readable for CCMR2_OUTPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr2_output::W](W) writer structure
impl crate::Writable for CCMR2_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCMR2_output to value 0
impl crate::Resettable for CCMR2_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
