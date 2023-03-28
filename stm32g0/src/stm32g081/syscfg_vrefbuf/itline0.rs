///Register `ITLINE0` reader
pub struct R(crate::R<ITLINE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WWDG` reader - Window watchdog interrupt pending flag
pub type WWDG_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Window watchdog interrupt pending flag
    #[inline(always)]
    pub fn wwdg(&self) -> WWDG_R {
        WWDG_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 0 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline0](index.html) module
pub struct ITLINE0_SPEC;
impl crate::RegisterSpec for ITLINE0_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline0::R](R) reader structure
impl crate::Readable for ITLINE0_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE0 to value 0
impl crate::Resettable for ITLINE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
