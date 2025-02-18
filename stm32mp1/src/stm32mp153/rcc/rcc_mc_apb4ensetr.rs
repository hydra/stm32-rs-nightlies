///Register `RCC_MC_APB4ENSETR` reader
pub struct R(crate::R<RCC_MC_APB4ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_APB4ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_APB4ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_APB4ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MC_APB4ENSETR` writer
pub struct W(crate::W<RCC_MC_APB4ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_APB4ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RCC_MC_APB4ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_APB4ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LTDCEN` reader - LTDCEN
pub type LTDCEN_R = crate::BitReader<bool>;
///Field `LTDCEN` writer - LTDCEN
pub type LTDCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB4ENSETR_SPEC, bool, O>;
///Field `DSIEN` reader - DSIEN
pub type DSIEN_R = crate::BitReader<bool>;
///Field `DSIEN` writer - DSIEN
pub type DSIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB4ENSETR_SPEC, bool, O>;
///Field `DDRPERFMEN` reader - DDRPERFMEN
pub type DDRPERFMEN_R = crate::BitReader<bool>;
///Field `DDRPERFMEN` writer - DDRPERFMEN
pub type DDRPERFMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB4ENSETR_SPEC, bool, O>;
///Field `USBPHYEN` reader - USBPHYEN
pub type USBPHYEN_R = crate::BitReader<bool>;
///Field `USBPHYEN` writer - USBPHYEN
pub type USBPHYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB4ENSETR_SPEC, bool, O>;
///Field `STGENROEN` reader - STGENROEN
pub type STGENROEN_R = crate::BitReader<bool>;
///Field `STGENROEN` writer - STGENROEN
pub type STGENROEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB4ENSETR_SPEC, bool, O>;
impl R {
    ///Bit 0 - LTDCEN
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DSIEN
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - DDRPERFMEN
    #[inline(always)]
    pub fn ddrperfmen(&self) -> DDRPERFMEN_R {
        DDRPERFMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - USBPHYEN
    #[inline(always)]
    pub fn usbphyen(&self) -> USBPHYEN_R {
        USBPHYEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - STGENROEN
    #[inline(always)]
    pub fn stgenroen(&self) -> STGENROEN_R {
        STGENROEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LTDCEN
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<0> {
        LTDCEN_W::new(self)
    }
    ///Bit 4 - DSIEN
    #[inline(always)]
    #[must_use]
    pub fn dsien(&mut self) -> DSIEN_W<4> {
        DSIEN_W::new(self)
    }
    ///Bit 8 - DDRPERFMEN
    #[inline(always)]
    #[must_use]
    pub fn ddrperfmen(&mut self) -> DDRPERFMEN_W<8> {
        DDRPERFMEN_W::new(self)
    }
    ///Bit 16 - USBPHYEN
    #[inline(always)]
    #[must_use]
    pub fn usbphyen(&mut self) -> USBPHYEN_W<16> {
        USBPHYEN_W::new(self)
    }
    ///Bit 20 - STGENROEN
    #[inline(always)]
    #[must_use]
    pub fn stgenroen(&mut self) -> STGENROEN_W<20> {
        STGENROEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to set the peripheral clock enable bit
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mc_apb4ensetr](index.html) module
pub struct RCC_MC_APB4ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_APB4ENSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mc_apb4ensetr::R](R) reader structure
impl crate::Readable for RCC_MC_APB4ENSETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mc_apb4ensetr::W](W) writer structure
impl crate::Writable for RCC_MC_APB4ENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MC_APB4ENSETR to value 0
impl crate::Resettable for RCC_MC_APB4ENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
