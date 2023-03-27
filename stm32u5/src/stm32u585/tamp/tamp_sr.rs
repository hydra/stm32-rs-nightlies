///Register `TAMP_SR` reader
pub struct R(crate::R<TAMP_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TAMP1F` reader - TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP1 input.
pub type TAMP1F_R = crate::BitReader<bool>;
///Field `TAMP2F` reader - TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP2 input.
pub type TAMP2F_R = crate::BitReader<bool>;
///Field `TAMP3F` reader - TAMP3 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP3 input.
pub type TAMP3F_R = crate::BitReader<bool>;
///Field `TAMP4F` reader - TAMP4 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP4 input.
pub type TAMP4F_R = crate::BitReader<bool>;
///Field `TAMP5F` reader - TAMP5 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP5 input.
pub type TAMP5F_R = crate::BitReader<bool>;
///Field `TAMP6F` reader - TAMP6 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP6 input.
pub type TAMP6F_R = crate::BitReader<bool>;
///Field `TAMP7F` reader - TAMP7 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP7 input.
pub type TAMP7F_R = crate::BitReader<bool>;
///Field `TAMP8F` reader - TAMP8 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP8 input
pub type TAMP8F_R = crate::BitReader<bool>;
///Field `ITAMP1F` reader - Internal tamper 1 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 1.
pub type ITAMP1F_R = crate::BitReader<bool>;
///Field `ITAMP2F` reader - Internal tamper 2 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 2.
pub type ITAMP2F_R = crate::BitReader<bool>;
///Field `ITAMP3F` reader - Internal tamper 3 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3.
pub type ITAMP3F_R = crate::BitReader<bool>;
///Field `ITAMP5F` reader - Internal tamper 5 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5.
pub type ITAMP5F_R = crate::BitReader<bool>;
///Field `ITAMP6F` reader - Internal tamper 6 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6.
pub type ITAMP6F_R = crate::BitReader<bool>;
///Field `ITAMP7F` reader - Internal tamper 7 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 7.
pub type ITAMP7F_R = crate::BitReader<bool>;
///Field `ITAMP8F` reader - Internal tamper 8 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 8.
pub type ITAMP8F_R = crate::BitReader<bool>;
///Field `ITAMP9F` reader - Internal tamper 9 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 9.
pub type ITAMP9F_R = crate::BitReader<bool>;
///Field `ITAMP11F` reader - Internal tamper 11 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 11.
pub type ITAMP11F_R = crate::BitReader<bool>;
///Field `ITAMP12F` reader - Internal tamper 12 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 12.
pub type ITAMP12F_R = crate::BitReader<bool>;
///Field `ITAMP13F` reader - Internal tamper 13 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 13.
pub type ITAMP13F_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP1 input.
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP2 input.
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP3 input.
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TAMP4 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP4 input.
    #[inline(always)]
    pub fn tamp4f(&self) -> TAMP4F_R {
        TAMP4F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TAMP5 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP5 input.
    #[inline(always)]
    pub fn tamp5f(&self) -> TAMP5F_R {
        TAMP5F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TAMP6 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP6 input.
    #[inline(always)]
    pub fn tamp6f(&self) -> TAMP6F_R {
        TAMP6F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMP7 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP7 input.
    #[inline(always)]
    pub fn tamp7f(&self) -> TAMP7F_R {
        TAMP7F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TAMP8 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP8 input
    #[inline(always)]
    pub fn tamp8f(&self) -> TAMP8F_R {
        TAMP8F_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Internal tamper 1 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 1.
    #[inline(always)]
    pub fn itamp1f(&self) -> ITAMP1F_R {
        ITAMP1F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Internal tamper 2 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 2.
    #[inline(always)]
    pub fn itamp2f(&self) -> ITAMP2F_R {
        ITAMP2F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Internal tamper 3 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3.
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - Internal tamper 5 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5.
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Internal tamper 6 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6.
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Internal tamper 7 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 7.
    #[inline(always)]
    pub fn itamp7f(&self) -> ITAMP7F_R {
        ITAMP7F_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Internal tamper 8 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 8.
    #[inline(always)]
    pub fn itamp8f(&self) -> ITAMP8F_R {
        ITAMP8F_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Internal tamper 9 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 9.
    #[inline(always)]
    pub fn itamp9f(&self) -> ITAMP9F_R {
        ITAMP9F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Internal tamper 11 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 11.
    #[inline(always)]
    pub fn itamp11f(&self) -> ITAMP11F_R {
        ITAMP11F_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Internal tamper 12 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 12.
    #[inline(always)]
    pub fn itamp12f(&self) -> ITAMP12F_R {
        ITAMP12F_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Internal tamper 13 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 13.
    #[inline(always)]
    pub fn itamp13f(&self) -> ITAMP13F_R {
        ITAMP13F_R::new(((self.bits >> 28) & 1) != 0)
    }
}
///TAMP status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tamp_sr](index.html) module
pub struct TAMP_SR_SPEC;
impl crate::RegisterSpec for TAMP_SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tamp_sr::R](R) reader structure
impl crate::Readable for TAMP_SR_SPEC {
    type Reader = R;
}
///`reset()` method sets TAMP_SR to value 0
impl crate::Resettable for TAMP_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
