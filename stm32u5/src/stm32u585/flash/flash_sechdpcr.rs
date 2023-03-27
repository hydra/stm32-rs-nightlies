///Register `FLASH_SECHDPCR` reader
pub struct R(crate::R<FLASH_SECHDPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SECHDPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SECHDPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SECHDPCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_SECHDPCR` writer
pub struct W(crate::W<FLASH_SECHDPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SECHDPCR_SPEC>;
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
impl From<crate::W<FLASH_SECHDPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SECHDPCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HDP1_ACCDIS` reader - HDP1 area access disable When set, this bit is only cleared by a system reset.
pub type HDP1_ACCDIS_R = crate::BitReader<bool>;
///Field `HDP1_ACCDIS` writer - HDP1 area access disable When set, this bit is only cleared by a system reset.
pub type HDP1_ACCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECHDPCR_SPEC, bool, O>;
///Field `HDP2_ACCDIS` reader - HDP2 area access disable When set, this bit is only cleared by a system reset.
pub type HDP2_ACCDIS_R = crate::BitReader<bool>;
///Field `HDP2_ACCDIS` writer - HDP2 area access disable When set, this bit is only cleared by a system reset.
pub type HDP2_ACCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECHDPCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - HDP1 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    pub fn hdp1_accdis(&self) -> HDP1_ACCDIS_R {
        HDP1_ACCDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HDP2 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    pub fn hdp2_accdis(&self) -> HDP2_ACCDIS_R {
        HDP2_ACCDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - HDP1 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    #[must_use]
    pub fn hdp1_accdis(&mut self) -> HDP1_ACCDIS_W<0> {
        HDP1_ACCDIS_W::new(self)
    }
    ///Bit 1 - HDP2 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    #[must_use]
    pub fn hdp2_accdis(&mut self) -> HDP2_ACCDIS_W<1> {
        HDP2_ACCDIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH secure HDP control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_sechdpcr](index.html) module
pub struct FLASH_SECHDPCR_SPEC;
impl crate::RegisterSpec for FLASH_SECHDPCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_sechdpcr::R](R) reader structure
impl crate::Readable for FLASH_SECHDPCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_sechdpcr::W](W) writer structure
impl crate::Writable for FLASH_SECHDPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_SECHDPCR to value 0
impl crate::Resettable for FLASH_SECHDPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
