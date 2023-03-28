///Register `SCSR` reader
pub struct R(crate::R<SCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SCSR` writer
pub struct W(crate::W<SCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCSR_SPEC>;
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
impl From<crate::W<SCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCMER` reader - CCM SRAM Erase
pub type CCMER_R = crate::BitReader<bool>;
///Field `CCMER` writer - CCM SRAM Erase
pub type CCMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCSR_SPEC, bool, O>;
///Field `CCMBSY` reader - CCM SRAM busy by erase operation
pub type CCMBSY_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - CCM SRAM Erase
    #[inline(always)]
    pub fn ccmer(&self) -> CCMER_R {
        CCMER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CCM SRAM busy by erase operation
    #[inline(always)]
    pub fn ccmbsy(&self) -> CCMBSY_R {
        CCMBSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CCM SRAM Erase
    #[inline(always)]
    #[must_use]
    pub fn ccmer(&mut self) -> CCMER_W<0> {
        CCMER_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CCM SRAM control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [scsr](index.html) module
pub struct SCSR_SPEC;
impl crate::RegisterSpec for SCSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [scsr::R](R) reader structure
impl crate::Readable for SCSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [scsr::W](W) writer structure
impl crate::Writable for SCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SCSR to value 0
impl crate::Resettable for SCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
