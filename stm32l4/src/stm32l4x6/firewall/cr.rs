///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FPA` reader - Firewall pre alarm
pub type FPA_R = crate::BitReader<FPAW_A>;
///Firewall pre alarm
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPAW_A {
    ///0: Any code executed outside the protected segment when the Firewall is opened will generate a system reset
    PreArmReset = 0,
    ///1: Any code executed outside the protected segment will close the Firewall
    PreArmSet = 1,
}
impl From<FPAW_A> for bool {
    #[inline(always)]
    fn from(variant: FPAW_A) -> Self {
        variant as u8 != 0
    }
}
impl FPA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPAW_A {
        match self.bits {
            false => FPAW_A::PreArmReset,
            true => FPAW_A::PreArmSet,
        }
    }
    ///Checks if the value of the field is `PreArmReset`
    #[inline(always)]
    pub fn is_pre_arm_reset(&self) -> bool {
        *self == FPAW_A::PreArmReset
    }
    ///Checks if the value of the field is `PreArmSet`
    #[inline(always)]
    pub fn is_pre_arm_set(&self) -> bool {
        *self == FPAW_A::PreArmSet
    }
}
///Field `FPA` writer - Firewall pre alarm
pub type FPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, FPAW_A, O>;
impl<'a, const O: u8> FPA_W<'a, O> {
    ///Any code executed outside the protected segment when the Firewall is opened will generate a system reset
    #[inline(always)]
    pub fn pre_arm_reset(self) -> &'a mut W {
        self.variant(FPAW_A::PreArmReset)
    }
    ///Any code executed outside the protected segment will close the Firewall
    #[inline(always)]
    pub fn pre_arm_set(self) -> &'a mut W {
        self.variant(FPAW_A::PreArmSet)
    }
}
///Field `VDS` reader - Volatile data shared
pub type VDS_R = crate::BitReader<VDSR_A>;
///Volatile data shared
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDSR_A {
    ///0: Volatile data segment is not shared and cannot be hit by a non protected executable code when the Firewall is closed
    NotShared = 0,
    ///1: Volatile data segment is shared with non protected application code
    Shared = 1,
}
impl From<VDSR_A> for bool {
    #[inline(always)]
    fn from(variant: VDSR_A) -> Self {
        variant as u8 != 0
    }
}
impl VDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VDSR_A {
        match self.bits {
            false => VDSR_A::NotShared,
            true => VDSR_A::Shared,
        }
    }
    ///Checks if the value of the field is `NotShared`
    #[inline(always)]
    pub fn is_not_shared(&self) -> bool {
        *self == VDSR_A::NotShared
    }
    ///Checks if the value of the field is `Shared`
    #[inline(always)]
    pub fn is_shared(&self) -> bool {
        *self == VDSR_A::Shared
    }
}
///Volatile data shared
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDSW_AW {
    ///0: Resets volatile data shared bit
    Reset = 0,
}
impl From<VDSW_AW> for bool {
    #[inline(always)]
    fn from(variant: VDSW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `VDS` writer - Volatile data shared
pub type VDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, VDSW_AW, O>;
impl<'a, const O: u8> VDS_W<'a, O> {
    ///Resets volatile data shared bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(VDSW_AW::Reset)
    }
}
///Field `VDE` reader - Volatile data execution
pub type VDE_R = crate::BitReader<VDER_A>;
///Volatile data execution
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDER_A {
    ///0: Volatile data segment cannot be executed if VDS = 0
    NotExecutable = 0,
    ///1: Volatile data segment is declared executable whatever VDS bit value
    Executable = 1,
}
impl From<VDER_A> for bool {
    #[inline(always)]
    fn from(variant: VDER_A) -> Self {
        variant as u8 != 0
    }
}
impl VDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VDER_A {
        match self.bits {
            false => VDER_A::NotExecutable,
            true => VDER_A::Executable,
        }
    }
    ///Checks if the value of the field is `NotExecutable`
    #[inline(always)]
    pub fn is_not_executable(&self) -> bool {
        *self == VDER_A::NotExecutable
    }
    ///Checks if the value of the field is `Executable`
    #[inline(always)]
    pub fn is_executable(&self) -> bool {
        *self == VDER_A::Executable
    }
}
///Volatile data execution
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDEW_AW {
    ///0: Resets volatile data execution bit
    Reset = 0,
}
impl From<VDEW_AW> for bool {
    #[inline(always)]
    fn from(variant: VDEW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `VDE` writer - Volatile data execution
pub type VDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, VDEW_AW, O>;
impl<'a, const O: u8> VDE_W<'a, O> {
    ///Resets volatile data execution bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(VDEW_AW::Reset)
    }
}
impl R {
    ///Bit 0 - Firewall pre alarm
    #[inline(always)]
    pub fn fpa(&self) -> FPA_R {
        FPA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Volatile data shared
    #[inline(always)]
    pub fn vds(&self) -> VDS_R {
        VDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Volatile data execution
    #[inline(always)]
    pub fn vde(&self) -> VDE_R {
        VDE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Firewall pre alarm
    #[inline(always)]
    #[must_use]
    pub fn fpa(&mut self) -> FPA_W<0> {
        FPA_W::new(self)
    }
    ///Bit 1 - Volatile data shared
    #[inline(always)]
    #[must_use]
    pub fn vds(&mut self) -> VDS_W<1> {
        VDS_W::new(self)
    }
    ///Bit 2 - Volatile data execution
    #[inline(always)]
    #[must_use]
    pub fn vde(&mut self) -> VDE_W<2> {
        VDE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
