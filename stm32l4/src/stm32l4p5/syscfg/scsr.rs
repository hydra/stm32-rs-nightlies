///Register `SCSR` reader
pub struct R(crate::R<SCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SCSR` writer
pub struct W(crate::W<SCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCSR_SPEC>;
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
impl From<crate::W<SCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SRAM2ER` reader - SRAM2 Erase
pub type SRAM2ER_R = crate::BitReader<SRAM2ER_A>;
///SRAM2 Erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2ER_A {
    ///1: Setting this bit starts a hardware SRAM2 erase operation
    Erase = 1,
}
impl From<SRAM2ER_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2ER_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM2ER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SRAM2ER_A> {
        match self.bits {
            true => Some(SRAM2ER_A::Erase),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Erase`
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == SRAM2ER_A::Erase
    }
}
///Field `SRAM2ER` writer - SRAM2 Erase
pub type SRAM2ER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCSR_SPEC, SRAM2ER_A, O>;
impl<'a, const O: u8> SRAM2ER_W<'a, O> {
    ///Setting this bit starts a hardware SRAM2 erase operation
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(SRAM2ER_A::Erase)
    }
}
///Field `SRAM2BS` reader - SRAM2 busy by erase operation
pub type SRAM2BS_R = crate::BitReader<SRAM2BS_A>;
///SRAM2 busy by erase operation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2BS_A {
    ///0: No SRAM2 erase operation is on going
    NotBusy = 0,
    ///1: SRAM2 erase operation is on going
    Busy = 1,
}
impl From<SRAM2BS_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2BS_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM2BS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SRAM2BS_A {
        match self.bits {
            false => SRAM2BS_A::NotBusy,
            true => SRAM2BS_A::Busy,
        }
    }
    ///Checks if the value of the field is `NotBusy`
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == SRAM2BS_A::NotBusy
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SRAM2BS_A::Busy
    }
}
impl R {
    ///Bit 0 - SRAM2 Erase
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM2 busy by erase operation
    #[inline(always)]
    pub fn sram2bs(&self) -> SRAM2BS_R {
        SRAM2BS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SRAM2 Erase
    #[inline(always)]
    #[must_use]
    pub fn sram2er(&mut self) -> SRAM2ER_W<0> {
        SRAM2ER_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SCSR
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [scsr](index.html) module
pub struct SCSR_SPEC;
impl crate::RegisterSpec for SCSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [scsr::R](R) reader structure
impl crate::Readable for SCSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [scsr::W](W) writer structure
impl crate::Writable for SCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SCSR to value 0
impl crate::Resettable for SCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
