///Register `ICSR` reader
pub struct R(crate::R<ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ICSR` writer
pub struct W(crate::W<ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSR_SPEC>;
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
impl From<crate::W<ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WUTWF` reader - Wakeup timer write flag
pub type WUTWF_R = crate::BitReader<bool>;
///Field `SHPF` reader - Shift operation pending
pub type SHPF_R = crate::BitReader<bool>;
///Field `INITS` reader - Initialization status flag
pub type INITS_R = crate::BitReader<bool>;
///Field `RSF` reader - Registers synchronization flag
pub type RSF_R = crate::BitReader<bool>;
///Field `RSF` writer - Registers synchronization flag
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
///Field `INITF` reader - Initialization flag
pub type INITF_R = crate::BitReader<bool>;
///Field `INIT` reader - Initialization mode
pub type INIT_R = crate::BitReader<bool>;
///Field `INIT` writer - Initialization mode
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
///Field `BIN` reader - BIN
pub type BIN_R = crate::FieldReader<u8, u8>;
///Field `BIN` writer - BIN
pub type BIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSR_SPEC, u8, u8, 2, O>;
///Field `BCDU` reader - BCDU
pub type BCDU_R = crate::FieldReader<u8, u8>;
///Field `BCDU` writer - BCDU
pub type BCDU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSR_SPEC, u8, u8, 3, O>;
///Field `RECALPF` reader - Recalibration pending Flag
pub type RECALPF_R = crate::BitReader<bool>;
impl R {
    ///Bit 2 - Wakeup timer write flag
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shift operation pending
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Initialization status flag
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Initialization flag
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - BIN
    #[inline(always)]
    pub fn bin(&self) -> BIN_R {
        BIN_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:12 - BCDU
    #[inline(always)]
    pub fn bcdu(&self) -> BCDU_R {
        BCDU_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 16 - Recalibration pending Flag
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<5> {
        RSF_W::new(self)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<7> {
        INIT_W::new(self)
    }
    ///Bits 8:9 - BIN
    #[inline(always)]
    #[must_use]
    pub fn bin(&mut self) -> BIN_W<8> {
        BIN_W::new(self)
    }
    ///Bits 10:12 - BCDU
    #[inline(always)]
    #[must_use]
    pub fn bcdu(&mut self) -> BCDU_W<10> {
        BCDU_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC initialization control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icsr](index.html) module
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [icsr::R](R) reader structure
impl crate::Readable for ICSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [icsr::W](W) writer structure
impl crate::Writable for ICSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ICSR to value 0x07
impl crate::Resettable for ICSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
