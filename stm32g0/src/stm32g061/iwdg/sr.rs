///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PVU` reader - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Prescaler value can be updated only when PVU bit is reset.
pub type PVU_R = crate::BitReader<bool>;
///Field `RVU` reader - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Reload value can be updated only when RVU bit is reset.
pub type RVU_R = crate::BitReader<bool>;
///Field `WVU` reader - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Window value can be updated only when WVU bit is reset.
pub type WVU_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Prescaler value can be updated only when PVU bit is reset.
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Reload value can be updated only when RVU bit is reset.
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Window value can be updated only when WVU bit is reset.
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
///Status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
