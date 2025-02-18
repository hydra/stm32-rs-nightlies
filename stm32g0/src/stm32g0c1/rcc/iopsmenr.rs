///Register `IOPSMENR` reader
pub struct R(crate::R<IOPSMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPSMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPSMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPSMENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IOPSMENR` writer
pub struct W(crate::W<IOPSMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPSMENR_SPEC>;
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
impl From<crate::W<IOPSMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPSMENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOASMEN` reader - I/O port A clock enable during Sleep mode
pub type GPIOASMEN_R = crate::BitReader<bool>;
///Field `GPIOASMEN` writer - I/O port A clock enable during Sleep mode
pub type GPIOASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
///Field `GPIOBSMEN` reader - I/O port B clock enable during Sleep mode
pub type GPIOBSMEN_R = crate::BitReader<bool>;
///Field `GPIOBSMEN` writer - I/O port B clock enable during Sleep mode
pub type GPIOBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
///Field `GPIOCSMEN` reader - I/O port C clock enable during Sleep mode
pub type GPIOCSMEN_R = crate::BitReader<bool>;
///Field `GPIOCSMEN` writer - I/O port C clock enable during Sleep mode
pub type GPIOCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
///Field `GPIODSMEN` reader - I/O port D clock enable during Sleep mode
pub type GPIODSMEN_R = crate::BitReader<bool>;
///Field `GPIODSMEN` writer - I/O port D clock enable during Sleep mode
pub type GPIODSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
///Field `GPIOESMEN` reader - I/O port E clock enable during Sleep mode
pub type GPIOESMEN_R = crate::BitReader<bool>;
///Field `GPIOESMEN` writer - I/O port E clock enable during Sleep mode
pub type GPIOESMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
///Field `GPIOFSMEN` reader - I/O port F clock enable during Sleep mode
pub type GPIOFSMEN_R = crate::BitReader<bool>;
///Field `GPIOFSMEN` writer - I/O port F clock enable during Sleep mode
pub type GPIOFSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
impl R {
    ///Bit 0 - I/O port A clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port E clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - I/O port A clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<0> {
        GPIOASMEN_W::new(self)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<1> {
        GPIOBSMEN_W::new(self)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<2> {
        GPIOCSMEN_W::new(self)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<3> {
        GPIODSMEN_W::new(self)
    }
    ///Bit 4 - I/O port E clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<4> {
        GPIOESMEN_W::new(self)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<5> {
        GPIOFSMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO in Sleep mode clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iopsmenr](index.html) module
pub struct IOPSMENR_SPEC;
impl crate::RegisterSpec for IOPSMENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [iopsmenr::R](R) reader structure
impl crate::Readable for IOPSMENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [iopsmenr::W](W) writer structure
impl crate::Writable for IOPSMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IOPSMENR to value 0x3f
impl crate::Resettable for IOPSMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
