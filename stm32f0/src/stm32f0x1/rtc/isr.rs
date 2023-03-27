///Register `ISR` reader
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ISR` writer
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ALRAWF` reader - Alarm A write flag
pub type ALRAWF_R = crate::BitReader<bool>;
///Field `SHPF` reader - Shift operation pending
pub type SHPF_R = crate::BitReader<bool>;
///Field `SHPF` writer - Shift operation pending
pub type SHPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
///Field `INITS` reader - Initialization status flag
pub type INITS_R = crate::BitReader<bool>;
///Field `RSF` reader - Registers synchronization flag
pub type RSF_R = crate::BitReader<bool>;
///Field `RSF` writer - Registers synchronization flag
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
///Field `INITF` reader - Initialization flag
pub type INITF_R = crate::BitReader<bool>;
///Field `INIT` reader - Initialization mode
pub type INIT_R = crate::BitReader<bool>;
///Field `INIT` writer - Initialization mode
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
///Field `ALRAF` reader - Alarm A flag
pub type ALRAF_R = crate::BitReader<bool>;
///Field `ALRAF` writer - Alarm A flag
pub type ALRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
///Field `TSF` reader - Time-stamp flag
pub type TSF_R = crate::BitReader<bool>;
///Field `TSF` writer - Time-stamp flag
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
///Field `TSOVF` reader - Time-stamp overflow flag
pub type TSOVF_R = crate::BitReader<bool>;
///Field `TSOVF` writer - Time-stamp overflow flag
pub type TSOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
///Field `TAMP1F` reader - RTC_TAMP1 detection flag
pub type TAMP1F_R = crate::BitReader<bool>;
///Field `TAMP1F` writer - RTC_TAMP1 detection flag
pub type TAMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
///Field `TAMP2F` reader - RTC_TAMP2 detection flag
pub type TAMP2F_R = crate::BitReader<bool>;
///Field `TAMP2F` writer - RTC_TAMP2 detection flag
pub type TAMP2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
///Field `RECALPF` reader - Recalibration pending Flag
pub type RECALPF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Alarm A write flag
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 1) != 0)
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
    ///Bit 8 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Time-stamp flag
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Time-stamp overflow flag
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RTC_TAMP1 detection flag
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RTC_TAMP2 detection flag
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Recalibration pending Flag
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - Shift operation pending
    #[inline(always)]
    #[must_use]
    pub fn shpf(&mut self) -> SHPF_W<3> {
        SHPF_W::new(self)
    }
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
    ///Bit 8 - Alarm A flag
    #[inline(always)]
    #[must_use]
    pub fn alraf(&mut self) -> ALRAF_W<8> {
        ALRAF_W::new(self)
    }
    ///Bit 11 - Time-stamp flag
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<11> {
        TSF_W::new(self)
    }
    ///Bit 12 - Time-stamp overflow flag
    #[inline(always)]
    #[must_use]
    pub fn tsovf(&mut self) -> TSOVF_W<12> {
        TSOVF_W::new(self)
    }
    ///Bit 13 - RTC_TAMP1 detection flag
    #[inline(always)]
    #[must_use]
    pub fn tamp1f(&mut self) -> TAMP1F_W<13> {
        TAMP1F_W::new(self)
    }
    ///Bit 14 - RTC_TAMP2 detection flag
    #[inline(always)]
    #[must_use]
    pub fn tamp2f(&mut self) -> TAMP2F_W<14> {
        TAMP2F_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///initialization and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](index.html) module
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [isr::R](R) reader structure
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [isr::W](W) writer structure
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ISR to value 0x07
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
