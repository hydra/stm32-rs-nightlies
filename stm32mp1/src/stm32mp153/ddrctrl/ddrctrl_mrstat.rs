///Register `DDRCTRL_MRSTAT` reader
pub struct R(crate::R<DDRCTRL_MRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_MRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_MRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_MRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MR_WR_BUSY` reader - MR_WR_BUSY
pub type MR_WR_BUSY_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - MR_WR_BUSY
    #[inline(always)]
    pub fn mr_wr_busy(&self) -> MR_WR_BUSY_R {
        MR_WR_BUSY_R::new((self.bits & 1) != 0)
    }
}
///DDRCTRL mode register read/write status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_mrstat](index.html) module
pub struct DDRCTRL_MRSTAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_MRSTAT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_mrstat::R](R) reader structure
impl crate::Readable for DDRCTRL_MRSTAT_SPEC {
    type Reader = R;
}
///`reset()` method sets DDRCTRL_MRSTAT to value 0
impl crate::Resettable for DDRCTRL_MRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
