///Register `TZSC_PRIVCFGR2` reader
pub struct R(crate::R<TZSC_PRIVCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_PRIVCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_PRIVCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_PRIVCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_PRIVCFGR2` writer
pub struct W(crate::W<TZSC_PRIVCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_PRIVCFGR2_SPEC>;
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
impl From<crate::W<TZSC_PRIVCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_PRIVCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM1PRIV` reader - privileged access mode for TIM1
pub type TIM1PRIV_R = crate::BitReader<bool>;
///Field `TIM1PRIV` writer - privileged access mode for TIM1
pub type TIM1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR2_SPEC, bool, O>;
///Field `SPI1PRIV` reader - privileged access mode for SPI1PRIV
pub type SPI1PRIV_R = crate::BitReader<bool>;
///Field `SPI1PRIV` writer - privileged access mode for SPI1PRIV
pub type SPI1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR2_SPEC, bool, O>;
///Field `TIM8PRIV` reader - privileged access mode for TIM8
pub type TIM8PRIV_R = crate::BitReader<bool>;
///Field `TIM8PRIV` writer - privileged access mode for TIM8
pub type TIM8PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR2_SPEC, bool, O>;
///Field `USART1PRIV` reader - privileged access mode for USART1
pub type USART1PRIV_R = crate::BitReader<bool>;
///Field `USART1PRIV` writer - privileged access mode for USART1
pub type USART1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR2_SPEC, bool, O>;
///Field `TIM15PRIV` reader - privileged access mode for TIM15
pub type TIM15PRIV_R = crate::BitReader<bool>;
///Field `TIM15PRIV` writer - privileged access mode for TIM15
pub type TIM15PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR2_SPEC, bool, O>;
///Field `TIM16PRIV` reader - privileged access mode for TIM16
pub type TIM16PRIV_R = crate::BitReader<bool>;
///Field `TIM16PRIV` writer - privileged access mode for TIM16
pub type TIM16PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR2_SPEC, bool, O>;
///Field `TIM17PRIV` reader - privileged access mode for TIM17
pub type TIM17PRIV_R = crate::BitReader<bool>;
///Field `TIM17PRIV` writer - privileged access mode for TIM17
pub type TIM17PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR2_SPEC, bool, O>;
///Field `SAI1PRIV` reader - privileged access mode for SAI1
pub type SAI1PRIV_R = crate::BitReader<bool>;
///Field `SAI1PRIV` writer - privileged access mode for SAI1
pub type SAI1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR2_SPEC, bool, O>;
///Field `SAI2PRIV` reader - privileged access mode for SAI2
pub type SAI2PRIV_R = crate::BitReader<bool>;
///Field `SAI2PRIV` writer - privileged access mode for SAI2
pub type SAI2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - privileged access mode for TIM1
    #[inline(always)]
    pub fn tim1priv(&self) -> TIM1PRIV_R {
        TIM1PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged access mode for SPI1PRIV
    #[inline(always)]
    pub fn spi1priv(&self) -> SPI1PRIV_R {
        SPI1PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - privileged access mode for TIM8
    #[inline(always)]
    pub fn tim8priv(&self) -> TIM8PRIV_R {
        TIM8PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - privileged access mode for USART1
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged access mode for TIM15
    #[inline(always)]
    pub fn tim15priv(&self) -> TIM15PRIV_R {
        TIM15PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - privileged access mode for TIM16
    #[inline(always)]
    pub fn tim16priv(&self) -> TIM16PRIV_R {
        TIM16PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - privileged access mode for TIM17
    #[inline(always)]
    pub fn tim17priv(&self) -> TIM17PRIV_R {
        TIM17PRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - privileged access mode for SAI1
    #[inline(always)]
    pub fn sai1priv(&self) -> SAI1PRIV_R {
        SAI1PRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - privileged access mode for SAI2
    #[inline(always)]
    pub fn sai2priv(&self) -> SAI2PRIV_R {
        SAI2PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - privileged access mode for TIM1
    #[inline(always)]
    #[must_use]
    pub fn tim1priv(&mut self) -> TIM1PRIV_W<0> {
        TIM1PRIV_W::new(self)
    }
    ///Bit 1 - privileged access mode for SPI1PRIV
    #[inline(always)]
    #[must_use]
    pub fn spi1priv(&mut self) -> SPI1PRIV_W<1> {
        SPI1PRIV_W::new(self)
    }
    ///Bit 2 - privileged access mode for TIM8
    #[inline(always)]
    #[must_use]
    pub fn tim8priv(&mut self) -> TIM8PRIV_W<2> {
        TIM8PRIV_W::new(self)
    }
    ///Bit 3 - privileged access mode for USART1
    #[inline(always)]
    #[must_use]
    pub fn usart1priv(&mut self) -> USART1PRIV_W<3> {
        USART1PRIV_W::new(self)
    }
    ///Bit 4 - privileged access mode for TIM15
    #[inline(always)]
    #[must_use]
    pub fn tim15priv(&mut self) -> TIM15PRIV_W<4> {
        TIM15PRIV_W::new(self)
    }
    ///Bit 5 - privileged access mode for TIM16
    #[inline(always)]
    #[must_use]
    pub fn tim16priv(&mut self) -> TIM16PRIV_W<5> {
        TIM16PRIV_W::new(self)
    }
    ///Bit 6 - privileged access mode for TIM17
    #[inline(always)]
    #[must_use]
    pub fn tim17priv(&mut self) -> TIM17PRIV_W<6> {
        TIM17PRIV_W::new(self)
    }
    ///Bit 7 - privileged access mode for SAI1
    #[inline(always)]
    #[must_use]
    pub fn sai1priv(&mut self) -> SAI1PRIV_W<7> {
        SAI1PRIV_W::new(self)
    }
    ///Bit 8 - privileged access mode for SAI2
    #[inline(always)]
    #[must_use]
    pub fn sai2priv(&mut self) -> SAI2PRIV_W<8> {
        SAI2PRIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC privilege configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_privcfgr2](index.html) module
pub struct TZSC_PRIVCFGR2_SPEC;
impl crate::RegisterSpec for TZSC_PRIVCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_privcfgr2::R](R) reader structure
impl crate::Readable for TZSC_PRIVCFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_privcfgr2::W](W) writer structure
impl crate::Writable for TZSC_PRIVCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZSC_PRIVCFGR2 to value 0
impl crate::Resettable for TZSC_PRIVCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
