///Register `ADF_SADCR` reader
pub struct R(crate::R<ADF_SADCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_SADCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_SADCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_SADCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADF_SADCR` writer
pub struct W(crate::W<ADF_SADCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADF_SADCR_SPEC>;
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
impl From<crate::W<ADF_SADCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADF_SADCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SADEN` reader - Sound activity detector enable
pub type SADEN_R = crate::BitReader<bool>;
///Field `SADEN` writer - Sound activity detector enable
pub type SADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_SADCR_SPEC, bool, O>;
///Field `DATCAP` reader - Data capture mode
pub type DATCAP_R = crate::FieldReader<u8, u8>;
///Field `DATCAP` writer - Data capture mode
pub type DATCAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_SADCR_SPEC, u8, u8, 2, O>;
///Field `DETCFG` reader - Sound trigger event configuration
pub type DETCFG_R = crate::BitReader<bool>;
///Field `DETCFG` writer - Sound trigger event configuration
pub type DETCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_SADCR_SPEC, bool, O>;
///Field `SADST` reader - SAD state
pub type SADST_R = crate::FieldReader<u8, u8>;
///Field `HYSTEN` reader - Hysteresis enable
pub type HYSTEN_R = crate::BitReader<bool>;
///Field `HYSTEN` writer - Hysteresis enable
pub type HYSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_SADCR_SPEC, bool, O>;
///Field `FRSIZE` reader - Frame size
pub type FRSIZE_R = crate::FieldReader<u8, u8>;
///Field `FRSIZE` writer - Frame size
pub type FRSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_SADCR_SPEC, u8, u8, 3, O>;
///Field `SADMOD` reader - SAD working mode
pub type SADMOD_R = crate::FieldReader<u8, u8>;
///Field `SADMOD` writer - SAD working mode
pub type SADMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_SADCR_SPEC, u8, u8, 2, O>;
///Field `SADACTIVE` reader - SAD Active flag
pub type SADACTIVE_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Sound activity detector enable
    #[inline(always)]
    pub fn saden(&self) -> SADEN_R {
        SADEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data capture mode
    #[inline(always)]
    pub fn datcap(&self) -> DATCAP_R {
        DATCAP_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - Sound trigger event configuration
    #[inline(always)]
    pub fn detcfg(&self) -> DETCFG_R {
        DETCFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - SAD state
    #[inline(always)]
    pub fn sadst(&self) -> SADST_R {
        SADST_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - Hysteresis enable
    #[inline(always)]
    pub fn hysten(&self) -> HYSTEN_R {
        HYSTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Frame size
    #[inline(always)]
    pub fn frsize(&self) -> FRSIZE_R {
        FRSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:13 - SAD working mode
    #[inline(always)]
    pub fn sadmod(&self) -> SADMOD_R {
        SADMOD_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 31 - SAD Active flag
    #[inline(always)]
    pub fn sadactive(&self) -> SADACTIVE_R {
        SADACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Sound activity detector enable
    #[inline(always)]
    #[must_use]
    pub fn saden(&mut self) -> SADEN_W<0> {
        SADEN_W::new(self)
    }
    ///Bits 1:2 - Data capture mode
    #[inline(always)]
    #[must_use]
    pub fn datcap(&mut self) -> DATCAP_W<1> {
        DATCAP_W::new(self)
    }
    ///Bit 3 - Sound trigger event configuration
    #[inline(always)]
    #[must_use]
    pub fn detcfg(&mut self) -> DETCFG_W<3> {
        DETCFG_W::new(self)
    }
    ///Bit 7 - Hysteresis enable
    #[inline(always)]
    #[must_use]
    pub fn hysten(&mut self) -> HYSTEN_W<7> {
        HYSTEN_W::new(self)
    }
    ///Bits 8:10 - Frame size
    #[inline(always)]
    #[must_use]
    pub fn frsize(&mut self) -> FRSIZE_W<8> {
        FRSIZE_W::new(self)
    }
    ///Bits 12:13 - SAD working mode
    #[inline(always)]
    #[must_use]
    pub fn sadmod(&mut self) -> SADMOD_W<12> {
        SADMOD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADF SAD control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_sadcr](index.html) module
pub struct ADF_SADCR_SPEC;
impl crate::RegisterSpec for ADF_SADCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_sadcr::R](R) reader structure
impl crate::Readable for ADF_SADCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adf_sadcr::W](W) writer structure
impl crate::Writable for ADF_SADCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADF_SADCR to value 0
impl crate::Resettable for ADF_SADCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
