///Register `AMTCR` reader
pub struct R(crate::R<AMTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMTCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AMTCR` writer
pub struct W(crate::W<AMTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMTCR_SPEC>;
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
impl From<crate::W<AMTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMTCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - Enable
pub type EN_R = crate::BitReader<EN_A>;
///Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    ///0: Disabled AHB/AXI dead-time functionality
    Disabled = 0,
    ///1: Enabled AHB/AXI dead-time functionality
    Enabled = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::Disabled,
            true => EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::Enabled
    }
}
///Field `EN` writer - Enable
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AMTCR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    ///Disabled AHB/AXI dead-time functionality
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::Disabled)
    }
    ///Enabled AHB/AXI dead-time functionality
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::Enabled)
    }
}
///Field `DT` reader - Dead Time
pub type DT_R = crate::FieldReader<u8, u8>;
///Field `DT` writer - Dead Time
pub type DT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AMTCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:15 - Dead Time
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bits 8:15 - Dead Time
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<8> {
        DT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB master timer configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [amtcr](index.html) module
pub struct AMTCR_SPEC;
impl crate::RegisterSpec for AMTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [amtcr::R](R) reader structure
impl crate::Readable for AMTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [amtcr::W](W) writer structure
impl crate::Writable for AMTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AMTCR to value 0
impl crate::Resettable for AMTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
