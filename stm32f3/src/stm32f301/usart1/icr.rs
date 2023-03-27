///Register `ICR` reader
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ICR` writer
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PECF` reader - Parity error clear flag
pub type PECF_R = crate::BitReader<PECF_A>;
///Parity error clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECF_A {
    ///1: Clears the PE flag in the ISR register
    Clear = 1,
}
impl From<PECF_A> for bool {
    #[inline(always)]
    fn from(variant: PECF_A) -> Self {
        variant as u8 != 0
    }
}
impl PECF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PECF_A> {
        match self.bits {
            true => Some(PECF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PECF_A::Clear
    }
}
///Field `PECF` writer - Parity error clear flag
pub type PECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, PECF_A, O>;
impl<'a, const O: u8> PECF_W<'a, O> {
    ///Clears the PE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECF_A::Clear)
    }
}
///Field `FECF` reader - Framing error clear flag
pub type FECF_R = crate::BitReader<FECF_A>;
///Framing error clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FECF_A {
    ///1: Clears the FE flag in the ISR register
    Clear = 1,
}
impl From<FECF_A> for bool {
    #[inline(always)]
    fn from(variant: FECF_A) -> Self {
        variant as u8 != 0
    }
}
impl FECF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FECF_A> {
        match self.bits {
            true => Some(FECF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FECF_A::Clear
    }
}
///Field `FECF` writer - Framing error clear flag
pub type FECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, FECF_A, O>;
impl<'a, const O: u8> FECF_W<'a, O> {
    ///Clears the FE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FECF_A::Clear)
    }
}
///Field `NCF` reader - Noise detected clear flag
pub type NCF_R = crate::BitReader<NCF_A>;
///Noise detected clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCF_A {
    ///1: Clears the NF flag in the ISR register
    Clear = 1,
}
impl From<NCF_A> for bool {
    #[inline(always)]
    fn from(variant: NCF_A) -> Self {
        variant as u8 != 0
    }
}
impl NCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<NCF_A> {
        match self.bits {
            true => Some(NCF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == NCF_A::Clear
    }
}
///Field `NCF` writer - Noise detected clear flag
pub type NCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, NCF_A, O>;
impl<'a, const O: u8> NCF_W<'a, O> {
    ///Clears the NF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NCF_A::Clear)
    }
}
///Field `ORECF` reader - Overrun error clear flag
pub type ORECF_R = crate::BitReader<ORECF_A>;
///Overrun error clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORECF_A {
    ///1: Clears the ORE flag in the ISR register
    Clear = 1,
}
impl From<ORECF_A> for bool {
    #[inline(always)]
    fn from(variant: ORECF_A) -> Self {
        variant as u8 != 0
    }
}
impl ORECF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<ORECF_A> {
        match self.bits {
            true => Some(ORECF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ORECF_A::Clear
    }
}
///Field `ORECF` writer - Overrun error clear flag
pub type ORECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ORECF_A, O>;
impl<'a, const O: u8> ORECF_W<'a, O> {
    ///Clears the ORE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ORECF_A::Clear)
    }
}
///Field `IDLECF` reader - Idle line detected clear flag
pub type IDLECF_R = crate::BitReader<IDLECF_A>;
///Idle line detected clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLECF_A {
    ///1: Clears the IDLE flag in the ISR register
    Clear = 1,
}
impl From<IDLECF_A> for bool {
    #[inline(always)]
    fn from(variant: IDLECF_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLECF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECF_A> {
        match self.bits {
            true => Some(IDLECF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == IDLECF_A::Clear
    }
}
///Field `IDLECF` writer - Idle line detected clear flag
pub type IDLECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, IDLECF_A, O>;
impl<'a, const O: u8> IDLECF_W<'a, O> {
    ///Clears the IDLE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDLECF_A::Clear)
    }
}
///Field `TCCF` reader - Transmission complete clear flag
pub type TCCF_R = crate::BitReader<TCCF_A>;
///Transmission complete clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCCF_A {
    ///1: Clears the TC flag in the ISR register
    Clear = 1,
}
impl From<TCCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCCF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TCCF_A> {
        match self.bits {
            true => Some(TCCF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TCCF_A::Clear
    }
}
///Field `TCCF` writer - Transmission complete clear flag
pub type TCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, TCCF_A, O>;
impl<'a, const O: u8> TCCF_W<'a, O> {
    ///Clears the TC flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCCF_A::Clear)
    }
}
///Field `LBDCF` reader - LIN break detection clear flag
pub type LBDCF_R = crate::BitReader<LBDCF_A>;
///LIN break detection clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDCF_A {
    ///1: Clears the LBDF flag in the ISR register
    Clear = 1,
}
impl From<LBDCF_A> for bool {
    #[inline(always)]
    fn from(variant: LBDCF_A) -> Self {
        variant as u8 != 0
    }
}
impl LBDCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<LBDCF_A> {
        match self.bits {
            true => Some(LBDCF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == LBDCF_A::Clear
    }
}
///Field `LBDCF` writer - LIN break detection clear flag
pub type LBDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, LBDCF_A, O>;
impl<'a, const O: u8> LBDCF_W<'a, O> {
    ///Clears the LBDF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LBDCF_A::Clear)
    }
}
///Field `CTSCF` reader - CTS clear flag
pub type CTSCF_R = crate::BitReader<CTSCF_A>;
///CTS clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSCF_A {
    ///1: Clears the CTSIF flag in the ISR register
    Clear = 1,
}
impl From<CTSCF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSCF_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CTSCF_A> {
        match self.bits {
            true => Some(CTSCF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTSCF_A::Clear
    }
}
///Field `CTSCF` writer - CTS clear flag
pub type CTSCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CTSCF_A, O>;
impl<'a, const O: u8> CTSCF_W<'a, O> {
    ///Clears the CTSIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSCF_A::Clear)
    }
}
///Field `RTOCF` reader - Receiver timeout clear flag
pub type RTOCF_R = crate::BitReader<RTOCF_A>;
///Receiver timeout clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOCF_A {
    ///1: Clears the RTOF flag in the ISR register
    Clear = 1,
}
impl From<RTOCF_A> for bool {
    #[inline(always)]
    fn from(variant: RTOCF_A) -> Self {
        variant as u8 != 0
    }
}
impl RTOCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RTOCF_A> {
        match self.bits {
            true => Some(RTOCF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RTOCF_A::Clear
    }
}
///Field `RTOCF` writer - Receiver timeout clear flag
pub type RTOCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, RTOCF_A, O>;
impl<'a, const O: u8> RTOCF_W<'a, O> {
    ///Clears the RTOF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RTOCF_A::Clear)
    }
}
///Field `EOBCF` reader - End of timeout clear flag
pub type EOBCF_R = crate::BitReader<EOBCF_A>;
///End of timeout clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBCF_A {
    ///1: Clears the EOBF flag in the ISR register
    Clear = 1,
}
impl From<EOBCF_A> for bool {
    #[inline(always)]
    fn from(variant: EOBCF_A) -> Self {
        variant as u8 != 0
    }
}
impl EOBCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EOBCF_A> {
        match self.bits {
            true => Some(EOBCF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EOBCF_A::Clear
    }
}
///Field `EOBCF` writer - End of timeout clear flag
pub type EOBCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, EOBCF_A, O>;
impl<'a, const O: u8> EOBCF_W<'a, O> {
    ///Clears the EOBF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOBCF_A::Clear)
    }
}
///Field `CMCF` reader - Character match clear flag
pub type CMCF_R = crate::BitReader<CMCF_A>;
///Character match clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMCF_A {
    ///1: Clears the CMF flag in the ISR register
    Clear = 1,
}
impl From<CMCF_A> for bool {
    #[inline(always)]
    fn from(variant: CMCF_A) -> Self {
        variant as u8 != 0
    }
}
impl CMCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CMCF_A> {
        match self.bits {
            true => Some(CMCF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMCF_A::Clear
    }
}
///Field `CMCF` writer - Character match clear flag
pub type CMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CMCF_A, O>;
impl<'a, const O: u8> CMCF_W<'a, O> {
    ///Clears the CMF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMCF_A::Clear)
    }
}
///Field `WUCF` reader - Wakeup from Stop mode clear flag
pub type WUCF_R = crate::BitReader<WUCF_A>;
///Wakeup from Stop mode clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUCF_A {
    ///1: Clears the WUF flag in the ISR register
    Clear = 1,
}
impl From<WUCF_A> for bool {
    #[inline(always)]
    fn from(variant: WUCF_A) -> Self {
        variant as u8 != 0
    }
}
impl WUCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WUCF_A> {
        match self.bits {
            true => Some(WUCF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUCF_A::Clear
    }
}
///Field `WUCF` writer - Wakeup from Stop mode clear flag
pub type WUCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, WUCF_A, O>;
impl<'a, const O: u8> WUCF_W<'a, O> {
    ///Clears the WUF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUCF_A::Clear)
    }
}
impl R {
    ///Bit 0 - Parity error clear flag
    #[inline(always)]
    pub fn pecf(&self) -> PECF_R {
        PECF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error clear flag
    #[inline(always)]
    pub fn fecf(&self) -> FECF_R {
        FECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Noise detected clear flag
    #[inline(always)]
    pub fn ncf(&self) -> NCF_R {
        NCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error clear flag
    #[inline(always)]
    pub fn orecf(&self) -> ORECF_R {
        ORECF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Idle line detected clear flag
    #[inline(always)]
    pub fn idlecf(&self) -> IDLECF_R {
        IDLECF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Transmission complete clear flag
    #[inline(always)]
    pub fn tccf(&self) -> TCCF_R {
        TCCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - LIN break detection clear flag
    #[inline(always)]
    pub fn lbdcf(&self) -> LBDCF_R {
        LBDCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS clear flag
    #[inline(always)]
    pub fn ctscf(&self) -> CTSCF_R {
        CTSCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Receiver timeout clear flag
    #[inline(always)]
    pub fn rtocf(&self) -> RTOCF_R {
        RTOCF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - End of timeout clear flag
    #[inline(always)]
    pub fn eobcf(&self) -> EOBCF_R {
        EOBCF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 17 - Character match clear flag
    #[inline(always)]
    pub fn cmcf(&self) -> CMCF_R {
        CMCF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Wakeup from Stop mode clear flag
    #[inline(always)]
    pub fn wucf(&self) -> WUCF_R {
        WUCF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Parity error clear flag
    #[inline(always)]
    #[must_use]
    pub fn pecf(&mut self) -> PECF_W<0> {
        PECF_W::new(self)
    }
    ///Bit 1 - Framing error clear flag
    #[inline(always)]
    #[must_use]
    pub fn fecf(&mut self) -> FECF_W<1> {
        FECF_W::new(self)
    }
    ///Bit 2 - Noise detected clear flag
    #[inline(always)]
    #[must_use]
    pub fn ncf(&mut self) -> NCF_W<2> {
        NCF_W::new(self)
    }
    ///Bit 3 - Overrun error clear flag
    #[inline(always)]
    #[must_use]
    pub fn orecf(&mut self) -> ORECF_W<3> {
        ORECF_W::new(self)
    }
    ///Bit 4 - Idle line detected clear flag
    #[inline(always)]
    #[must_use]
    pub fn idlecf(&mut self) -> IDLECF_W<4> {
        IDLECF_W::new(self)
    }
    ///Bit 6 - Transmission complete clear flag
    #[inline(always)]
    #[must_use]
    pub fn tccf(&mut self) -> TCCF_W<6> {
        TCCF_W::new(self)
    }
    ///Bit 8 - LIN break detection clear flag
    #[inline(always)]
    #[must_use]
    pub fn lbdcf(&mut self) -> LBDCF_W<8> {
        LBDCF_W::new(self)
    }
    ///Bit 9 - CTS clear flag
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CTSCF_W<9> {
        CTSCF_W::new(self)
    }
    ///Bit 11 - Receiver timeout clear flag
    #[inline(always)]
    #[must_use]
    pub fn rtocf(&mut self) -> RTOCF_W<11> {
        RTOCF_W::new(self)
    }
    ///Bit 12 - End of timeout clear flag
    #[inline(always)]
    #[must_use]
    pub fn eobcf(&mut self) -> EOBCF_W<12> {
        EOBCF_W::new(self)
    }
    ///Bit 17 - Character match clear flag
    #[inline(always)]
    #[must_use]
    pub fn cmcf(&mut self) -> CMCF_W<17> {
        CMCF_W::new(self)
    }
    ///Bit 20 - Wakeup from Stop mode clear flag
    #[inline(always)]
    #[must_use]
    pub fn wucf(&mut self) -> WUCF_W<20> {
        WUCF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](index.html) module
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [icr::R](R) reader structure
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [icr::W](W) writer structure
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
