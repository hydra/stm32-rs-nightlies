///Register `MISR` reader
pub struct R(crate::R<MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TAMP1MF` reader - TAMP1MF:
pub type TAMP1MF_R = crate::BitReader<TAMP1MF_A>;
///TAMP1MF:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1MF_A {
    ///0: No tamper detected - Masked
    Idle = 0,
    ///1: Tamper detected - Masked
    Tamper = 1,
}
impl From<TAMP1MF_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1MF_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1MF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP1MF_A {
        match self.bits {
            false => TAMP1MF_A::Idle,
            true => TAMP1MF_A::Tamper,
        }
    }
    ///Checks if the value of the field is `Idle`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TAMP1MF_A::Idle
    }
    ///Checks if the value of the field is `Tamper`
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        *self == TAMP1MF_A::Tamper
    }
}
///Field `TAMP2MF` reader - TAMP2MF
pub use TAMP1MF_R as TAMP2MF_R;
///Field `TAMP3MF` reader - TAMP3MF
pub use TAMP1MF_R as TAMP3MF_R;
///Field `ITAMP3MF` reader - ITAMP3MF
pub type ITAMP3MF_R = crate::BitReader<ITAMP3MF_A>;
///ITAMP3MF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3MF_A {
    ///0: No tamper detected - Masked
    Idle = 0,
    ///1: Internal tamper detected - Masked
    Tamper = 1,
}
impl From<ITAMP3MF_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3MF_A) -> Self {
        variant as u8 != 0
    }
}
impl ITAMP3MF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITAMP3MF_A {
        match self.bits {
            false => ITAMP3MF_A::Idle,
            true => ITAMP3MF_A::Tamper,
        }
    }
    ///Checks if the value of the field is `Idle`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ITAMP3MF_A::Idle
    }
    ///Checks if the value of the field is `Tamper`
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        *self == ITAMP3MF_A::Tamper
    }
}
///Field `ITAMP5MF` reader - ITAMP5MF
pub use ITAMP3MF_R as ITAMP5MF_R;
///Field `ITAMP6MF` reader - ITAMP6MF
pub use ITAMP3MF_R as ITAMP6MF_R;
///Field `ITAMP8MF` reader - ITAMP8MF
pub use ITAMP3MF_R as ITAMP8MF_R;
impl R {
    ///Bit 0 - TAMP1MF:
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2MF
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3MF
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 18 - ITAMP3MF
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ITAMP5MF
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ITAMP6MF
    #[inline(always)]
    pub fn itamp6mf(&self) -> ITAMP6MF_R {
        ITAMP6MF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - ITAMP8MF
    #[inline(always)]
    pub fn itamp8mf(&self) -> ITAMP8MF_R {
        ITAMP8MF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
///TAMP masked interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [misr](index.html) module
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [misr::R](R) reader structure
impl crate::Readable for MISR_SPEC {
    type Reader = R;
}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
