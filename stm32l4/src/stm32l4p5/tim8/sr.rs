///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UIF` reader - Update interrupt flag
pub type UIF_R = crate::BitReader<UIF_A>;
///Update interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIF_A {
    ///0: No update occurred
    Clear = 0,
    ///1: Update interrupt pending.
    UpdatePending = 1,
}
impl From<UIF_A> for bool {
    #[inline(always)]
    fn from(variant: UIF_A) -> Self {
        variant as u8 != 0
    }
}
impl UIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UIF_A {
        match self.bits {
            false => UIF_A::Clear,
            true => UIF_A::UpdatePending,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UIF_A::Clear
    }
    ///Checks if the value of the field is `UpdatePending`
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UIF_A::UpdatePending
    }
}
///Field `UIF` writer - Update interrupt flag
pub type UIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, UIF_A, O>;
impl<'a, const O: u8> UIF_W<'a, O> {
    ///No update occurred
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UIF_A::Clear)
    }
    ///Update interrupt pending.
    #[inline(always)]
    pub fn update_pending(self) -> &'a mut W {
        self.variant(UIF_A::UpdatePending)
    }
}
///Field `CC1IF` reader - Capture/compare 1 interrupt flag
pub type CC1IF_R = crate::BitReader<CC1IFR_A>;
///Capture/compare 1 interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFR_A {
    ///1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register.
    Match = 1,
}
impl From<CC1IFR_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1IF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CC1IFR_A> {
        match self.bits {
            true => Some(CC1IFR_A::Match),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Match`
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CC1IFR_A::Match
    }
}
///Capture/compare 1 interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<CC1IFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1IFW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1IF` writer - Capture/compare 1 interrupt flag
pub type CC1IF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, CC1IFW_AW, O>;
impl<'a, const O: u8> CC1IF_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC1IFW_AW::Clear)
    }
}
///Field `CC2IF` reader - Capture/Compare 2 interrupt flag
pub use CC1IF_R as CC2IF_R;
///Field `CC3IF` reader - Capture/Compare 3 interrupt flag
pub use CC1IF_R as CC3IF_R;
///Field `CC4IF` reader - Capture/Compare 4 interrupt flag
pub use CC1IF_R as CC4IF_R;
///Field `CC2IF` writer - Capture/Compare 2 interrupt flag
pub use CC1IF_W as CC2IF_W;
///Field `CC3IF` writer - Capture/Compare 3 interrupt flag
pub use CC1IF_W as CC3IF_W;
///Field `CC4IF` writer - Capture/Compare 4 interrupt flag
pub use CC1IF_W as CC4IF_W;
///Field `COMIF` reader - COM interrupt flag
pub type COMIF_R = crate::BitReader<bool>;
///Field `COMIF` writer - COM interrupt flag
pub type COMIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `TIF` reader - Trigger interrupt flag
pub type TIF_R = crate::BitReader<TIFR_A>;
///Trigger interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFR_A {
    ///0: No trigger event occurred
    NoTrigger = 0,
    ///1: Trigger interrupt pending
    Trigger = 1,
}
impl From<TIFR_A> for bool {
    #[inline(always)]
    fn from(variant: TIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl TIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIFR_A {
        match self.bits {
            false => TIFR_A::NoTrigger,
            true => TIFR_A::Trigger,
        }
    }
    ///Checks if the value of the field is `NoTrigger`
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == TIFR_A::NoTrigger
    }
    ///Checks if the value of the field is `Trigger`
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TIFR_A::Trigger
    }
}
///Trigger interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<TIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: TIFW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TIF` writer - Trigger interrupt flag
pub type TIF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, TIFW_AW, O>;
impl<'a, const O: u8> TIF_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIFW_AW::Clear)
    }
}
///Field `BIF` reader - Break interrupt flag
pub type BIF_R = crate::BitReader<bool>;
///Field `BIF` writer - Break interrupt flag
pub type BIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `B2IF` reader - Break 2 interrupt flag
pub type B2IF_R = crate::BitReader<bool>;
///Field `B2IF` writer - Break 2 interrupt flag
pub type B2IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `CC1OF` reader - Capture/Compare 1 overcapture flag
pub type CC1OF_R = crate::BitReader<CC1OFR_A>;
///Capture/Compare 1 overcapture flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFR_A {
    ///1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set
    Overcapture = 1,
}
impl From<CC1OFR_A> for bool {
    #[inline(always)]
    fn from(variant: CC1OFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1OF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CC1OFR_A> {
        match self.bits {
            true => Some(CC1OFR_A::Overcapture),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Overcapture`
    #[inline(always)]
    pub fn is_overcapture(&self) -> bool {
        *self == CC1OFR_A::Overcapture
    }
}
///Capture/Compare 1 overcapture flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<CC1OFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1OFW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1OF` writer - Capture/Compare 1 overcapture flag
pub type CC1OF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, CC1OFW_AW, O>;
impl<'a, const O: u8> CC1OF_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC1OFW_AW::Clear)
    }
}
///Field `CC2OF` reader - Capture/compare 2 overcapture flag
pub use CC1OF_R as CC2OF_R;
///Field `CC3OF` reader - Capture/Compare 3 overcapture flag
pub use CC1OF_R as CC3OF_R;
///Field `CC4OF` reader - Capture/Compare 4 overcapture flag
pub use CC1OF_R as CC4OF_R;
///Field `CC2OF` writer - Capture/compare 2 overcapture flag
pub use CC1OF_W as CC2OF_W;
///Field `CC3OF` writer - Capture/Compare 3 overcapture flag
pub use CC1OF_W as CC3OF_W;
///Field `CC4OF` writer - Capture/Compare 4 overcapture flag
pub use CC1OF_W as CC4OF_W;
///Field `SBIF` reader - System Break interrupt flag
pub type SBIF_R = crate::BitReader<bool>;
///Field `SBIF` writer - System Break interrupt flag
pub type SBIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `CC5IF` reader - Compare 5 interrupt flag
pub use CC1IF_R as CC5IF_R;
///Field `CC6IF` reader - Compare 6 interrupt flag
pub use CC1IF_R as CC6IF_R;
///Field `CC5IF` writer - Compare 5 interrupt flag
pub use CC1IF_W as CC5IF_W;
///Field `CC6IF` writer - Compare 6 interrupt flag
pub use CC1IF_W as CC6IF_W;
impl R {
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 3 interrupt flag
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 4 interrupt flag
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt flag
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Break 2 interrupt flag
    #[inline(always)]
    pub fn b2if(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/compare 2 overcapture flag
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - System Break interrupt flag
    #[inline(always)]
    pub fn sbif(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Compare 5 interrupt flag
    #[inline(always)]
    pub fn cc5if(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Compare 6 interrupt flag
    #[inline(always)]
    pub fn cc6if(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<0> {
        UIF_W::new(self)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<1> {
        CC1IF_W::new(self)
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<2> {
        CC2IF_W::new(self)
    }
    ///Bit 3 - Capture/Compare 3 interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cc3if(&mut self) -> CC3IF_W<3> {
        CC3IF_W::new(self)
    }
    ///Bit 4 - Capture/Compare 4 interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cc4if(&mut self) -> CC4IF_W<4> {
        CC4IF_W::new(self)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn comif(&mut self) -> COMIF_W<5> {
        COMIF_W::new(self)
    }
    ///Bit 6 - Trigger interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<6> {
        TIF_W::new(self)
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<7> {
        BIF_W::new(self)
    }
    ///Bit 8 - Break 2 interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn b2if(&mut self) -> B2IF_W<8> {
        B2IF_W::new(self)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<9> {
        CC1OF_W::new(self)
    }
    ///Bit 10 - Capture/compare 2 overcapture flag
    #[inline(always)]
    #[must_use]
    pub fn cc2of(&mut self) -> CC2OF_W<10> {
        CC2OF_W::new(self)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag
    #[inline(always)]
    #[must_use]
    pub fn cc3of(&mut self) -> CC3OF_W<11> {
        CC3OF_W::new(self)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag
    #[inline(always)]
    #[must_use]
    pub fn cc4of(&mut self) -> CC4OF_W<12> {
        CC4OF_W::new(self)
    }
    ///Bit 13 - System Break interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn sbif(&mut self) -> SBIF_W<13> {
        SBIF_W::new(self)
    }
    ///Bit 16 - Compare 5 interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cc5if(&mut self) -> CC5IF_W<16> {
        CC5IF_W::new(self)
    }
    ///Bit 17 - Compare 6 interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cc6if(&mut self) -> CC6IF_W<17> {
        CC6IF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0003_1e5e;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
