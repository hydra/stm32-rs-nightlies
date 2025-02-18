///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ADDRDY_MST` reader - ADDRDY_MST
pub type ADDRDY_MST_R = crate::BitReader<bool>;
///Field `EOSMP_MST` reader - EOSMP_MST
pub type EOSMP_MST_R = crate::BitReader<EOSMP_MST_A>;
///EOSMP_MST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMP_MST_A {
    ///0: End of sampling phase no yet reached
    NotEnded = 0,
    ///1: End of sampling phase reached
    Ended = 1,
}
impl From<EOSMP_MST_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_MST_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSMP_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOSMP_MST_A {
        match self.bits {
            false => EOSMP_MST_A::NotEnded,
            true => EOSMP_MST_A::Ended,
        }
    }
    ///Checks if the value of the field is `NotEnded`
    #[inline(always)]
    pub fn is_not_ended(&self) -> bool {
        *self == EOSMP_MST_A::NotEnded
    }
    ///Checks if the value of the field is `Ended`
    #[inline(always)]
    pub fn is_ended(&self) -> bool {
        *self == EOSMP_MST_A::Ended
    }
}
///Field `EOC_MST` reader - EOC_MST
pub type EOC_MST_R = crate::BitReader<EOC_MST_A>;
///EOC_MST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOC_MST_A {
    ///0: Regular conversion is not complete
    NotComplete = 0,
    ///1: Regular conversion complete
    Complete = 1,
}
impl From<EOC_MST_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_MST_A) -> Self {
        variant as u8 != 0
    }
}
impl EOC_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOC_MST_A {
        match self.bits {
            false => EOC_MST_A::NotComplete,
            true => EOC_MST_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC_MST_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC_MST_A::Complete
    }
}
///Field `EOS_MST` reader - EOS_MST
pub type EOS_MST_R = crate::BitReader<EOS_MST_A>;
///EOS_MST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOS_MST_A {
    ///0: Regular sequence is not complete
    NotComplete = 0,
    ///1: Regular sequence complete
    Complete = 1,
}
impl From<EOS_MST_A> for bool {
    #[inline(always)]
    fn from(variant: EOS_MST_A) -> Self {
        variant as u8 != 0
    }
}
impl EOS_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOS_MST_A {
        match self.bits {
            false => EOS_MST_A::NotComplete,
            true => EOS_MST_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOS_MST_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOS_MST_A::Complete
    }
}
///Field `OVR_MST` reader - OVR_MST
pub type OVR_MST_R = crate::BitReader<OVR_MST_A>;
///OVR_MST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_MST_A {
    ///0: No overrun occurred
    NoOverrun = 0,
    ///1: Overrun occurred
    Overrun = 1,
}
impl From<OVR_MST_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_MST_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVR_MST_A {
        match self.bits {
            false => OVR_MST_A::NoOverrun,
            true => OVR_MST_A::Overrun,
        }
    }
    ///Checks if the value of the field is `NoOverrun`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_MST_A::NoOverrun
    }
    ///Checks if the value of the field is `Overrun`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_MST_A::Overrun
    }
}
///Field `JEOC_MST` reader - JEOC_MST
pub type JEOC_MST_R = crate::BitReader<JEOC_MST_A>;
///JEOC_MST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOC_MST_A {
    ///0: Injected conversion is not complete
    NotComplete = 0,
    ///1: Injected conversion complete
    Complete = 1,
}
impl From<JEOC_MST_A> for bool {
    #[inline(always)]
    fn from(variant: JEOC_MST_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOC_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEOC_MST_A {
        match self.bits {
            false => JEOC_MST_A::NotComplete,
            true => JEOC_MST_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC_MST_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC_MST_A::Complete
    }
}
///Field `JEOS_MST` reader - JEOS_MST
pub type JEOS_MST_R = crate::BitReader<JEOS_MST_A>;
///JEOS_MST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOS_MST_A {
    ///0: Injected sequence is not complete
    NotComplete = 0,
    ///1: Injected sequence complete
    Complete = 1,
}
impl From<JEOS_MST_A> for bool {
    #[inline(always)]
    fn from(variant: JEOS_MST_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOS_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEOS_MST_A {
        match self.bits {
            false => JEOS_MST_A::NotComplete,
            true => JEOS_MST_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOS_MST_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOS_MST_A::Complete
    }
}
///Field `AWD1_MST` reader - AWD1_MST
pub type AWD1_MST_R = crate::BitReader<AWD1_MST_A>;
///AWD1_MST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1_MST_A {
    ///0: No analog watchdog event occurred
    NoEvent = 0,
    ///1: Analog watchdog event occurred
    Event = 1,
}
impl From<AWD1_MST_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1_MST_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD1_MST_A {
        match self.bits {
            false => AWD1_MST_A::NoEvent,
            true => AWD1_MST_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1_MST_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1_MST_A::Event
    }
}
///Field `AWD2_MST` reader - AWD2_MST
pub use AWD1_MST_R as AWD2_MST_R;
///Field `AWD3_MST` reader - AWD3_MST
pub use AWD1_MST_R as AWD3_MST_R;
///Field `JQOVF_MST` reader - JQOVF_MST
pub type JQOVF_MST_R = crate::BitReader<JQOVF_MST_A>;
///JQOVF_MST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVF_MST_A {
    ///0: No injected context queue overflow has occurred
    NoOverflow = 0,
    ///1: Injected context queue overflow has occurred
    Overflow = 1,
}
impl From<JQOVF_MST_A> for bool {
    #[inline(always)]
    fn from(variant: JQOVF_MST_A) -> Self {
        variant as u8 != 0
    }
}
impl JQOVF_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JQOVF_MST_A {
        match self.bits {
            false => JQOVF_MST_A::NoOverflow,
            true => JQOVF_MST_A::Overflow,
        }
    }
    ///Checks if the value of the field is `NoOverflow`
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == JQOVF_MST_A::NoOverflow
    }
    ///Checks if the value of the field is `Overflow`
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == JQOVF_MST_A::Overflow
    }
}
///Field `ADRDY_SLV` reader - ADRDY_SLV
pub type ADRDY_SLV_R = crate::BitReader<ADRDY_SLV_A>;
///ADRDY_SLV
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDY_SLV_A {
    ///0: ADC is not ready to start conversion
    NotReady = 0,
    ///1: ADC is ready to start conversion
    Ready = 1,
}
impl From<ADRDY_SLV_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_SLV_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRDY_SLV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADRDY_SLV_A {
        match self.bits {
            false => ADRDY_SLV_A::NotReady,
            true => ADRDY_SLV_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDY_SLV_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDY_SLV_A::Ready
    }
}
///Field `AWD1_SLV` reader - Analog watchdog 1 flag of the slave ADC
pub use AWD1_MST_R as AWD1_SLV_R;
///Field `AWD2_SLV` reader - Analog watchdog 2 flag of the slave ADC
pub use AWD1_MST_R as AWD2_SLV_R;
///Field `AWD3_SLV` reader - Analog watchdog 3 flag of the slave ADC
pub use AWD1_MST_R as AWD3_SLV_R;
///Field `EOC_SLV` reader - End of regular conversion of the slave ADC
pub use EOC_MST_R as EOC_SLV_R;
///Field `EOSMP_SLV` reader - EOSMP_SLV
pub use EOSMP_MST_R as EOSMP_SLV_R;
///Field `EOS_SLV` reader - End of regular sequence flag of the slave ADC
pub use EOS_MST_R as EOS_SLV_R;
///Field `JEOC_SLV` reader - End of injected conversion flag of the slave ADC
pub use JEOC_MST_R as JEOC_SLV_R;
///Field `JEOS_SLV` reader - End of injected sequence flag of the slave ADC
pub use JEOS_MST_R as JEOS_SLV_R;
///Field `JQOVF_SLV` reader - Injected Context Queue Overflow flag of the slave ADC
pub use JQOVF_MST_R as JQOVF_SLV_R;
///Field `OVR_SLV` reader - Overrun flag of the slave ADC
pub use OVR_MST_R as OVR_SLV_R;
impl R {
    ///Bit 0 - ADDRDY_MST
    #[inline(always)]
    pub fn addrdy_mst(&self) -> ADDRDY_MST_R {
        ADDRDY_MST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EOSMP_MST
    #[inline(always)]
    pub fn eosmp_mst(&self) -> EOSMP_MST_R {
        EOSMP_MST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EOC_MST
    #[inline(always)]
    pub fn eoc_mst(&self) -> EOC_MST_R {
        EOC_MST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOS_MST
    #[inline(always)]
    pub fn eos_mst(&self) -> EOS_MST_R {
        EOS_MST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OVR_MST
    #[inline(always)]
    pub fn ovr_mst(&self) -> OVR_MST_R {
        OVR_MST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - JEOC_MST
    #[inline(always)]
    pub fn jeoc_mst(&self) -> JEOC_MST_R {
        JEOC_MST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - JEOS_MST
    #[inline(always)]
    pub fn jeos_mst(&self) -> JEOS_MST_R {
        JEOS_MST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AWD1_MST
    #[inline(always)]
    pub fn awd1_mst(&self) -> AWD1_MST_R {
        AWD1_MST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AWD2_MST
    #[inline(always)]
    pub fn awd2_mst(&self) -> AWD2_MST_R {
        AWD2_MST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AWD3_MST
    #[inline(always)]
    pub fn awd3_mst(&self) -> AWD3_MST_R {
        AWD3_MST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - JQOVF_MST
    #[inline(always)]
    pub fn jqovf_mst(&self) -> JQOVF_MST_R {
        JQOVF_MST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - ADRDY_SLV
    #[inline(always)]
    pub fn adrdy_slv(&self) -> ADRDY_SLV_R {
        ADRDY_SLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - EOSMP_SLV
    #[inline(always)]
    pub fn eosmp_slv(&self) -> EOSMP_SLV_R {
        EOSMP_SLV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - End of regular conversion of the slave ADC
    #[inline(always)]
    pub fn eoc_slv(&self) -> EOC_SLV_R {
        EOC_SLV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - End of regular sequence flag of the slave ADC
    #[inline(always)]
    pub fn eos_slv(&self) -> EOS_SLV_R {
        EOS_SLV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Overrun flag of the slave ADC
    #[inline(always)]
    pub fn ovr_slv(&self) -> OVR_SLV_R {
        OVR_SLV_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - End of injected conversion flag of the slave ADC
    #[inline(always)]
    pub fn jeoc_slv(&self) -> JEOC_SLV_R {
        JEOC_SLV_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - End of injected sequence flag of the slave ADC
    #[inline(always)]
    pub fn jeos_slv(&self) -> JEOS_SLV_R {
        JEOS_SLV_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog 1 flag of the slave ADC
    #[inline(always)]
    pub fn awd1_slv(&self) -> AWD1_SLV_R {
        AWD1_SLV_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Analog watchdog 2 flag of the slave ADC
    #[inline(always)]
    pub fn awd2_slv(&self) -> AWD2_SLV_R {
        AWD2_SLV_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Analog watchdog 3 flag of the slave ADC
    #[inline(always)]
    pub fn awd3_slv(&self) -> AWD3_SLV_R {
        AWD3_SLV_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Injected Context Queue Overflow flag of the slave ADC
    #[inline(always)]
    pub fn jqovf_slv(&self) -> JQOVF_SLV_R {
        JQOVF_SLV_R::new(((self.bits >> 26) & 1) != 0)
    }
}
///ADC Common status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
