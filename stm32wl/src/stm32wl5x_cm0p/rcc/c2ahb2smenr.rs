///Register `C2AHB2SMENR` reader
pub struct R(crate::R<C2AHB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2AHB2SMENR` writer
pub struct W(crate::W<C2AHB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB2SMENR_SPEC>;
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
impl From<crate::W<C2AHB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOASMEN` reader - IO port A clock enable during CPU2 CSleep mode.
pub type GPIOASMEN_R = crate::BitReader<bool>;
///Field `GPIOASMEN` writer - IO port A clock enable during CPU2 CSleep mode.
pub type GPIOASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
///Field `GPIOBSMEN` reader - IO port B clock enable during CPU2 CSleep mode.
pub type GPIOBSMEN_R = crate::BitReader<bool>;
///Field `GPIOBSMEN` writer - IO port B clock enable during CPU2 CSleep mode.
pub type GPIOBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
///Field `GPIOCSMEN` reader - IO port C clock enable during CPU2 CSleep mode.
pub type GPIOCSMEN_R = crate::BitReader<bool>;
///Field `GPIOCSMEN` writer - IO port C clock enable during CPU2 CSleep mode.
pub type GPIOCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
///Field `GPIOHSMEN` reader - IO port H clock enable during CPU2 CSleep mode.
pub type GPIOHSMEN_R = crate::BitReader<bool>;
///Field `GPIOHSMEN` writer - IO port H clock enable during CPU2 CSleep mode.
pub type GPIOHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
impl R {
    ///Bit 0 - IO port A clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - IO port H clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A clock enable during CPU2 CSleep mode.
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<0> {
        GPIOASMEN_W::new(self)
    }
    ///Bit 1 - IO port B clock enable during CPU2 CSleep mode.
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<1> {
        GPIOBSMEN_W::new(self)
    }
    ///Bit 2 - IO port C clock enable during CPU2 CSleep mode.
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<2> {
        GPIOCSMEN_W::new(self)
    }
    ///Bit 7 - IO port H clock enable during CPU2 CSleep mode.
    #[inline(always)]
    #[must_use]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<7> {
        GPIOHSMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPU2 AHB2 peripheral clocks enable in Sleep modes register \[dual core device only\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2ahb2smenr](index.html) module
pub struct C2AHB2SMENR_SPEC;
impl crate::RegisterSpec for C2AHB2SMENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2ahb2smenr::R](R) reader structure
impl crate::Readable for C2AHB2SMENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2ahb2smenr::W](W) writer structure
impl crate::Writable for C2AHB2SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2AHB2SMENR to value 0x87
impl crate::Resettable for C2AHB2SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x87;
}
