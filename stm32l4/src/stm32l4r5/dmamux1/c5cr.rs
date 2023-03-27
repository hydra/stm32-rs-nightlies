///Register `C5CR` reader
pub struct R(crate::R<C5CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C5CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C5CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C5CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C5CR` writer
pub struct W(crate::W<C5CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C5CR_SPEC>;
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
impl From<crate::W<C5CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C5CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMAREQ_ID` reader - DMA request identification
pub type DMAREQ_ID_R = crate::FieldReader<u8, u8>;
///Field `DMAREQ_ID` writer - DMA request identification
pub type DMAREQ_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C5CR_SPEC, u8, u8, 7, O>;
///Field `SOIE` reader - Synchronization overrun interrupt enable
pub type SOIE_R = crate::BitReader<bool>;
///Field `SOIE` writer - Synchronization overrun interrupt enable
pub type SOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C5CR_SPEC, bool, O>;
///Field `EGE` reader - Event generation enable
pub type EGE_R = crate::BitReader<bool>;
///Field `EGE` writer - Event generation enable
pub type EGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C5CR_SPEC, bool, O>;
///Field `SE` reader - Synchronization enable
pub type SE_R = crate::BitReader<bool>;
///Field `SE` writer - Synchronization enable
pub type SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C5CR_SPEC, bool, O>;
///Field `SPOL` reader - Synchronization polarity
pub type SPOL_R = crate::FieldReader<u8, u8>;
///Field `SPOL` writer - Synchronization polarity
pub type SPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C5CR_SPEC, u8, u8, 2, O>;
///Field `NBREQ` reader - Number of DMA requests minus 1 to forward
pub type NBREQ_R = crate::FieldReader<u8, u8>;
///Field `NBREQ` writer - Number of DMA requests minus 1 to forward
pub type NBREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C5CR_SPEC, u8, u8, 5, O>;
///Field `SYNC_ID` reader - Synchronization identification
pub type SYNC_ID_R = crate::FieldReader<u8, u8>;
///Field `SYNC_ID` writer - Synchronization identification
pub type SYNC_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C5CR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:6 - DMA request identification
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Synchronization polarity
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - Synchronization identification
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:6 - DMA request identification
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<0> {
        DMAREQ_ID_W::new(self)
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SOIE_W<8> {
        SOIE_W::new(self)
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EGE_W<9> {
        EGE_W::new(self)
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<16> {
        SE_W::new(self)
    }
    ///Bits 17:18 - Synchronization polarity
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<17> {
        SPOL_W::new(self)
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NBREQ_W<19> {
        NBREQ_W::new(self)
    }
    ///Bits 24:28 - Synchronization identification
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SYNC_ID_W<24> {
        SYNC_ID_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///channel 5 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c5cr](index.html) module
pub struct C5CR_SPEC;
impl crate::RegisterSpec for C5CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c5cr::R](R) reader structure
impl crate::Readable for C5CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c5cr::W](W) writer structure
impl crate::Writable for C5CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C5CR to value 0
impl crate::Resettable for C5CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
