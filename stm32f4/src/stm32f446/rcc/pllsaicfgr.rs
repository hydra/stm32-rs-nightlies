///Register `PLLSAICFGR` reader
pub struct R(crate::R<PLLSAICFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSAICFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSAICFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSAICFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PLLSAICFGR` writer
pub struct W(crate::W<PLLSAICFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSAICFGR_SPEC>;
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
impl From<crate::W<PLLSAICFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSAICFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLLSAIM` reader - Division factor for audio PLLSAI input clock
pub type PLLSAIM_R = crate::FieldReader<u8, u8>;
///Field `PLLSAIM` writer - Division factor for audio PLLSAI input clock
pub type PLLSAIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAICFGR_SPEC, u8, u8, 6, O>;
///Field `PLLSAIN` reader - PLLSAI division factor for VCO
pub type PLLSAIN_R = crate::FieldReader<u16, u16>;
///Field `PLLSAIN` writer - PLLSAI division factor for VCO
pub type PLLSAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAICFGR_SPEC, u16, u16, 9, O>;
///Field `PLLSAIP` reader - PLLSAI division factor for 48 MHz clock
pub type PLLSAIP_R = crate::FieldReader<u8, PLLSAIP_A>;
///PLLSAI division factor for 48 MHz clock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAIP_A {
    ///0: PLL*P=2
    Div2 = 0,
    ///1: PLL*P=4
    Div4 = 1,
    ///2: PLL*P=6
    Div6 = 2,
    ///3: PLL*P=8
    Div8 = 3,
}
impl From<PLLSAIP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIP_A) -> Self {
        variant as _
    }
}
impl PLLSAIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAIP_A {
        match self.bits {
            0 => PLLSAIP_A::Div2,
            1 => PLLSAIP_A::Div4,
            2 => PLLSAIP_A::Div6,
            3 => PLLSAIP_A::Div8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIP_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIP_A::Div4
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAIP_A::Div6
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIP_A::Div8
    }
}
///Field `PLLSAIP` writer - PLLSAI division factor for 48 MHz clock
pub type PLLSAIP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLLSAICFGR_SPEC, u8, PLLSAIP_A, 2, O>;
impl<'a, const O: u8> PLLSAIP_W<'a, O> {
    ///PLL*P=2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAIP_A::Div2)
    }
    ///PLL*P=4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAIP_A::Div4)
    }
    ///PLL*P=6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAIP_A::Div6)
    }
    ///PLL*P=8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAIP_A::Div8)
    }
}
///Field `PLLSAIQ` reader - PLLSAI division factor for SAIs clock
pub type PLLSAIQ_R = crate::FieldReader<u8, u8>;
///Field `PLLSAIQ` writer - PLLSAI division factor for SAIs clock
pub type PLLSAIQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAICFGR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:5 - Division factor for audio PLLSAI input clock
    #[inline(always)]
    pub fn pllsaim(&self) -> PLLSAIM_R {
        PLLSAIM_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:14 - PLLSAI division factor for VCO
    #[inline(always)]
    pub fn pllsain(&self) -> PLLSAIN_R {
        PLLSAIN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    ///Bits 16:17 - PLLSAI division factor for 48 MHz clock
    #[inline(always)]
    pub fn pllsaip(&self) -> PLLSAIP_R {
        PLLSAIP_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 24:27 - PLLSAI division factor for SAIs clock
    #[inline(always)]
    pub fn pllsaiq(&self) -> PLLSAIQ_R {
        PLLSAIQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:5 - Division factor for audio PLLSAI input clock
    #[inline(always)]
    #[must_use]
    pub fn pllsaim(&mut self) -> PLLSAIM_W<0> {
        PLLSAIM_W::new(self)
    }
    ///Bits 6:14 - PLLSAI division factor for VCO
    #[inline(always)]
    #[must_use]
    pub fn pllsain(&mut self) -> PLLSAIN_W<6> {
        PLLSAIN_W::new(self)
    }
    ///Bits 16:17 - PLLSAI division factor for 48 MHz clock
    #[inline(always)]
    #[must_use]
    pub fn pllsaip(&mut self) -> PLLSAIP_W<16> {
        PLLSAIP_W::new(self)
    }
    ///Bits 24:27 - PLLSAI division factor for SAIs clock
    #[inline(always)]
    #[must_use]
    pub fn pllsaiq(&mut self) -> PLLSAIQ_W<24> {
        PLLSAIQ_W::new(self)
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
///For information about available fields see [pllsaicfgr](index.html) module
pub struct PLLSAICFGR_SPEC;
impl crate::RegisterSpec for PLLSAICFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pllsaicfgr::R](R) reader structure
impl crate::Readable for PLLSAICFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pllsaicfgr::W](W) writer structure
impl crate::Writable for PLLSAICFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PLLSAICFGR to value 0x2400_3000
impl crate::Resettable for PLLSAICFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2400_3000;
}
