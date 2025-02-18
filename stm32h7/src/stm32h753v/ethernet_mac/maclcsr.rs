///Register `MACLCSR` reader
pub struct R(crate::R<MACLCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACLCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACLCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACLCSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACLCSR` writer
pub struct W(crate::W<MACLCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACLCSR_SPEC>;
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
impl From<crate::W<MACLCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACLCSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TLPIEN` reader - Transmit LPI Entry
pub type TLPIEN_R = crate::BitReader<bool>;
///Field `TLPIEX` reader - Transmit LPI Exit
pub type TLPIEX_R = crate::BitReader<bool>;
///Field `RLPIEN` reader - Receive LPI Entry
pub type RLPIEN_R = crate::BitReader<bool>;
///Field `RLPIEX` reader - Receive LPI Exit
pub type RLPIEX_R = crate::BitReader<bool>;
///Field `TLPIST` reader - Transmit LPI State
pub type TLPIST_R = crate::BitReader<bool>;
///Field `RLPIST` reader - Receive LPI State
pub type RLPIST_R = crate::BitReader<bool>;
///Field `LPIEN` reader - LPI Enable
pub type LPIEN_R = crate::BitReader<bool>;
///Field `LPIEN` writer - LPI Enable
pub type LPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACLCSR_SPEC, bool, O>;
///Field `PLS` reader - PHY Link Status
pub type PLS_R = crate::BitReader<bool>;
///Field `PLS` writer - PHY Link Status
pub type PLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACLCSR_SPEC, bool, O>;
///Field `PLSEN` reader - PHY Link Status Enable
pub type PLSEN_R = crate::BitReader<bool>;
///Field `PLSEN` writer - PHY Link Status Enable
pub type PLSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACLCSR_SPEC, bool, O>;
///Field `LPITXA` reader - LPI Tx Automate
pub type LPITXA_R = crate::BitReader<bool>;
///Field `LPITXA` writer - LPI Tx Automate
pub type LPITXA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACLCSR_SPEC, bool, O>;
///Field `LPITE` reader - LPI Timer Enable
pub type LPITE_R = crate::BitReader<bool>;
///Field `LPITE` writer - LPI Timer Enable
pub type LPITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACLCSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Transmit LPI Entry
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit LPI Exit
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive LPI Entry
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Receive LPI Exit
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Transmit LPI State
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Receive LPI State
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - LPI Enable
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PHY Link Status
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PHY Link Status Enable
    #[inline(always)]
    pub fn plsen(&self) -> PLSEN_R {
        PLSEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - LPI Tx Automate
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - LPI Timer Enable
    #[inline(always)]
    pub fn lpite(&self) -> LPITE_R {
        LPITE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 16 - LPI Enable
    #[inline(always)]
    #[must_use]
    pub fn lpien(&mut self) -> LPIEN_W<16> {
        LPIEN_W::new(self)
    }
    ///Bit 17 - PHY Link Status
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<17> {
        PLS_W::new(self)
    }
    ///Bit 18 - PHY Link Status Enable
    #[inline(always)]
    #[must_use]
    pub fn plsen(&mut self) -> PLSEN_W<18> {
        PLSEN_W::new(self)
    }
    ///Bit 19 - LPI Tx Automate
    #[inline(always)]
    #[must_use]
    pub fn lpitxa(&mut self) -> LPITXA_W<19> {
        LPITXA_W::new(self)
    }
    ///Bit 20 - LPI Timer Enable
    #[inline(always)]
    #[must_use]
    pub fn lpite(&mut self) -> LPITE_W<20> {
        LPITE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPI control status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maclcsr](index.html) module
pub struct MACLCSR_SPEC;
impl crate::RegisterSpec for MACLCSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [maclcsr::R](R) reader structure
impl crate::Readable for MACLCSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [maclcsr::W](W) writer structure
impl crate::Writable for MACLCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACLCSR to value 0
impl crate::Resettable for MACLCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
