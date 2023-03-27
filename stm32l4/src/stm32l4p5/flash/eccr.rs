///Register `ECCR` reader
pub struct R(crate::R<ECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ECCR` writer
pub struct W(crate::W<ECCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCR_SPEC>;
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
impl From<crate::W<ECCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDR_ECC` reader - ECC fail address
pub type ADDR_ECC_R = crate::FieldReader<u32, u32>;
///Field `BK_ECC` reader - ECC fail bank
pub type BK_ECC_R = crate::BitReader<BK_ECC_A>;
///ECC fail bank
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK_ECC_A {
    ///0: Bank 1
    Bank1 = 0,
    ///1: Bank 2
    Bank2 = 1,
}
impl From<BK_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: BK_ECC_A) -> Self {
        variant as u8 != 0
    }
}
impl BK_ECC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BK_ECC_A {
        match self.bits {
            false => BK_ECC_A::Bank1,
            true => BK_ECC_A::Bank2,
        }
    }
    ///Checks if the value of the field is `Bank1`
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BK_ECC_A::Bank1
    }
    ///Checks if the value of the field is `Bank2`
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BK_ECC_A::Bank2
    }
}
///Field `SYSF_ECC` reader - System Flash ECC fail
pub type SYSF_ECC_R = crate::BitReader<SYSF_ECC_A>;
///System Flash ECC fail
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSF_ECC_A {
    ///1: This bit indicates that the ECC error correction or double ECC error detection is located in the System Flash
    InSystemFlash = 1,
}
impl From<SYSF_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: SYSF_ECC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSF_ECC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSF_ECC_A> {
        match self.bits {
            true => Some(SYSF_ECC_A::InSystemFlash),
            _ => None,
        }
    }
    ///Checks if the value of the field is `InSystemFlash`
    #[inline(always)]
    pub fn is_in_system_flash(&self) -> bool {
        *self == SYSF_ECC_A::InSystemFlash
    }
}
///Field `ECCIE` reader - ECC correction interrupt enable
pub type ECCIE_R = crate::BitReader<ECCIE_A>;
///ECC correction interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCIE_A {
    ///0: ECCC interrupt disabled
    Disabled = 0,
    ///1: ECCC interrupt enabled
    Enabled = 1,
}
impl From<ECCIE_A> for bool {
    #[inline(always)]
    fn from(variant: ECCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECCIE_A {
        match self.bits {
            false => ECCIE_A::Disabled,
            true => ECCIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECCIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECCIE_A::Enabled
    }
}
///Field `ECCIE` writer - ECC correction interrupt enable
pub type ECCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, ECCIE_A, O>;
impl<'a, const O: u8> ECCIE_W<'a, O> {
    ///ECCC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECCIE_A::Disabled)
    }
    ///ECCC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECCIE_A::Enabled)
    }
}
///Field `ECCC2` reader - ECC2 correction
pub type ECCC2_R = crate::BitReader<ECCC2R_A>;
///ECC2 correction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCC2R_A {
    ///0: No ECC error detected on MSB
    NoError = 0,
    ///1: Set by hardware when one ECC errors have been detected and corrected on MSB
    Error = 1,
}
impl From<ECCC2R_A> for bool {
    #[inline(always)]
    fn from(variant: ECCC2R_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECCC2R_A {
        match self.bits {
            false => ECCC2R_A::NoError,
            true => ECCC2R_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCC2R_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCC2R_A::Error
    }
}
///ECC2 correction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCC2W_AW {
    ///1: Cleared by writing 1
    Clear = 1,
}
impl From<ECCC2W_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCC2W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCC2` writer - ECC2 correction
pub type ECCC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, ECCC2W_AW, O>;
impl<'a, const O: u8> ECCC2_W<'a, O> {
    ///Cleared by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ECCC2W_AW::Clear)
    }
}
///Field `ECCD2` reader - ECC2 detection
pub type ECCD2_R = crate::BitReader<ECCD2R_A>;
///ECC2 detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCD2R_A {
    ///0: No double ECC errors detected on MSB
    NoError = 0,
    ///1: Set by hardware when two ECC errors have been detected on MSB
    Error = 1,
}
impl From<ECCD2R_A> for bool {
    #[inline(always)]
    fn from(variant: ECCD2R_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCD2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECCD2R_A {
        match self.bits {
            false => ECCD2R_A::NoError,
            true => ECCD2R_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCD2R_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCD2R_A::Error
    }
}
///ECC2 detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCD2W_AW {
    ///1: Cleared by writing 1
    Clear = 1,
}
impl From<ECCD2W_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCD2W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCD2` writer - ECC2 detection
pub type ECCD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, ECCD2W_AW, O>;
impl<'a, const O: u8> ECCD2_W<'a, O> {
    ///Cleared by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ECCD2W_AW::Clear)
    }
}
///Field `ECCC` reader - ECC correction
pub type ECCC_R = crate::BitReader<ECCCR_A>;
///ECC correction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCR_A {
    ///0: No ECC error detected on LSB
    NoError = 0,
    ///1: Set by hardware when one ECC errors have been detected and corrected on LSB
    Error = 1,
}
impl From<ECCCR_A> for bool {
    #[inline(always)]
    fn from(variant: ECCCR_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECCCR_A {
        match self.bits {
            false => ECCCR_A::NoError,
            true => ECCCR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCCR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCCR_A::Error
    }
}
///ECC correction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCW_AW {
    ///1: Cleared by writing 1
    Clear = 1,
}
impl From<ECCCW_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCCW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCC` writer - ECC correction
pub type ECCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, ECCCW_AW, O>;
impl<'a, const O: u8> ECCC_W<'a, O> {
    ///Cleared by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ECCCW_AW::Clear)
    }
}
///Field `ECCD` reader - ECC detection
pub type ECCD_R = crate::BitReader<ECCDR_A>;
///ECC detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCDR_A {
    ///0: No double ECC errors detected on LSB
    NoError = 0,
    ///1: Set by hardware when two ECC errors have been detected on LSB
    Error = 1,
}
impl From<ECCDR_A> for bool {
    #[inline(always)]
    fn from(variant: ECCDR_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECCDR_A {
        match self.bits {
            false => ECCDR_A::NoError,
            true => ECCDR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCDR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCDR_A::Error
    }
}
///ECC detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCDW_AW {
    ///1: Cleared by writing 1
    Clear = 1,
}
impl From<ECCDW_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCDW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCD` writer - ECC detection
pub type ECCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, ECCDW_AW, O>;
impl<'a, const O: u8> ECCD_W<'a, O> {
    ///Cleared by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ECCDW_AW::Clear)
    }
}
impl R {
    ///Bits 0:20 - ECC fail address
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new(self.bits & 0x001f_ffff)
    }
    ///Bit 19 - ECC fail bank
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - System Flash ECC fail
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - ECC2 correction
    #[inline(always)]
    pub fn eccc2(&self) -> ECCC2_R {
        ECCC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ECC2 detection
    #[inline(always)]
    pub fn eccd2(&self) -> ECCD2_R {
        ECCD2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eccie(&mut self) -> ECCIE_W<24> {
        ECCIE_W::new(self)
    }
    ///Bit 28 - ECC2 correction
    #[inline(always)]
    #[must_use]
    pub fn eccc2(&mut self) -> ECCC2_W<28> {
        ECCC2_W::new(self)
    }
    ///Bit 29 - ECC2 detection
    #[inline(always)]
    #[must_use]
    pub fn eccd2(&mut self) -> ECCD2_W<29> {
        ECCD2_W::new(self)
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    #[must_use]
    pub fn eccc(&mut self) -> ECCC_W<30> {
        ECCC_W::new(self)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    #[must_use]
    pub fn eccd(&mut self) -> ECCD_W<31> {
        ECCD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash ECC register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eccr](index.html) module
pub struct ECCR_SPEC;
impl crate::RegisterSpec for ECCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eccr::R](R) reader structure
impl crate::Readable for ECCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eccr::W](W) writer structure
impl crate::Writable for ECCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ECCR to value 0
impl crate::Resettable for ECCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
