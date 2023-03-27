///Register `SRCR` reader
pub struct R(crate::R<SRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SRCR` writer
pub struct W(crate::W<SRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCR_SPEC>;
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
impl From<crate::W<SRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IMR` reader - Immediate Reload
pub type IMR_R = crate::BitReader<IMR_A>;
///Immediate Reload
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMR_A {
    ///0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
    NoEffect = 0,
    ///1: The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload
    Reload = 1,
}
impl From<IMR_A> for bool {
    #[inline(always)]
    fn from(variant: IMR_A) -> Self {
        variant as u8 != 0
    }
}
impl IMR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IMR_A {
        match self.bits {
            false => IMR_A::NoEffect,
            true => IMR_A::Reload,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == IMR_A::NoEffect
    }
    ///Checks if the value of the field is `Reload`
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == IMR_A::Reload
    }
}
///Field `IMR` writer - Immediate Reload
pub type IMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCR_SPEC, IMR_A, O>;
impl<'a, const O: u8> IMR_W<'a, O> {
    ///This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(IMR_A::NoEffect)
    }
    ///The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(IMR_A::Reload)
    }
}
///Field `VBR` reader - Vertical Blanking Reload
pub type VBR_R = crate::BitReader<VBR_A>;
///Vertical Blanking Reload
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBR_A {
    ///0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
    NoEffect = 0,
    ///1: The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area).
    Reload = 1,
}
impl From<VBR_A> for bool {
    #[inline(always)]
    fn from(variant: VBR_A) -> Self {
        variant as u8 != 0
    }
}
impl VBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VBR_A {
        match self.bits {
            false => VBR_A::NoEffect,
            true => VBR_A::Reload,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == VBR_A::NoEffect
    }
    ///Checks if the value of the field is `Reload`
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == VBR_A::Reload
    }
}
///Field `VBR` writer - Vertical Blanking Reload
pub type VBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCR_SPEC, VBR_A, O>;
impl<'a, const O: u8> VBR_W<'a, O> {
    ///This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(VBR_A::NoEffect)
    }
    ///The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area).
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(VBR_A::Reload)
    }
}
impl R {
    ///Bit 0 - Immediate Reload
    #[inline(always)]
    pub fn imr(&self) -> IMR_R {
        IMR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Vertical Blanking Reload
    #[inline(always)]
    pub fn vbr(&self) -> VBR_R {
        VBR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Immediate Reload
    #[inline(always)]
    #[must_use]
    pub fn imr(&mut self) -> IMR_W<0> {
        IMR_W::new(self)
    }
    ///Bit 1 - Vertical Blanking Reload
    #[inline(always)]
    #[must_use]
    pub fn vbr(&mut self) -> VBR_W<1> {
        VBR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Shadow Reload Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [srcr](index.html) module
pub struct SRCR_SPEC;
impl crate::RegisterSpec for SRCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [srcr::R](R) reader structure
impl crate::Readable for SRCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [srcr::W](W) writer structure
impl crate::Writable for SRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SRCR to value 0
impl crate::Resettable for SRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
