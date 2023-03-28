///Register `PLLCFGR` reader
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PLLCFGR` writer
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
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
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLLM` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM_R = crate::FieldReader<u8, u8>;
///Field `PLLM` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 6, O>;
///Field `PLLN` reader - Main PLL (PLL) multiplication factor for VCO
pub type PLLN_R = crate::FieldReader<u16, u16>;
///Field `PLLN` writer - Main PLL (PLL) multiplication factor for VCO
pub type PLLN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u16, u16, 9, O>;
///Field `PLLP` reader - Main PLL (PLL) division factor for main system clock
pub type PLLP_R = crate::FieldReader<u8, PLLP_A>;
///Main PLL (PLL) division factor for main system clock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLP_A {
    ///0: PLLP=2
    Div2 = 0,
    ///1: PLLP=4
    Div4 = 1,
    ///2: PLLP=6
    Div6 = 2,
    ///3: PLLP=8
    Div8 = 3,
}
impl From<PLLP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as _
    }
}
impl PLLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLP_A {
        match self.bits {
            0 => PLLP_A::Div2,
            1 => PLLP_A::Div4,
            2 => PLLP_A::Div6,
            3 => PLLP_A::Div8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLP_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLP_A::Div4
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLP_A::Div6
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLP_A::Div8
    }
}
///Field `PLLP` writer - Main PLL (PLL) division factor for main system clock
pub type PLLP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLLCFGR_SPEC, u8, PLLP_A, 2, O>;
impl<'a, const O: u8> PLLP_W<'a, O> {
    ///PLLP=2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLP_A::Div2)
    }
    ///PLLP=4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLP_A::Div4)
    }
    ///PLLP=6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLP_A::Div6)
    }
    ///PLLP=8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLP_A::Div8)
    }
}
///Field `PLLSRC` reader - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
pub type PLLSRC_R = crate::BitReader<PLLSRC_A>;
///Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSRC_A {
    ///0: HSI clock selected as PLL and PLLI2S clock entry
    Hsi = 0,
    ///1: HSE oscillator clock selected as PLL and PLLI2S clock entry
    Hse = 1,
}
impl From<PLLSRC_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            false => PLLSRC_A::Hsi,
            true => PLLSRC_A::Hse,
        }
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLLSRC_A::Hsi
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC_A::Hse
    }
}
///Field `PLLSRC` writer - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
pub type PLLSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, PLLSRC_A, O>;
impl<'a, const O: u8> PLLSRC_W<'a, O> {
    ///HSI clock selected as PLL and PLLI2S clock entry
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(PLLSRC_A::Hsi)
    }
    ///HSE oscillator clock selected as PLL and PLLI2S clock entry
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::Hse)
    }
}
///Field `PLLQ` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ_R = crate::FieldReader<u8, u8>;
///Field `PLLQ` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 4, O>;
///Field `PLLR` reader - Main PLL division factor for I2Ss, SAIs, SYSTEM and SPDIF-Rx clocks
pub type PLLR_R = crate::FieldReader<u8, u8>;
///Field `PLLR` writer - Main PLL division factor for I2Ss, SAIs, SYSTEM and SPDIF-Rx clocks
pub type PLLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:14 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    ///Bits 16:17 - Main PLL (PLL) division factor for main system clock
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:30 - Main PLL division factor for I2Ss, SAIs, SYSTEM and SPDIF-Rx clocks
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<0> {
        PLLM_W::new(self)
    }
    ///Bits 6:14 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<6> {
        PLLN_W::new(self)
    }
    ///Bits 16:17 - Main PLL (PLL) division factor for main system clock
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<16> {
        PLLP_W::new(self)
    }
    ///Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<22> {
        PLLSRC_W::new(self)
    }
    ///Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<24> {
        PLLQ_W::new(self)
    }
    ///Bits 28:30 - Main PLL division factor for I2Ss, SAIs, SYSTEM and SPDIF-Rx clocks
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<28> {
        PLLR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PLL configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pllcfgr](index.html) module
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pllcfgr::R](R) reader structure
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pllcfgr::W](W) writer structure
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PLLCFGR to value 0x2400_3010
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2400_3010;
}
