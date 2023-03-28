///Register `CCMR1_Output` reader
pub struct R(crate::R<CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR1_Output` writer
pub struct W(crate::W<CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR1_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC1S` reader - Capture/Compare 1 selection
pub type CC1S_R = crate::FieldReader<u8, u8>;
///Field `CC1S` writer - Capture/Compare 1 selection
pub type CC1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_OUTPUT_SPEC, u8, u8, 2, O>;
///Field `OC1FE` reader - Output compare 1 fast enable
pub type OC1FE_R = crate::BitReader<bool>;
///Field `OC1FE` writer - Output compare 1 fast enable
pub type OC1FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC1PE` reader - Output compare 1 preload enable
pub type OC1PE_R = crate::BitReader<bool>;
///Field `OC1PE` writer - Output compare 1 preload enable
pub type OC1PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC1M` reader - Output compare 1 mode
pub type OC1M_R = crate::FieldReader<u8, u8>;
///Field `OC1M` writer - Output compare 1 mode
pub type OC1M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_OUTPUT_SPEC, u8, u8, 3, O>;
///Field `OC1CE` reader - Output compare 1 clear enable
pub type OC1CE_R = crate::BitReader<bool>;
///Field `OC1CE` writer - Output compare 1 clear enable
pub type OC1CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `CC2S` reader - Capture/Compare 2 selection
pub type CC2S_R = crate::FieldReader<u8, u8>;
///Field `CC2S` writer - Capture/Compare 2 selection
pub type CC2S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_OUTPUT_SPEC, u8, u8, 2, O>;
///Field `OC2FE` reader - Output compare 2 fast enable
pub type OC2FE_R = crate::BitReader<bool>;
///Field `OC2FE` writer - Output compare 2 fast enable
pub type OC2FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC2PE` reader - Output compare 2 preload enable
pub type OC2PE_R = crate::BitReader<bool>;
///Field `OC2PE` writer - Output compare 2 preload enable
pub type OC2PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC2M` reader - Output compare 2 mode
pub type OC2M_R = crate::FieldReader<u8, u8>;
///Field `OC2M` writer - Output compare 2 mode
pub type OC2M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_OUTPUT_SPEC, u8, u8, 3, O>;
///Field `OC2CE` reader - Output compare 2 clear enable
pub type OC2CE_R = crate::BitReader<bool>;
///Field `OC2CE` writer - Output compare 2 clear enable
pub type OC2CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC1M_bit3` reader - Output Compare 1 mode - bit 3
pub type OC1M_BIT3_R = crate::BitReader<bool>;
///Field `OC1M_bit3` writer - Output Compare 1 mode - bit 3
pub type OC1M_BIT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
///Field `OC2M_bit3` reader - Output Compare 2 mode - bit 3
pub type OC2M_BIT3_R = crate::BitReader<bool>;
///Field `OC2M_bit3` writer - Output Compare 2 mode - bit 3
pub type OC2M_BIT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Output compare 1 mode
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Capture/Compare 2 selection
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Output compare 2 fast enable
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output compare 2 preload enable
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Output compare 2 mode
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 2 clear enable
    #[inline(always)]
    pub fn oc2ce(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Output Compare 1 mode - bit 3
    #[inline(always)]
    pub fn oc1m_bit3(&self) -> OC1M_BIT3_R {
        OC1M_BIT3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output Compare 2 mode - bit 3
    #[inline(always)]
    pub fn oc2m_bit3(&self) -> OC2M_BIT3_R {
        OC2M_BIT3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    ///Bit 2 - Output compare 1 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<2> {
        OC1FE_W::new(self)
    }
    ///Bit 3 - Output compare 1 preload enable
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<3> {
        OC1PE_W::new(self)
    }
    ///Bits 4:6 - Output compare 1 mode
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OC1M_W<4> {
        OC1M_W::new(self)
    }
    ///Bit 7 - Output compare 1 clear enable
    #[inline(always)]
    #[must_use]
    pub fn oc1ce(&mut self) -> OC1CE_W<7> {
        OC1CE_W::new(self)
    }
    ///Bits 8:9 - Capture/Compare 2 selection
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<8> {
        CC2S_W::new(self)
    }
    ///Bit 10 - Output compare 2 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc2fe(&mut self) -> OC2FE_W<10> {
        OC2FE_W::new(self)
    }
    ///Bit 11 - Output compare 2 preload enable
    #[inline(always)]
    #[must_use]
    pub fn oc2pe(&mut self) -> OC2PE_W<11> {
        OC2PE_W::new(self)
    }
    ///Bits 12:14 - Output compare 2 mode
    #[inline(always)]
    #[must_use]
    pub fn oc2m(&mut self) -> OC2M_W<12> {
        OC2M_W::new(self)
    }
    ///Bit 15 - Output compare 2 clear enable
    #[inline(always)]
    #[must_use]
    pub fn oc2ce(&mut self) -> OC2CE_W<15> {
        OC2CE_W::new(self)
    }
    ///Bit 16 - Output Compare 1 mode - bit 3
    #[inline(always)]
    #[must_use]
    pub fn oc1m_bit3(&mut self) -> OC1M_BIT3_W<16> {
        OC1M_BIT3_W::new(self)
    }
    ///Bit 24 - Output Compare 2 mode - bit 3
    #[inline(always)]
    #[must_use]
    pub fn oc2m_bit3(&mut self) -> OC2M_BIT3_W<24> {
        OC2M_BIT3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare mode register 1 (output mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr1_output](index.html) module
pub struct CCMR1_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR1_OUTPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr1_output::R](R) reader structure
impl crate::Readable for CCMR1_OUTPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr1_output::W](W) writer structure
impl crate::Writable for CCMR1_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCMR1_Output to value 0
impl crate::Resettable for CCMR1_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
