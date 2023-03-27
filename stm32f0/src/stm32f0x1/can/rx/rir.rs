///Register `RIR` reader
pub struct R(crate::R<RIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RTR` reader - RTR
pub type RTR_R = crate::BitReader<RTR_A>;
///RTR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTR_A {
    ///0: Data frame
    Data = 0,
    ///1: Remote frame
    Remote = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
impl RTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::Data,
            true => RTR_A::Remote,
        }
    }
    ///Checks if the value of the field is `Data`
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RTR_A::Data
    }
    ///Checks if the value of the field is `Remote`
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == RTR_A::Remote
    }
}
///Field `IDE` reader - IDE
pub type IDE_R = crate::BitReader<IDE_A>;
///IDE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDE_A {
    ///0: Standard identifier
    Standard = 0,
    ///1: Extended identifier
    Extended = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::Standard,
            true => IDE_A::Extended,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IDE_A::Standard
    }
    ///Checks if the value of the field is `Extended`
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == IDE_A::Extended
    }
}
///Field `EXID` reader - EXID
pub type EXID_R = crate::FieldReader<u32, u32>;
///Field `STID` reader - STID
pub type STID_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
///CAN_RI0R
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rir](index.html) module
pub struct RIR_SPEC;
impl crate::RegisterSpec for RIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rir::R](R) reader structure
impl crate::Readable for RIR_SPEC {
    type Reader = R;
}
///`reset()` method sets RIR to value 0
impl crate::Resettable for RIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
