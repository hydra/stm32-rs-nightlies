///Register `PWR_BDCR2` reader
pub struct R(crate::R<PWR_BDCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_BDCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_BDCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_BDCR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_BDCR2` writer
pub struct W(crate::W<PWR_BDCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_BDCR2_SPEC>;
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
impl From<crate::W<PWR_BDCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_BDCR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VBE` reader - VBAT charging enable
pub type VBE_R = crate::BitReader<bool>;
///Field `VBE` writer - VBAT charging enable
pub type VBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_BDCR2_SPEC, bool, O>;
///Field `VBRS` reader - VBAT charging resistor selection
pub type VBRS_R = crate::BitReader<bool>;
///Field `VBRS` writer - VBAT charging resistor selection
pub type VBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_BDCR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - VBAT charging enable
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VBAT charging resistor selection
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VBAT charging enable
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<0> {
        VBE_W::new(self)
    }
    ///Bit 1 - VBAT charging resistor selection
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<1> {
        VBRS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR Backup domain control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_bdcr2](index.html) module
pub struct PWR_BDCR2_SPEC;
impl crate::RegisterSpec for PWR_BDCR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_bdcr2::R](R) reader structure
impl crate::Readable for PWR_BDCR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_bdcr2::W](W) writer structure
impl crate::Writable for PWR_BDCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_BDCR2 to value 0
impl crate::Resettable for PWR_BDCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
