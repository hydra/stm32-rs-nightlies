///Register `TAMP_SMISR` reader
pub struct R(crate::R<TAMP_SMISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_SMISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_SMISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_SMISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TAMP1MF` reader - TAMP1 secure interrupt masked flag This flag is set by hardware when the tamper 1 secure interrupt is raised.
pub type TAMP1MF_R = crate::BitReader<bool>;
///Field `TAMP2MF` reader - TAMP2 secure interrupt masked flag This flag is set by hardware when the tamper 2 secure interrupt is raised.
pub type TAMP2MF_R = crate::BitReader<bool>;
///Field `TAMP3MF` reader - TAMP3 secure interrupt masked flag This flag is set by hardware when the tamper 3 secure interrupt is raised.
pub type TAMP3MF_R = crate::BitReader<bool>;
///Field `TAMP4MF` reader - TAMP4 secure interrupt masked flag This flag is set by hardware when the tamper 4 secure interrupt is raised.
pub type TAMP4MF_R = crate::BitReader<bool>;
///Field `TAMP5MF` reader - TAMP5 secure interrupt masked flag This flag is set by hardware when the tamper 5 secure interrupt is raised.
pub type TAMP5MF_R = crate::BitReader<bool>;
///Field `TAMP6MF` reader - TAMP6 secure interrupt masked flag This flag is set by hardware when the tamper 6 secure interrupt is raised.
pub type TAMP6MF_R = crate::BitReader<bool>;
///Field `TAMP7MF` reader - TAMP7 secure interrupt masked flag This flag is set by hardware when the tamper 7 secure interrupt is raised.
pub type TAMP7MF_R = crate::BitReader<bool>;
///Field `TAMP8MF` reader - TAMP8 secure interrupt masked flag This flag is set by hardware when the tamper 8 secure interrupt is raised.
pub type TAMP8MF_R = crate::BitReader<bool>;
///Field `ITAMP1MF` reader - Internal tamper 1 secure interrupt masked flag This flag is set by hardware when the internal tamper 1 secure interrupt is raised.
pub type ITAMP1MF_R = crate::BitReader<bool>;
///Field `ITAMP2MF` reader - Internal tamper 2 secure interrupt masked flag This flag is set by hardware when the internal tamper 2 secure interrupt is raised.
pub type ITAMP2MF_R = crate::BitReader<bool>;
///Field `ITAMP3MF` reader - Internal tamper 3 secure interrupt masked flag This flag is set by hardware when the internal tamper 3 secure interrupt is raised.
pub type ITAMP3MF_R = crate::BitReader<bool>;
///Field `ITAMP5MF` reader - Internal tamper 5 secure interrupt masked flag This flag is set by hardware when the internal tamper 5 secure interrupt is raised.
pub type ITAMP5MF_R = crate::BitReader<bool>;
///Field `ITAMP6MF` reader - Internal tamper 6 secure interrupt masked flag This flag is set by hardware when the internal tamper 6 secure interrupt is raised.
pub type ITAMP6MF_R = crate::BitReader<bool>;
///Field `ITAMP7MF` reader - VCORE monitoring tamper secure interrupt masked flag This flag is set by hardware when the internal tamper 7 secure interrupt is raised.
pub type ITAMP7MF_R = crate::BitReader<bool>;
///Field `ITAMP8MF` reader - Internal tamper 8 secure interrupt masked flag This flag is set by hardware when the internal tamper 8 secure interrupt is raised.
pub type ITAMP8MF_R = crate::BitReader<bool>;
///Field `ITAMP9MF` reader - internal tamper 9 secure interrupt masked flag This flag is set by hardware when the internal tamper 9 secure interrupt is raised.
pub type ITAMP9MF_R = crate::BitReader<bool>;
///Field `ITAMP11MF` reader - internal tamper 11 secure interrupt masked flag This flag is set by hardware when the internal tamper 11 secure interrupt is raised.
pub type ITAMP11MF_R = crate::BitReader<bool>;
///Field `ITAMP12MF` reader - internal tamper 12 secure interrupt masked flag This flag is set by hardware when the internal tamper 12 secure interrupt is raised.
pub type ITAMP12MF_R = crate::BitReader<bool>;
///Field `ITAMP13MF` reader - internal tamper 13 secure interrupt masked flag This flag is set by hardware when the internal tamper 13 secure interrupt is raised.
pub type ITAMP13MF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TAMP1 secure interrupt masked flag This flag is set by hardware when the tamper 1 secure interrupt is raised.
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2 secure interrupt masked flag This flag is set by hardware when the tamper 2 secure interrupt is raised.
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3 secure interrupt masked flag This flag is set by hardware when the tamper 3 secure interrupt is raised.
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TAMP4 secure interrupt masked flag This flag is set by hardware when the tamper 4 secure interrupt is raised.
    #[inline(always)]
    pub fn tamp4mf(&self) -> TAMP4MF_R {
        TAMP4MF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TAMP5 secure interrupt masked flag This flag is set by hardware when the tamper 5 secure interrupt is raised.
    #[inline(always)]
    pub fn tamp5mf(&self) -> TAMP5MF_R {
        TAMP5MF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TAMP6 secure interrupt masked flag This flag is set by hardware when the tamper 6 secure interrupt is raised.
    #[inline(always)]
    pub fn tamp6mf(&self) -> TAMP6MF_R {
        TAMP6MF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMP7 secure interrupt masked flag This flag is set by hardware when the tamper 7 secure interrupt is raised.
    #[inline(always)]
    pub fn tamp7mf(&self) -> TAMP7MF_R {
        TAMP7MF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TAMP8 secure interrupt masked flag This flag is set by hardware when the tamper 8 secure interrupt is raised.
    #[inline(always)]
    pub fn tamp8mf(&self) -> TAMP8MF_R {
        TAMP8MF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Internal tamper 1 secure interrupt masked flag This flag is set by hardware when the internal tamper 1 secure interrupt is raised.
    #[inline(always)]
    pub fn itamp1mf(&self) -> ITAMP1MF_R {
        ITAMP1MF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Internal tamper 2 secure interrupt masked flag This flag is set by hardware when the internal tamper 2 secure interrupt is raised.
    #[inline(always)]
    pub fn itamp2mf(&self) -> ITAMP2MF_R {
        ITAMP2MF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Internal tamper 3 secure interrupt masked flag This flag is set by hardware when the internal tamper 3 secure interrupt is raised.
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - Internal tamper 5 secure interrupt masked flag This flag is set by hardware when the internal tamper 5 secure interrupt is raised.
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Internal tamper 6 secure interrupt masked flag This flag is set by hardware when the internal tamper 6 secure interrupt is raised.
    #[inline(always)]
    pub fn itamp6mf(&self) -> ITAMP6MF_R {
        ITAMP6MF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - VCORE monitoring tamper secure interrupt masked flag This flag is set by hardware when the internal tamper 7 secure interrupt is raised.
    #[inline(always)]
    pub fn itamp7mf(&self) -> ITAMP7MF_R {
        ITAMP7MF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Internal tamper 8 secure interrupt masked flag This flag is set by hardware when the internal tamper 8 secure interrupt is raised.
    #[inline(always)]
    pub fn itamp8mf(&self) -> ITAMP8MF_R {
        ITAMP8MF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - internal tamper 9 secure interrupt masked flag This flag is set by hardware when the internal tamper 9 secure interrupt is raised.
    #[inline(always)]
    pub fn itamp9mf(&self) -> ITAMP9MF_R {
        ITAMP9MF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - internal tamper 11 secure interrupt masked flag This flag is set by hardware when the internal tamper 11 secure interrupt is raised.
    #[inline(always)]
    pub fn itamp11mf(&self) -> ITAMP11MF_R {
        ITAMP11MF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - internal tamper 12 secure interrupt masked flag This flag is set by hardware when the internal tamper 12 secure interrupt is raised.
    #[inline(always)]
    pub fn itamp12mf(&self) -> ITAMP12MF_R {
        ITAMP12MF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - internal tamper 13 secure interrupt masked flag This flag is set by hardware when the internal tamper 13 secure interrupt is raised.
    #[inline(always)]
    pub fn itamp13mf(&self) -> ITAMP13MF_R {
        ITAMP13MF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
///TAMP secure masked interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tamp_smisr](index.html) module
pub struct TAMP_SMISR_SPEC;
impl crate::RegisterSpec for TAMP_SMISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tamp_smisr::R](R) reader structure
impl crate::Readable for TAMP_SMISR_SPEC {
    type Reader = R;
}
///`reset()` method sets TAMP_SMISR to value 0
impl crate::Resettable for TAMP_SMISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
