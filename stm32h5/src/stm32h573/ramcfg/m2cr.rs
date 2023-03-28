///Register `M2CR` reader
pub struct R(crate::R<M2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M2CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `M2CR` writer
pub struct W(crate::W<M2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M2CR_SPEC>;
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
impl From<crate::W<M2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M2CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ECCE` reader - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
pub type ECCE_R = crate::BitReader<bool>;
///Field `ECCE` writer - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
pub type ECCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M2CR_SPEC, bool, O>;
///Field `ALE` reader - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
pub type ALE_R = crate::BitReader<bool>;
///Field `ALE` writer - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
pub type ALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M2CR_SPEC, bool, O>;
///Field `SRAMER` reader - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.
pub type SRAMER_R = crate::BitReader<bool>;
///Field `SRAMER` writer - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.
pub type SRAMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, M2CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
    #[inline(always)]
    pub fn ecce(&self) -> ECCE_R {
        ECCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
    #[inline(always)]
    #[must_use]
    pub fn ecce(&mut self) -> ECCE_W<0> {
        ECCE_W::new(self)
    }
    ///Bit 4 - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
    #[inline(always)]
    #[must_use]
    pub fn ale(&mut self) -> ALE_W<4> {
        ALE_W::new(self)
    }
    ///Bit 8 - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.
    #[inline(always)]
    #[must_use]
    pub fn sramer(&mut self) -> SRAMER_W<8> {
        SRAMER_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RAMCFG memory 2 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m2cr](index.html) module
pub struct M2CR_SPEC;
impl crate::RegisterSpec for M2CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m2cr::R](R) reader structure
impl crate::Readable for M2CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [m2cr::W](W) writer structure
impl crate::Writable for M2CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets M2CR to value 0
impl crate::Resettable for M2CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
