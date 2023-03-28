///Register `TIMCISR` reader
pub struct R(crate::R<TIMCISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMCISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMCISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMCISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CMP1` reader - Compare 1 Interrupt Flag
pub type CMP1_R = crate::BitReader<bool>;
///Field `CMP2` reader - Compare 2 Interrupt Flag
pub type CMP2_R = crate::BitReader<bool>;
///Field `CMP3` reader - Compare 3 Interrupt Flag
pub type CMP3_R = crate::BitReader<bool>;
///Field `CMP4` reader - Compare 4 Interrupt Flag
pub type CMP4_R = crate::BitReader<bool>;
///Field `REP` reader - Repetition Interrupt Flag
pub type REP_R = crate::BitReader<bool>;
///Field `UPD` reader - Update Interrupt Flag
pub type UPD_R = crate::BitReader<bool>;
///Field `CPT1` reader - Capture1 Interrupt Flag
pub type CPT1_R = crate::BitReader<bool>;
///Field `CPT2` reader - Capture2 Interrupt Flag
pub type CPT2_R = crate::BitReader<bool>;
///Field `SETx1` reader - Output 1 Set Interrupt Flag
pub type SETX1_R = crate::BitReader<bool>;
///Field `RSTx1` reader - Output 1 Reset Interrupt Flag
pub type RSTX1_R = crate::BitReader<bool>;
///Field `SETx2` reader - Output 2 Set Interrupt Flag
pub type SETX2_R = crate::BitReader<bool>;
///Field `RSTx2` reader - Output 2 Reset Interrupt Flag
pub type RSTX2_R = crate::BitReader<bool>;
///Field `RST` reader - Reset Interrupt Flag
pub type RST_R = crate::BitReader<bool>;
///Field `DLYPRT` reader - Delayed Protection Flag
pub type DLYPRT_R = crate::BitReader<bool>;
///Field `CPPSTAT` reader - Current Push Pull Status
pub type CPPSTAT_R = crate::BitReader<bool>;
///Field `IPPSTAT` reader - Idle Push Pull Status
pub type IPPSTAT_R = crate::BitReader<bool>;
///Field `O1STAT` reader - Output 1 State
pub type O1STAT_R = crate::BitReader<bool>;
///Field `O2STAT` reader - Output 2 State
pub type O2STAT_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Compare 1 Interrupt Flag
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare 2 Interrupt Flag
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare 3 Interrupt Flag
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare 4 Interrupt Flag
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Repetition Interrupt Flag
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Update Interrupt Flag
    #[inline(always)]
    pub fn upd(&self) -> UPD_R {
        UPD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Capture1 Interrupt Flag
    #[inline(always)]
    pub fn cpt1(&self) -> CPT1_R {
        CPT1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Capture2 Interrupt Flag
    #[inline(always)]
    pub fn cpt2(&self) -> CPT2_R {
        CPT2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output 1 Set Interrupt Flag
    #[inline(always)]
    pub fn setx1(&self) -> SETX1_R {
        SETX1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output 1 Reset Interrupt Flag
    #[inline(always)]
    pub fn rstx1(&self) -> RSTX1_R {
        RSTX1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output 2 Set Interrupt Flag
    #[inline(always)]
    pub fn setx2(&self) -> SETX2_R {
        SETX2_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output 2 Reset Interrupt Flag
    #[inline(always)]
    pub fn rstx2(&self) -> RSTX2_R {
        RSTX2_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Reset Interrupt Flag
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Delayed Protection Flag
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Current Push Pull Status
    #[inline(always)]
    pub fn cppstat(&self) -> CPPSTAT_R {
        CPPSTAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Idle Push Pull Status
    #[inline(always)]
    pub fn ippstat(&self) -> IPPSTAT_R {
        IPPSTAT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Output 1 State
    #[inline(always)]
    pub fn o1stat(&self) -> O1STAT_R {
        O1STAT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Output 2 State
    #[inline(always)]
    pub fn o2stat(&self) -> O2STAT_R {
        O2STAT_R::new(((self.bits >> 19) & 1) != 0)
    }
}
///Timerx Interrupt Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timcisr](index.html) module
pub struct TIMCISR_SPEC;
impl crate::RegisterSpec for TIMCISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [timcisr::R](R) reader structure
impl crate::Readable for TIMCISR_SPEC {
    type Reader = R;
}
///`reset()` method sets TIMCISR to value 0
impl crate::Resettable for TIMCISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
