///Register `MACIMR` reader
pub struct R(crate::R<MACIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACIMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACIMR` writer
pub struct W(crate::W<MACIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACIMR_SPEC>;
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
impl From<crate::W<MACIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACIMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PMTIM` reader - PMT interrupt mask
pub type PMTIM_R = crate::BitReader<PMTIM_A>;
///PMT interrupt mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMTIM_A {
    ///0: PMT Status interrupt generation enabled
    Unmasked = 0,
    ///1: PMT Status interrupt generation disabled
    Masked = 1,
}
impl From<PMTIM_A> for bool {
    #[inline(always)]
    fn from(variant: PMTIM_A) -> Self {
        variant as u8 != 0
    }
}
impl PMTIM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PMTIM_A {
        match self.bits {
            false => PMTIM_A::Unmasked,
            true => PMTIM_A::Masked,
        }
    }
    ///Checks if the value of the field is `Unmasked`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == PMTIM_A::Unmasked
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMTIM_A::Masked
    }
}
///Field `PMTIM` writer - PMT interrupt mask
pub type PMTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIMR_SPEC, PMTIM_A, O>;
impl<'a, const O: u8> PMTIM_W<'a, O> {
    ///PMT Status interrupt generation enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(PMTIM_A::Unmasked)
    }
    ///PMT Status interrupt generation disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMTIM_A::Masked)
    }
}
///Field `TSTIM` reader - Time stamp trigger interrupt mask
pub type TSTIM_R = crate::BitReader<TSTIM_A>;
///Time stamp trigger interrupt mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTIM_A {
    ///0: Time stamp interrupt generation enabled
    Unmasked = 0,
    ///1: Time stamp interrupt generation disabled
    Masked = 1,
}
impl From<TSTIM_A> for bool {
    #[inline(always)]
    fn from(variant: TSTIM_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTIM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSTIM_A {
        match self.bits {
            false => TSTIM_A::Unmasked,
            true => TSTIM_A::Masked,
        }
    }
    ///Checks if the value of the field is `Unmasked`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TSTIM_A::Unmasked
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TSTIM_A::Masked
    }
}
///Field `TSTIM` writer - Time stamp trigger interrupt mask
pub type TSTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIMR_SPEC, TSTIM_A, O>;
impl<'a, const O: u8> TSTIM_W<'a, O> {
    ///Time stamp interrupt generation enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TSTIM_A::Unmasked)
    }
    ///Time stamp interrupt generation disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TSTIM_A::Masked)
    }
}
impl R {
    ///Bit 3 - PMT interrupt mask
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - Time stamp trigger interrupt mask
    #[inline(always)]
    pub fn tstim(&self) -> TSTIM_R {
        TSTIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - PMT interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn pmtim(&mut self) -> PMTIM_W<3> {
        PMTIM_W::new(self)
    }
    ///Bit 9 - Time stamp trigger interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn tstim(&mut self) -> TSTIM_W<9> {
        TSTIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC interrupt mask register (ETH_MACIMR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macimr](index.html) module
pub struct MACIMR_SPEC;
impl crate::RegisterSpec for MACIMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macimr::R](R) reader structure
impl crate::Readable for MACIMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macimr::W](W) writer structure
impl crate::Writable for MACIMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACIMR to value 0
impl crate::Resettable for MACIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
