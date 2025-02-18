///Register `C1TOC2SR` reader
pub struct R(crate::R<C1TOC2SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1TOC2SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1TOC2SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1TOC2SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CH1F` reader - CH1F
pub type CH1F_R = crate::BitReader<CH1F_A>;
///CH1F
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1F_A {
    ///0: Channel free, data can be written by the sending processor. Generates a channel TX free interrupt to the current processor, when unmasked
    Free = 0,
    ///1: Channel occupied, data can be read by the receiving processor. Generates a channel RX occupied interrupt to the other processor, when unmasked
    Occupied = 1,
}
impl From<CH1F_A> for bool {
    #[inline(always)]
    fn from(variant: CH1F_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CH1F_A {
        match self.bits {
            false => CH1F_A::Free,
            true => CH1F_A::Occupied,
        }
    }
    ///Checks if the value of the field is `Free`
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == CH1F_A::Free
    }
    ///Checks if the value of the field is `Occupied`
    #[inline(always)]
    pub fn is_occupied(&self) -> bool {
        *self == CH1F_A::Occupied
    }
}
///Field `CH2F` reader - CH2F
pub use CH1F_R as CH2F_R;
///Field `CH3F` reader - CH3F
pub use CH1F_R as CH3F_R;
///Field `CH4F` reader - CH4F
pub use CH1F_R as CH4F_R;
///Field `CH5F` reader - CH5F
pub use CH1F_R as CH5F_R;
///Field `CH6F` reader - CH6F
pub type CH6F_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - CH1F
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CH2F
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CH3F
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CH4F
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CH5F
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CH6F
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 5) & 1) != 0)
    }
}
///IPCC processor 1 to processor 2 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1toc2sr](index.html) module
pub struct C1TOC2SR_SPEC;
impl crate::RegisterSpec for C1TOC2SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1toc2sr::R](R) reader structure
impl crate::Readable for C1TOC2SR_SPEC {
    type Reader = R;
}
///`reset()` method sets C1TOC2SR to value 0
impl crate::Resettable for C1TOC2SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
