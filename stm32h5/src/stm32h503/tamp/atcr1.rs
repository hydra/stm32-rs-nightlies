///Register `ATCR1` reader
pub struct R(crate::R<ATCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ATCR1` writer
pub struct W(crate::W<ATCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCR1_SPEC>;
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
impl From<crate::W<ATCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TAMP1AM` reader - Tamper 1 active mode
pub type TAMP1AM_R = crate::BitReader<bool>;
///Field `TAMP1AM` writer - Tamper 1 active mode
pub type TAMP1AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
///Field `TAMP2AM` reader - Tamper 2 active mode
pub type TAMP2AM_R = crate::BitReader<bool>;
///Field `TAMP2AM` writer - Tamper 2 active mode
pub type TAMP2AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
///Field `ATOSEL1` reader - Active tamper shared output 1 selection The selected output must be available in the package pinout
pub type ATOSEL1_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL1` writer - Active tamper shared output 1 selection The selected output must be available in the package pinout
pub type ATOSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
///Field `ATOSEL2` reader - Active tamper shared output 2 selection The selected output must be available in the package pinout
pub type ATOSEL2_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL2` writer - Active tamper shared output 2 selection The selected output must be available in the package pinout
pub type ATOSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
///Field `ATOSEL3` reader - Active tamper shared output 3 selection The selected output must be available in the package pinout
pub type ATOSEL3_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL3` writer - Active tamper shared output 3 selection The selected output must be available in the package pinout
pub type ATOSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
///Field `ATCKSEL` reader - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. f&lt;sub>CK_ATPRE&lt;/sub> = f&lt;sub>RTCCLK&lt;/sub> / 2&lt;sup>ATCKSEL &lt;/sup>when (PREDIV_A+1) = 128. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable.
pub type ATCKSEL_R = crate::FieldReader<u8, u8>;
///Field `ATCKSEL` writer - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. f&lt;sub>CK_ATPRE&lt;/sub> = f&lt;sub>RTCCLK&lt;/sub> / 2&lt;sup>ATCKSEL &lt;/sup>when (PREDIV_A+1) = 128. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable.
pub type ATCKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 3, O>;
///Field `ATPER` reader - Active tamper output change period The tamper output is changed every CK_ATPER = (2&lt;sup>ATPER &lt;/sup>x CK_ATPRE) cycles. Refer to Table 239: Minimum ATPER value.
pub type ATPER_R = crate::FieldReader<u8, u8>;
///Field `ATPER` writer - Active tamper output change period The tamper output is changed every CK_ATPER = (2&lt;sup>ATPER &lt;/sup>x CK_ATPRE) cycles. Refer to Table 239: Minimum ATPER value.
pub type ATPER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 3, O>;
///Field `ATOSHARE` reader - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8
pub type ATOSHARE_R = crate::BitReader<bool>;
///Field `ATOSHARE` writer - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8
pub type ATOSHARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
///Field `FLTEN` reader - Active tamper filter enable
pub type FLTEN_R = crate::BitReader<bool>;
///Field `FLTEN` writer - Active tamper filter enable
pub type FLTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - Tamper 1 active mode
    #[inline(always)]
    pub fn tamp1am(&self) -> TAMP1AM_R {
        TAMP1AM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tamper 2 active mode
    #[inline(always)]
    pub fn tamp2am(&self) -> TAMP2AM_R {
        TAMP2AM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:9 - Active tamper shared output 1 selection The selected output must be available in the package pinout
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Active tamper shared output 2 selection The selected output must be available in the package pinout
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Active tamper shared output 3 selection The selected output must be available in the package pinout
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. f&lt;sub>CK_ATPRE&lt;/sub> = f&lt;sub>RTCCLK&lt;/sub> / 2&lt;sup>ATCKSEL &lt;/sup>when (PREDIV_A+1) = 128. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable.
    #[inline(always)]
    pub fn atcksel(&self) -> ATCKSEL_R {
        ATCKSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 24:26 - Active tamper output change period The tamper output is changed every CK_ATPER = (2&lt;sup>ATPER &lt;/sup>x CK_ATPRE) cycles. Refer to Table 239: Minimum ATPER value.
    #[inline(always)]
    pub fn atper(&self) -> ATPER_R {
        ATPER_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 30 - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8
    #[inline(always)]
    pub fn atoshare(&self) -> ATOSHARE_R {
        ATOSHARE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Active tamper filter enable
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Tamper 1 active mode
    #[inline(always)]
    #[must_use]
    pub fn tamp1am(&mut self) -> TAMP1AM_W<0> {
        TAMP1AM_W::new(self)
    }
    ///Bit 1 - Tamper 2 active mode
    #[inline(always)]
    #[must_use]
    pub fn tamp2am(&mut self) -> TAMP2AM_W<1> {
        TAMP2AM_W::new(self)
    }
    ///Bits 8:9 - Active tamper shared output 1 selection The selected output must be available in the package pinout
    #[inline(always)]
    #[must_use]
    pub fn atosel1(&mut self) -> ATOSEL1_W<8> {
        ATOSEL1_W::new(self)
    }
    ///Bits 10:11 - Active tamper shared output 2 selection The selected output must be available in the package pinout
    #[inline(always)]
    #[must_use]
    pub fn atosel2(&mut self) -> ATOSEL2_W<10> {
        ATOSEL2_W::new(self)
    }
    ///Bits 12:13 - Active tamper shared output 3 selection The selected output must be available in the package pinout
    #[inline(always)]
    #[must_use]
    pub fn atosel3(&mut self) -> ATOSEL3_W<12> {
        ATOSEL3_W::new(self)
    }
    ///Bits 16:18 - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. f&lt;sub>CK_ATPRE&lt;/sub> = f&lt;sub>RTCCLK&lt;/sub> / 2&lt;sup>ATCKSEL &lt;/sup>when (PREDIV_A+1) = 128. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable.
    #[inline(always)]
    #[must_use]
    pub fn atcksel(&mut self) -> ATCKSEL_W<16> {
        ATCKSEL_W::new(self)
    }
    ///Bits 24:26 - Active tamper output change period The tamper output is changed every CK_ATPER = (2&lt;sup>ATPER &lt;/sup>x CK_ATPRE) cycles. Refer to Table 239: Minimum ATPER value.
    #[inline(always)]
    #[must_use]
    pub fn atper(&mut self) -> ATPER_W<24> {
        ATPER_W::new(self)
    }
    ///Bit 30 - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8
    #[inline(always)]
    #[must_use]
    pub fn atoshare(&mut self) -> ATOSHARE_W<30> {
        ATOSHARE_W::new(self)
    }
    ///Bit 31 - Active tamper filter enable
    #[inline(always)]
    #[must_use]
    pub fn flten(&mut self) -> FLTEN_W<31> {
        FLTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP active tamper control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [atcr1](index.html) module
pub struct ATCR1_SPEC;
impl crate::RegisterSpec for ATCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [atcr1::R](R) reader structure
impl crate::Readable for ATCR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [atcr1::W](W) writer structure
impl crate::Writable for ATCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ATCR1 to value 0x0007_0000
impl crate::Resettable for ATCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0000;
}
