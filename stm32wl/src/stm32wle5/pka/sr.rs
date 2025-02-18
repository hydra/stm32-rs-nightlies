///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `BUSY` reader - PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started.
pub type BUSY_R = crate::BitReader<BUSY_A>;
///PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    ///0: No operation in pgoress
    Idle = 0,
    ///1: Operation in progress
    Busy = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::Idle,
            true => BUSY_A::Busy,
        }
    }
    ///Checks if the value of the field is `Idle`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::Idle
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::Busy
    }
}
///Field `PROCENDF` reader - PKA End of Operation flag
pub type PROCENDF_R = crate::BitReader<PROCENDF_A>;
///PKA End of Operation flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROCENDF_A {
    ///0: Operation in progress
    InProgress = 0,
    ///1: PKA operation is completed - set when BUSY is deasserted
    Completed = 1,
}
impl From<PROCENDF_A> for bool {
    #[inline(always)]
    fn from(variant: PROCENDF_A) -> Self {
        variant as u8 != 0
    }
}
impl PROCENDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PROCENDF_A {
        match self.bits {
            false => PROCENDF_A::InProgress,
            true => PROCENDF_A::Completed,
        }
    }
    ///Checks if the value of the field is `InProgress`
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == PROCENDF_A::InProgress
    }
    ///Checks if the value of the field is `Completed`
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == PROCENDF_A::Completed
    }
}
///Field `RAMERRF` reader - PKA RAM error flag
pub type RAMERRF_R = crate::BitReader<RAMERRF_A>;
///PKA RAM error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMERRF_A {
    ///0: No error
    NoError = 0,
    ///1: An AHB access to the PKA RAM occurred while the PKA core was computing and using its internal RAM (AHB PKA_RAM access are not allowed while PKA operation is in progress)
    Error = 1,
}
impl From<RAMERRF_A> for bool {
    #[inline(always)]
    fn from(variant: RAMERRF_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMERRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RAMERRF_A {
        match self.bits {
            false => RAMERRF_A::NoError,
            true => RAMERRF_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RAMERRF_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RAMERRF_A::Error
    }
}
///Field `ADDRERRF` reader - Address error flag
pub type ADDRERRF_R = crate::BitReader<ADDRERRF_A>;
///Address error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRERRF_A {
    ///0: No error
    NoError = 0,
    ///1: Address access is out of range (unmapped address)
    Error = 1,
}
impl From<ADDRERRF_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRF_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRERRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDRERRF_A {
        match self.bits {
            false => ADDRERRF_A::NoError,
            true => ADDRERRF_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ADDRERRF_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ADDRERRF_A::Error
    }
}
impl R {
    ///Bit 16 - PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started.
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PKA End of Operation flag
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - PKA RAM error flag
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Address error flag
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
///status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
