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
///Field `CCRCFAILC` reader - CCRCFAIL flag clear bit
pub type CCRCFAILC_R = crate::BitReader<CCRCFAILCW_A>;
///CCRCFAIL flag clear bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRCFAILCW_A {
    ///1: Clear flag
    Clear = 1,
}
impl From<CCRCFAILCW_A> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAILCW_A) -> Self {
        variant as u8 != 0
    }
}
impl CCRCFAILC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CCRCFAILCW_A> {
        match self.bits {
            true => Some(CCRCFAILCW_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCRCFAILCW_A::Clear
    }
}
///Field `CCRCFAILC` writer - CCRCFAIL flag clear bit
pub type CCRCFAILC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CCRCFAILCW_A, O>;
impl<'a, const O: u8> CCRCFAILC_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILCW_A::Clear)
    }
}
///Field `DCRCFAILC` reader - DCRCFAIL flag clear bit
pub use CCRCFAILC_R as DCRCFAILC_R;
///Field `CTIMEOUTC` reader - CTIMEOUT flag clear bit
pub use CCRCFAILC_R as CTIMEOUTC_R;
///Field `DTIMEOUTC` reader - DTIMEOUT flag clear bit
pub use CCRCFAILC_R as DTIMEOUTC_R;
///Field `TXUNDERRC` reader - TXUNDERR flag clear bit
pub use CCRCFAILC_R as TXUNDERRC_R;
///Field `RXOVERRC` reader - RXOVERR flag clear bit
pub use CCRCFAILC_R as RXOVERRC_R;
///Field `CMDRENDC` reader - CMDREND flag clear bit
pub use CCRCFAILC_R as CMDRENDC_R;
///Field `CMDSENTC` reader - CMDSENT flag clear bit
pub use CCRCFAILC_R as CMDSENTC_R;
///Field `DATAENDC` reader - DATAEND flag clear bit
pub use CCRCFAILC_R as DATAENDC_R;
///Field `STBITERRC` reader - STBITERR flag clear bit
pub use CCRCFAILC_R as STBITERRC_R;
///Field `DBCKENDC` reader - DBCKEND flag clear bit
pub use CCRCFAILC_R as DBCKENDC_R;
///Field `SDIOITC` reader - SDIOIT flag clear bit
pub use CCRCFAILC_R as SDIOITC_R;
///Field `CEATAENDC` reader - CEATAEND flag clear bit
pub use CCRCFAILC_R as CEATAENDC_R;
///Field `DCRCFAILC` writer - DCRCFAIL flag clear bit
pub use CCRCFAILC_W as DCRCFAILC_W;
///Field `CTIMEOUTC` writer - CTIMEOUT flag clear bit
pub use CCRCFAILC_W as CTIMEOUTC_W;
///Field `DTIMEOUTC` writer - DTIMEOUT flag clear bit
pub use CCRCFAILC_W as DTIMEOUTC_W;
///Field `TXUNDERRC` writer - TXUNDERR flag clear bit
pub use CCRCFAILC_W as TXUNDERRC_W;
///Field `RXOVERRC` writer - RXOVERR flag clear bit
pub use CCRCFAILC_W as RXOVERRC_W;
///Field `CMDRENDC` writer - CMDREND flag clear bit
pub use CCRCFAILC_W as CMDRENDC_W;
///Field `CMDSENTC` writer - CMDSENT flag clear bit
pub use CCRCFAILC_W as CMDSENTC_W;
///Field `DATAENDC` writer - DATAEND flag clear bit
pub use CCRCFAILC_W as DATAENDC_W;
///Field `STBITERRC` writer - STBITERR flag clear bit
pub use CCRCFAILC_W as STBITERRC_W;
///Field `DBCKENDC` writer - DBCKEND flag clear bit
pub use CCRCFAILC_W as DBCKENDC_W;
///Field `SDIOITC` writer - SDIOIT flag clear bit
pub use CCRCFAILC_W as SDIOITC_W;
///Field `CEATAENDC` writer - CEATAEND flag clear bit
pub use CCRCFAILC_W as CEATAENDC_W;
impl R {
    ///Bit 0 - CCRCFAIL flag clear bit
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CCRCFAILC_R {
        CCRCFAILC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DCRCFAIL flag clear bit
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DCRCFAILC_R {
        DCRCFAILC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTIMEOUT flag clear bit
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CTIMEOUTC_R {
        CTIMEOUTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DTIMEOUT flag clear bit
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DTIMEOUTC_R {
        DTIMEOUTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXUNDERR flag clear bit
    #[inline(always)]
    pub fn txunderrc(&self) -> TXUNDERRC_R {
        TXUNDERRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXOVERR flag clear bit
    #[inline(always)]
    pub fn rxoverrc(&self) -> RXOVERRC_R {
        RXOVERRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CMDREND flag clear bit
    #[inline(always)]
    pub fn cmdrendc(&self) -> CMDRENDC_R {
        CMDRENDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CMDSENT flag clear bit
    #[inline(always)]
    pub fn cmdsentc(&self) -> CMDSENTC_R {
        CMDSENTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DATAEND flag clear bit
    #[inline(always)]
    pub fn dataendc(&self) -> DATAENDC_R {
        DATAENDC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - STBITERR flag clear bit
    #[inline(always)]
    pub fn stbiterrc(&self) -> STBITERRC_R {
        STBITERRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBCKEND flag clear bit
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 22 - SDIOIT flag clear bit
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CEATAEND flag clear bit
    #[inline(always)]
    pub fn ceataendc(&self) -> CEATAENDC_R {
        CEATAENDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CCRCFAIL flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W<0> {
        CCRCFAILC_W::new(self)
    }
    ///Bit 1 - DCRCFAIL flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W<1> {
        DCRCFAILC_W::new(self)
    }
    ///Bit 2 - CTIMEOUT flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W<2> {
        CTIMEOUTC_W::new(self)
    }
    ///Bit 3 - DTIMEOUT flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W<3> {
        DTIMEOUTC_W::new(self)
    }
    ///Bit 4 - TXUNDERR flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W<4> {
        TXUNDERRC_W::new(self)
    }
    ///Bit 5 - RXOVERR flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W<5> {
        RXOVERRC_W::new(self)
    }
    ///Bit 6 - CMDREND flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W<6> {
        CMDRENDC_W::new(self)
    }
    ///Bit 7 - CMDSENT flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W<7> {
        CMDSENTC_W::new(self)
    }
    ///Bit 8 - DATAEND flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn dataendc(&mut self) -> DATAENDC_W<8> {
        DATAENDC_W::new(self)
    }
    ///Bit 9 - STBITERR flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn stbiterrc(&mut self) -> STBITERRC_W<9> {
        STBITERRC_W::new(self)
    }
    ///Bit 10 - DBCKEND flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn dbckendc(&mut self) -> DBCKENDC_W<10> {
        DBCKENDC_W::new(self)
    }
    ///Bit 22 - SDIOIT flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn sdioitc(&mut self) -> SDIOITC_W<22> {
        SDIOITC_W::new(self)
    }
    ///Bit 23 - CEATAEND flag clear bit
    #[inline(always)]
    #[must_use]
    pub fn ceataendc(&mut self) -> CEATAENDC_W<23> {
        CEATAENDC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt clear register
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
