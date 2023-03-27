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
///Address matched flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRCF_AW {
    ///1: Clears the ADDR flag in ISR register
    Clear = 1,
}
impl From<ADDRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDRCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDRCF` writer - Address matched flag clear
pub type ADDRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ADDRCF_AW, O>;
impl<'a, const O: u8> ADDRCF_W<'a, O> {
    ///Clears the ADDR flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADDRCF_AW::Clear)
    }
}
///Not Acknowledge flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKCF_AW {
    ///1: Clears the NACK flag in ISR register
    Clear = 1,
}
impl From<NACKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: NACKCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `NACKCF` writer - Not Acknowledge flag clear
pub type NACKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, NACKCF_AW, O>;
impl<'a, const O: u8> NACKCF_W<'a, O> {
    ///Clears the NACK flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NACKCF_AW::Clear)
    }
}
///Stop detection flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPCF_AW {
    ///1: Clears the STOP flag in ISR register
    Clear = 1,
}
impl From<STOPCF_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPCF` writer - Stop detection flag clear
pub type STOPCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, STOPCF_AW, O>;
impl<'a, const O: u8> STOPCF_W<'a, O> {
    ///Clears the STOP flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STOPCF_AW::Clear)
    }
}
///Bus error flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERRCF_AW {
    ///1: Clears the BERR flag in ISR register
    Clear = 1,
}
impl From<BERRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: BERRCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `BERRCF` writer - Bus error flag clear
pub type BERRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, BERRCF_AW, O>;
impl<'a, const O: u8> BERRCF_W<'a, O> {
    ///Clears the BERR flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BERRCF_AW::Clear)
    }
}
///Arbitration Lost flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLOCF_AW {
    ///1: Clears the ARLO flag in ISR register
    Clear = 1,
}
impl From<ARLOCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ARLOCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ARLOCF` writer - Arbitration Lost flag clear
pub type ARLOCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ARLOCF_AW, O>;
impl<'a, const O: u8> ARLOCF_W<'a, O> {
    ///Clears the ARLO flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARLOCF_AW::Clear)
    }
}
///Overrun/Underrun flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRCF_AW {
    ///1: Clears the OVR flag in ISR register
    Clear = 1,
}
impl From<OVRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRCF` writer - Overrun/Underrun flag clear
pub type OVRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, OVRCF_AW, O>;
impl<'a, const O: u8> OVRCF_W<'a, O> {
    ///Clears the OVR flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVRCF_AW::Clear)
    }
}
///PEC Error flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECCF_AW {
    ///1: Clears the PEC flag in ISR register
    Clear = 1,
}
impl From<PECCF_AW> for bool {
    #[inline(always)]
    fn from(variant: PECCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PECCF` writer - PEC Error flag clear
pub type PECCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, PECCF_AW, O>;
impl<'a, const O: u8> PECCF_W<'a, O> {
    ///Clears the PEC flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECCF_AW::Clear)
    }
}
///Timeout detection flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUTCF_AW {
    ///1: Clears the TIMOUT flag in ISR register
    Clear = 1,
}
impl From<TIMOUTCF_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMOUTCF` writer - Timeout detection flag clear
pub type TIMOUTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, TIMOUTCF_AW, O>;
impl<'a, const O: u8> TIMOUTCF_W<'a, O> {
    ///Clears the TIMOUT flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIMOUTCF_AW::Clear)
    }
}
///Alert flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTCF_AW {
    ///1: Clears the ALERT flag in ISR register
    Clear = 1,
}
impl From<ALERTCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ALERTCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ALERTCF` writer - Alert flag clear
pub type ALERTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ALERTCF_AW, O>;
impl<'a, const O: u8> ALERTCF_W<'a, O> {
    ///Clears the ALERT flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALERTCF_AW::Clear)
    }
}
impl W {
    ///Bit 3 - Address matched flag clear
    #[inline(always)]
    #[must_use]
    pub fn addrcf(&mut self) -> ADDRCF_W<3> {
        ADDRCF_W::new(self)
    }
    ///Bit 4 - Not Acknowledge flag clear
    #[inline(always)]
    #[must_use]
    pub fn nackcf(&mut self) -> NACKCF_W<4> {
        NACKCF_W::new(self)
    }
    ///Bit 5 - Stop detection flag clear
    #[inline(always)]
    #[must_use]
    pub fn stopcf(&mut self) -> STOPCF_W<5> {
        STOPCF_W::new(self)
    }
    ///Bit 8 - Bus error flag clear
    #[inline(always)]
    #[must_use]
    pub fn berrcf(&mut self) -> BERRCF_W<8> {
        BERRCF_W::new(self)
    }
    ///Bit 9 - Arbitration Lost flag clear
    #[inline(always)]
    #[must_use]
    pub fn arlocf(&mut self) -> ARLOCF_W<9> {
        ARLOCF_W::new(self)
    }
    ///Bit 10 - Overrun/Underrun flag clear
    #[inline(always)]
    #[must_use]
    pub fn ovrcf(&mut self) -> OVRCF_W<10> {
        OVRCF_W::new(self)
    }
    ///Bit 11 - PEC Error flag clear
    #[inline(always)]
    #[must_use]
    pub fn peccf(&mut self) -> PECCF_W<11> {
        PECCF_W::new(self)
    }
    ///Bit 12 - Timeout detection flag clear
    #[inline(always)]
    #[must_use]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W<12> {
        TIMOUTCF_W::new(self)
    }
    ///Bit 13 - Alert flag clear
    #[inline(always)]
    #[must_use]
    pub fn alertcf(&mut self) -> ALERTCF_W<13> {
        ALERTCF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](index.html) module
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
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
