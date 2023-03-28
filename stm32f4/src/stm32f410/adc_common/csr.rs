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
///Field `AWD1` reader - Analog watchdog flag of ADC 1
pub type AWD1_R = crate::BitReader<AWD1_A>;
///Analog watchdog flag of ADC 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1_A {
    ///0: No analog watchdog event occurred
    NoEvent = 0,
    ///1: Analog watchdog event occurred
    Event = 1,
}
impl From<AWD1_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD1_A {
        match self.bits {
            false => AWD1_A::NoEvent,
            true => AWD1_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1_A::Event
    }
}
///Field `EOC1` reader - End of conversion of ADC 1
pub type EOC1_R = crate::BitReader<EOC1_A>;
///End of conversion of ADC 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOC1_A {
    ///0: Conversion is not complete
    NotComplete = 0,
    ///1: Conversion complete
    Complete = 1,
}
impl From<EOC1_A> for bool {
    #[inline(always)]
    fn from(variant: EOC1_A) -> Self {
        variant as u8 != 0
    }
}
impl EOC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOC1_A {
        match self.bits {
            false => EOC1_A::NotComplete,
            true => EOC1_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC1_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC1_A::Complete
    }
}
///Field `JEOC1` reader - Injected channel end of conversion of ADC 1
pub type JEOC1_R = crate::BitReader<JEOC1_A>;
///Injected channel end of conversion of ADC 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOC1_A {
    ///0: Conversion is not complete
    NotComplete = 0,
    ///1: Conversion complete
    Complete = 1,
}
impl From<JEOC1_A> for bool {
    #[inline(always)]
    fn from(variant: JEOC1_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEOC1_A {
        match self.bits {
            false => JEOC1_A::NotComplete,
            true => JEOC1_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC1_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC1_A::Complete
    }
}
///Field `JSTRT1` reader - Injected channel Start flag of ADC 1
pub type JSTRT1_R = crate::BitReader<JSTRT1_A>;
///Injected channel Start flag of ADC 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSTRT1_A {
    ///0: No injected channel conversion started
    NotStarted = 0,
    ///1: Injected channel conversion has started
    Started = 1,
}
impl From<JSTRT1_A> for bool {
    #[inline(always)]
    fn from(variant: JSTRT1_A) -> Self {
        variant as u8 != 0
    }
}
impl JSTRT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JSTRT1_A {
        match self.bits {
            false => JSTRT1_A::NotStarted,
            true => JSTRT1_A::Started,
        }
    }
    ///Checks if the value of the field is `NotStarted`
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRT1_A::NotStarted
    }
    ///Checks if the value of the field is `Started`
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRT1_A::Started
    }
}
///Field `STRT1` reader - Regular channel Start flag of ADC 1
pub type STRT1_R = crate::BitReader<STRT1_A>;
///Regular channel Start flag of ADC 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRT1_A {
    ///0: No regular channel conversion started
    NotStarted = 0,
    ///1: Regular channel conversion has started
    Started = 1,
}
impl From<STRT1_A> for bool {
    #[inline(always)]
    fn from(variant: STRT1_A) -> Self {
        variant as u8 != 0
    }
}
impl STRT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STRT1_A {
        match self.bits {
            false => STRT1_A::NotStarted,
            true => STRT1_A::Started,
        }
    }
    ///Checks if the value of the field is `NotStarted`
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRT1_A::NotStarted
    }
    ///Checks if the value of the field is `Started`
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRT1_A::Started
    }
}
///Field `OVR1` reader - Overrun flag of ADC 1
pub type OVR1_R = crate::BitReader<OVR1_A>;
///Overrun flag of ADC 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR1_A {
    ///0: No overrun occurred
    NoOverrun = 0,
    ///1: Overrun occurred
    Overrun = 1,
}
impl From<OVR1_A> for bool {
    #[inline(always)]
    fn from(variant: OVR1_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVR1_A {
        match self.bits {
            false => OVR1_A::NoOverrun,
            true => OVR1_A::Overrun,
        }
    }
    ///Checks if the value of the field is `NoOverrun`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR1_A::NoOverrun
    }
    ///Checks if the value of the field is `Overrun`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR1_A::Overrun
    }
}
impl R {
    ///Bit 0 - Analog watchdog flag of ADC 1
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of conversion of ADC 1
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected channel end of conversion of ADC 1
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Injected channel Start flag of ADC 1
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Regular channel Start flag of ADC 1
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overrun flag of ADC 1
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 1) != 0)
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
