///Register `CFR` reader
pub struct R(crate::R<CFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFR` writer
pub struct W(crate::W<CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFR_SPEC>;
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
impl From<crate::W<CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `W` reader - 7-bit window value These bits contain the window value to be compared with the down-counter.
pub type W_R = crate::FieldReader<u8, u8>;
///Field `W` writer - 7-bit window value These bits contain the window value to be compared with the down-counter.
pub type W_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFR_SPEC, u8, u8, 7, O>;
///Field `EWI` reader - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
pub type EWI_R = crate::BitReader<EWIW_A>;
///Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIW_A {
    ///1: interrupt occurs whenever the counter reaches the value 0x40
    Enable = 1,
}
impl From<EWIW_A> for bool {
    #[inline(always)]
    fn from(variant: EWIW_A) -> Self {
        variant as u8 != 0
    }
}
impl EWI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EWIW_A> {
        match self.bits {
            true => Some(EWIW_A::Enable),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Enable`
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EWIW_A::Enable
    }
}
///Field `EWI` writer - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
pub type EWI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, EWIW_A, O>;
impl<'a, const O: u8> EWI_W<'a, O> {
    ///interrupt occurs whenever the counter reaches the value 0x40
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EWIW_A::Enable)
    }
}
///Field `WDGTB` reader - Timer base The timebase of the prescaler can be modified as follows:
pub type WDGTB_R = crate::FieldReader<u8, WDGTB_A>;
///Timer base The timebase of the prescaler can be modified as follows:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDGTB_A {
    ///0: Counter clock (PCLK1 div 4096) div 1
    Div1 = 0,
    ///1: Counter clock (PCLK1 div 4096) div 2
    Div2 = 1,
    ///2: Counter clock (PCLK1 div 4096) div 4
    Div4 = 2,
    ///3: Counter clock (PCLK1 div 4096) div 8
    Div8 = 3,
    ///4: Counter clock (PCLK1 div 4096) div 16
    Div16 = 4,
    ///5: Counter clock (PCLK1 div 4096) div 32
    Div32 = 5,
    ///6: Counter clock (PCLK1 div 4096) div 64
    Div64 = 6,
    ///7: Counter clock (PCLK1 div 4096) div 128
    Div128 = 7,
}
impl From<WDGTB_A> for u8 {
    #[inline(always)]
    fn from(variant: WDGTB_A) -> Self {
        variant as _
    }
}
impl WDGTB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WDGTB_A {
        match self.bits {
            0 => WDGTB_A::Div1,
            1 => WDGTB_A::Div2,
            2 => WDGTB_A::Div4,
            3 => WDGTB_A::Div8,
            4 => WDGTB_A::Div16,
            5 => WDGTB_A::Div32,
            6 => WDGTB_A::Div64,
            7 => WDGTB_A::Div128,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == WDGTB_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == WDGTB_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == WDGTB_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == WDGTB_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == WDGTB_A::Div16
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == WDGTB_A::Div32
    }
    ///Checks if the value of the field is `Div64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == WDGTB_A::Div64
    }
    ///Checks if the value of the field is `Div128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == WDGTB_A::Div128
    }
}
///Field `WDGTB` writer - Timer base The timebase of the prescaler can be modified as follows:
pub type WDGTB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFR_SPEC, u8, WDGTB_A, 3, O>;
impl<'a, const O: u8> WDGTB_W<'a, O> {
    ///Counter clock (PCLK1 div 4096) div 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(WDGTB_A::Div1)
    }
    ///Counter clock (PCLK1 div 4096) div 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(WDGTB_A::Div2)
    }
    ///Counter clock (PCLK1 div 4096) div 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(WDGTB_A::Div4)
    }
    ///Counter clock (PCLK1 div 4096) div 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(WDGTB_A::Div8)
    }
    ///Counter clock (PCLK1 div 4096) div 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(WDGTB_A::Div16)
    }
    ///Counter clock (PCLK1 div 4096) div 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(WDGTB_A::Div32)
    }
    ///Counter clock (PCLK1 div 4096) div 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(WDGTB_A::Div64)
    }
    ///Counter clock (PCLK1 div 4096) div 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(WDGTB_A::Div128)
    }
}
impl R {
    ///Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter.
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    ///Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter.
    #[inline(always)]
    #[must_use]
    pub fn w(&mut self) -> W_W<0> {
        W_W::new(self)
    }
    ///Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EWI_W<9> {
        EWI_W::new(self)
    }
    ///Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:
    #[inline(always)]
    #[must_use]
    pub fn wdgtb(&mut self) -> WDGTB_W<11> {
        WDGTB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfr](index.html) module
pub struct CFR_SPEC;
impl crate::RegisterSpec for CFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfr::R](R) reader structure
impl crate::Readable for CFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfr::W](W) writer structure
impl crate::Writable for CFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFR to value 0x7f
impl crate::Resettable for CFR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
