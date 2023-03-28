///Register `GTZC1_MPCBB2_CR` reader
pub struct R(crate::R<GTZC1_MPCBB2_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_MPCBB2_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_MPCBB2_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_MPCBB2_CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GTZC1_MPCBB2_CR` writer
pub struct W(crate::W<GTZC1_MPCBB2_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_MPCBB2_CR_SPEC>;
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
impl From<crate::W<GTZC1_MPCBB2_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_MPCBB2_CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GLOCK` reader - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset.
pub type GLOCK_R = crate::BitReader<bool>;
///Field `GLOCK` writer - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset.
pub type GLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB2_CR_SPEC, bool, O>;
///Field `INVSECSTATE` reader - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not.
pub type INVSECSTATE_R = crate::BitReader<bool>;
///Field `INVSECSTATE` writer - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not.
pub type INVSECSTATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB2_CR_SPEC, bool, O>;
///Field `SRWILADIS` reader - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal).
pub type SRWILADIS_R = crate::BitReader<bool>;
///Field `SRWILADIS` writer - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal).
pub type SRWILADIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB2_CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset.
    #[inline(always)]
    pub fn glock(&self) -> GLOCK_R {
        GLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 30 - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not.
    #[inline(always)]
    pub fn invsecstate(&self) -> INVSECSTATE_R {
        INVSECSTATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal).
    #[inline(always)]
    pub fn srwiladis(&self) -> SRWILADIS_R {
        SRWILADIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset.
    #[inline(always)]
    #[must_use]
    pub fn glock(&mut self) -> GLOCK_W<0> {
        GLOCK_W::new(self)
    }
    ///Bit 30 - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not.
    #[inline(always)]
    #[must_use]
    pub fn invsecstate(&mut self) -> INVSECSTATE_W<30> {
        INVSECSTATE_W::new(self)
    }
    ///Bit 31 - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal).
    #[inline(always)]
    #[must_use]
    pub fn srwiladis(&mut self) -> SRWILADIS_W<31> {
        SRWILADIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GTZC1 SRAM2 MPCBB control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gtzc1_mpcbb2_cr](index.html) module
pub struct GTZC1_MPCBB2_CR_SPEC;
impl crate::RegisterSpec for GTZC1_MPCBB2_CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gtzc1_mpcbb2_cr::R](R) reader structure
impl crate::Readable for GTZC1_MPCBB2_CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gtzc1_mpcbb2_cr::W](W) writer structure
impl crate::Writable for GTZC1_MPCBB2_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GTZC1_MPCBB2_CR to value 0
impl crate::Resettable for GTZC1_MPCBB2_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
