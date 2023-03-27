///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TMEIE` reader - TMEIE
pub type TMEIE_R = crate::BitReader<TMEIE_A>;
///TMEIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMEIE_A {
    ///0: No interrupt when RQCPx bit is set
    Disabled = 0,
    ///1: Interrupt generated when RQCPx bit is set
    Enabled = 1,
}
impl From<TMEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TMEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TMEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TMEIE_A {
        match self.bits {
            false => TMEIE_A::Disabled,
            true => TMEIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TMEIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TMEIE_A::Enabled
    }
}
///Field `TMEIE` writer - TMEIE
pub type TMEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TMEIE_A, O>;
impl<'a, const O: u8> TMEIE_W<'a, O> {
    ///No interrupt when RQCPx bit is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TMEIE_A::Disabled)
    }
    ///Interrupt generated when RQCPx bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TMEIE_A::Enabled)
    }
}
///Field `FMPIE0` reader - FMPIE0
pub type FMPIE0_R = crate::BitReader<FMPIE0_A>;
///FMPIE0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPIE0_A {
    ///0: No interrupt generated when state of FMP\[1:0\]
    ///bits are not 00
    Disabled = 0,
    ///1: Interrupt generated when state of FMP\[1:0\]
    ///bits are not 00b
    Enabled = 1,
}
impl From<FMPIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FMPIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl FMPIE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FMPIE0_A {
        match self.bits {
            false => FMPIE0_A::Disabled,
            true => FMPIE0_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMPIE0_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMPIE0_A::Enabled
    }
}
///Field `FMPIE0` writer - FMPIE0
pub type FMPIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, FMPIE0_A, O>;
impl<'a, const O: u8> FMPIE0_W<'a, O> {
    ///No interrupt generated when state of FMP\[1:0\]
    ///bits are not 00
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMPIE0_A::Disabled)
    }
    ///Interrupt generated when state of FMP\[1:0\]
    ///bits are not 00b
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMPIE0_A::Enabled)
    }
}
///Field `FFIE0` reader - FFIE0
pub type FFIE0_R = crate::BitReader<FFIE0_A>;
///FFIE0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFIE0_A {
    ///0: No interrupt when FULL bit is set
    Disabled = 0,
    ///1: Interrupt generated when FULL bit is set
    Enabled = 1,
}
impl From<FFIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FFIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl FFIE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FFIE0_A {
        match self.bits {
            false => FFIE0_A::Disabled,
            true => FFIE0_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FFIE0_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FFIE0_A::Enabled
    }
}
///Field `FFIE0` writer - FFIE0
pub type FFIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, FFIE0_A, O>;
impl<'a, const O: u8> FFIE0_W<'a, O> {
    ///No interrupt when FULL bit is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FFIE0_A::Disabled)
    }
    ///Interrupt generated when FULL bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FFIE0_A::Enabled)
    }
}
///Field `FOVIE0` reader - FOVIE0
pub type FOVIE0_R = crate::BitReader<FOVIE0_A>;
///FOVIE0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVIE0_A {
    ///0: No interrupt when FOVR bit is set
    Disabled = 0,
    ///1: Interrupt generated when FOVR bit is set
    Enabled = 1,
}
impl From<FOVIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FOVIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl FOVIE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FOVIE0_A {
        match self.bits {
            false => FOVIE0_A::Disabled,
            true => FOVIE0_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FOVIE0_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FOVIE0_A::Enabled
    }
}
///Field `FOVIE0` writer - FOVIE0
pub type FOVIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, FOVIE0_A, O>;
impl<'a, const O: u8> FOVIE0_W<'a, O> {
    ///No interrupt when FOVR bit is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FOVIE0_A::Disabled)
    }
    ///Interrupt generated when FOVR bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FOVIE0_A::Enabled)
    }
}
///Field `FMPIE1` reader - FMPIE1
pub type FMPIE1_R = crate::BitReader<FMPIE1_A>;
///FMPIE1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPIE1_A {
    ///0: No interrupt generated when state of FMP\[1:0\]
    ///bits are not 00b
    Disabled = 0,
    ///1: Interrupt generated when state of FMP\[1:0\]
    ///bits are not 00b
    Enabled = 1,
}
impl From<FMPIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FMPIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl FMPIE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FMPIE1_A {
        match self.bits {
            false => FMPIE1_A::Disabled,
            true => FMPIE1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMPIE1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMPIE1_A::Enabled
    }
}
///Field `FMPIE1` writer - FMPIE1
pub type FMPIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, FMPIE1_A, O>;
impl<'a, const O: u8> FMPIE1_W<'a, O> {
    ///No interrupt generated when state of FMP\[1:0\]
    ///bits are not 00b
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMPIE1_A::Disabled)
    }
    ///Interrupt generated when state of FMP\[1:0\]
    ///bits are not 00b
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMPIE1_A::Enabled)
    }
}
///Field `FFIE1` reader - FFIE1
pub type FFIE1_R = crate::BitReader<FFIE1_A>;
///FFIE1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFIE1_A {
    ///0: No interrupt when FULL bit is set
    Disabled = 0,
    ///1: Interrupt generated when FULL bit is set
    Enabled = 1,
}
impl From<FFIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FFIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl FFIE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FFIE1_A {
        match self.bits {
            false => FFIE1_A::Disabled,
            true => FFIE1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FFIE1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FFIE1_A::Enabled
    }
}
///Field `FFIE1` writer - FFIE1
pub type FFIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, FFIE1_A, O>;
impl<'a, const O: u8> FFIE1_W<'a, O> {
    ///No interrupt when FULL bit is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FFIE1_A::Disabled)
    }
    ///Interrupt generated when FULL bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FFIE1_A::Enabled)
    }
}
///Field `FOVIE1` reader - FOVIE1
pub type FOVIE1_R = crate::BitReader<FOVIE1_A>;
///FOVIE1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVIE1_A {
    ///0: No interrupt when FOVR is set
    Disabled = 0,
    ///1: Interrupt generation when FOVR is set
    Enabled = 1,
}
impl From<FOVIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FOVIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl FOVIE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FOVIE1_A {
        match self.bits {
            false => FOVIE1_A::Disabled,
            true => FOVIE1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FOVIE1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FOVIE1_A::Enabled
    }
}
///Field `FOVIE1` writer - FOVIE1
pub type FOVIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, FOVIE1_A, O>;
impl<'a, const O: u8> FOVIE1_W<'a, O> {
    ///No interrupt when FOVR is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FOVIE1_A::Disabled)
    }
    ///Interrupt generation when FOVR is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FOVIE1_A::Enabled)
    }
}
///Field `EWGIE` reader - EWGIE
pub type EWGIE_R = crate::BitReader<EWGIE_A>;
///EWGIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWGIE_A {
    ///0: ERRI bit will not be set when EWGF is set
    Disabled = 0,
    ///1: ERRI bit will be set when EWGF is set
    Enabled = 1,
}
impl From<EWGIE_A> for bool {
    #[inline(always)]
    fn from(variant: EWGIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EWGIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWGIE_A {
        match self.bits {
            false => EWGIE_A::Disabled,
            true => EWGIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWGIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWGIE_A::Enabled
    }
}
///Field `EWGIE` writer - EWGIE
pub type EWGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EWGIE_A, O>;
impl<'a, const O: u8> EWGIE_W<'a, O> {
    ///ERRI bit will not be set when EWGF is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWGIE_A::Disabled)
    }
    ///ERRI bit will be set when EWGF is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWGIE_A::Enabled)
    }
}
///Field `EPVIE` reader - EPVIE
pub type EPVIE_R = crate::BitReader<EPVIE_A>;
///EPVIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPVIE_A {
    ///0: ERRI bit will not be set when EPVF is set
    Disabled = 0,
    ///1: ERRI bit will be set when EPVF is set
    Enabled = 1,
}
impl From<EPVIE_A> for bool {
    #[inline(always)]
    fn from(variant: EPVIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EPVIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EPVIE_A {
        match self.bits {
            false => EPVIE_A::Disabled,
            true => EPVIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EPVIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EPVIE_A::Enabled
    }
}
///Field `EPVIE` writer - EPVIE
pub type EPVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EPVIE_A, O>;
impl<'a, const O: u8> EPVIE_W<'a, O> {
    ///ERRI bit will not be set when EPVF is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EPVIE_A::Disabled)
    }
    ///ERRI bit will be set when EPVF is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EPVIE_A::Enabled)
    }
}
///Field `BOFIE` reader - BOFIE
pub type BOFIE_R = crate::BitReader<BOFIE_A>;
///BOFIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFIE_A {
    ///0: ERRI bit will not be set when BOFF is set
    Disabled = 0,
    ///1: ERRI bit will be set when BOFF is set
    Enabled = 1,
}
impl From<BOFIE_A> for bool {
    #[inline(always)]
    fn from(variant: BOFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BOFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BOFIE_A {
        match self.bits {
            false => BOFIE_A::Disabled,
            true => BOFIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOFIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOFIE_A::Enabled
    }
}
///Field `BOFIE` writer - BOFIE
pub type BOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, BOFIE_A, O>;
impl<'a, const O: u8> BOFIE_W<'a, O> {
    ///ERRI bit will not be set when BOFF is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOFIE_A::Disabled)
    }
    ///ERRI bit will be set when BOFF is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOFIE_A::Enabled)
    }
}
///Field `LECIE` reader - LECIE
pub type LECIE_R = crate::BitReader<LECIE_A>;
///LECIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LECIE_A {
    ///0: ERRI bit will not be set when the error code in LEC\[2:0\]
    ///is set by hardware on error detection
    Disabled = 0,
    ///1: ERRI bit will be set when the error code in LEC\[2:0\]
    ///is set by hardware on error detection
    Enabled = 1,
}
impl From<LECIE_A> for bool {
    #[inline(always)]
    fn from(variant: LECIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LECIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LECIE_A {
        match self.bits {
            false => LECIE_A::Disabled,
            true => LECIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LECIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LECIE_A::Enabled
    }
}
///Field `LECIE` writer - LECIE
pub type LECIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, LECIE_A, O>;
impl<'a, const O: u8> LECIE_W<'a, O> {
    ///ERRI bit will not be set when the error code in LEC\[2:0\]
    ///is set by hardware on error detection
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LECIE_A::Disabled)
    }
    ///ERRI bit will be set when the error code in LEC\[2:0\]
    ///is set by hardware on error detection
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LECIE_A::Enabled)
    }
}
///Field `ERRIE` reader - ERRIE
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
///ERRIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    ///0: No interrupt will be generated when an error condition is pending in the CAN_ESR
    Disabled = 0,
    ///1: An interrupt will be generation when an error condition is pending in the CAN_ESR
    Enabled = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::Disabled,
            true => ERRIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::Enabled
    }
}
///Field `ERRIE` writer - ERRIE
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ERRIE_A, O>;
impl<'a, const O: u8> ERRIE_W<'a, O> {
    ///No interrupt will be generated when an error condition is pending in the CAN_ESR
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Disabled)
    }
    ///An interrupt will be generation when an error condition is pending in the CAN_ESR
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Enabled)
    }
}
///Field `WKUIE` reader - WKUIE
pub type WKUIE_R = crate::BitReader<WKUIE_A>;
///WKUIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUIE_A {
    ///0: No interrupt when WKUI is set
    Disabled = 0,
    ///1: Interrupt generated when WKUI bit is set
    Enabled = 1,
}
impl From<WKUIE_A> for bool {
    #[inline(always)]
    fn from(variant: WKUIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WKUIE_A {
        match self.bits {
            false => WKUIE_A::Disabled,
            true => WKUIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WKUIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WKUIE_A::Enabled
    }
}
///Field `WKUIE` writer - WKUIE
pub type WKUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, WKUIE_A, O>;
impl<'a, const O: u8> WKUIE_W<'a, O> {
    ///No interrupt when WKUI is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WKUIE_A::Disabled)
    }
    ///Interrupt generated when WKUI bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WKUIE_A::Enabled)
    }
}
///Field `SLKIE` reader - SLKIE
pub type SLKIE_R = crate::BitReader<SLKIE_A>;
///SLKIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLKIE_A {
    ///0: No interrupt when SLAKI bit is set
    Disabled = 0,
    ///1: Interrupt generated when SLAKI bit is set
    Enabled = 1,
}
impl From<SLKIE_A> for bool {
    #[inline(always)]
    fn from(variant: SLKIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SLKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SLKIE_A {
        match self.bits {
            false => SLKIE_A::Disabled,
            true => SLKIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLKIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLKIE_A::Enabled
    }
}
///Field `SLKIE` writer - SLKIE
pub type SLKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, SLKIE_A, O>;
impl<'a, const O: u8> SLKIE_W<'a, O> {
    ///No interrupt when SLAKI bit is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLKIE_A::Disabled)
    }
    ///Interrupt generated when SLAKI bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLKIE_A::Enabled)
    }
}
impl R {
    ///Bit 0 - TMEIE
    #[inline(always)]
    pub fn tmeie(&self) -> TMEIE_R {
        TMEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FMPIE0
    #[inline(always)]
    pub fn fmpie0(&self) -> FMPIE0_R {
        FMPIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FFIE0
    #[inline(always)]
    pub fn ffie0(&self) -> FFIE0_R {
        FFIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FOVIE0
    #[inline(always)]
    pub fn fovie0(&self) -> FOVIE0_R {
        FOVIE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FMPIE1
    #[inline(always)]
    pub fn fmpie1(&self) -> FMPIE1_R {
        FMPIE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - FFIE1
    #[inline(always)]
    pub fn ffie1(&self) -> FFIE1_R {
        FFIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FOVIE1
    #[inline(always)]
    pub fn fovie1(&self) -> FOVIE1_R {
        FOVIE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - EWGIE
    #[inline(always)]
    pub fn ewgie(&self) -> EWGIE_R {
        EWGIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - EPVIE
    #[inline(always)]
    pub fn epvie(&self) -> EPVIE_R {
        EPVIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BOFIE
    #[inline(always)]
    pub fn bofie(&self) -> BOFIE_R {
        BOFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LECIE
    #[inline(always)]
    pub fn lecie(&self) -> LECIE_R {
        LECIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - ERRIE
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - WKUIE
    #[inline(always)]
    pub fn wkuie(&self) -> WKUIE_R {
        WKUIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SLKIE
    #[inline(always)]
    pub fn slkie(&self) -> SLKIE_R {
        SLKIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TMEIE
    #[inline(always)]
    #[must_use]
    pub fn tmeie(&mut self) -> TMEIE_W<0> {
        TMEIE_W::new(self)
    }
    ///Bit 1 - FMPIE0
    #[inline(always)]
    #[must_use]
    pub fn fmpie0(&mut self) -> FMPIE0_W<1> {
        FMPIE0_W::new(self)
    }
    ///Bit 2 - FFIE0
    #[inline(always)]
    #[must_use]
    pub fn ffie0(&mut self) -> FFIE0_W<2> {
        FFIE0_W::new(self)
    }
    ///Bit 3 - FOVIE0
    #[inline(always)]
    #[must_use]
    pub fn fovie0(&mut self) -> FOVIE0_W<3> {
        FOVIE0_W::new(self)
    }
    ///Bit 4 - FMPIE1
    #[inline(always)]
    #[must_use]
    pub fn fmpie1(&mut self) -> FMPIE1_W<4> {
        FMPIE1_W::new(self)
    }
    ///Bit 5 - FFIE1
    #[inline(always)]
    #[must_use]
    pub fn ffie1(&mut self) -> FFIE1_W<5> {
        FFIE1_W::new(self)
    }
    ///Bit 6 - FOVIE1
    #[inline(always)]
    #[must_use]
    pub fn fovie1(&mut self) -> FOVIE1_W<6> {
        FOVIE1_W::new(self)
    }
    ///Bit 8 - EWGIE
    #[inline(always)]
    #[must_use]
    pub fn ewgie(&mut self) -> EWGIE_W<8> {
        EWGIE_W::new(self)
    }
    ///Bit 9 - EPVIE
    #[inline(always)]
    #[must_use]
    pub fn epvie(&mut self) -> EPVIE_W<9> {
        EPVIE_W::new(self)
    }
    ///Bit 10 - BOFIE
    #[inline(always)]
    #[must_use]
    pub fn bofie(&mut self) -> BOFIE_W<10> {
        BOFIE_W::new(self)
    }
    ///Bit 11 - LECIE
    #[inline(always)]
    #[must_use]
    pub fn lecie(&mut self) -> LECIE_W<11> {
        LECIE_W::new(self)
    }
    ///Bit 15 - ERRIE
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<15> {
        ERRIE_W::new(self)
    }
    ///Bit 16 - WKUIE
    #[inline(always)]
    #[must_use]
    pub fn wkuie(&mut self) -> WKUIE_W<16> {
        WKUIE_W::new(self)
    }
    ///Bit 17 - SLKIE
    #[inline(always)]
    #[must_use]
    pub fn slkie(&mut self) -> SLKIE_W<17> {
        SLKIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
