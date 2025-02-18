///Register `CFGR1` reader
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR1` writer
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BOOSTEN` reader - BOOSTEN
pub type BOOSTEN_R = crate::BitReader<bool>;
///Field `BOOSTEN` writer - BOOSTEN
pub type BOOSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
///Field `ANASWVDD` reader - GPIO analog switch control voltage selection
pub type ANASWVDD_R = crate::BitReader<bool>;
///Field `ANASWVDD` writer - GPIO analog switch control voltage selection
pub type ANASWVDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
///Field `I2C_PB6_FMP` reader - FM+ drive capability on PB6
pub type I2C_PB6_FMP_R = crate::BitReader<bool>;
///Field `I2C_PB6_FMP` writer - FM+ drive capability on PB6
pub type I2C_PB6_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
///Field `I2C_PB7_FMP` reader - FM+ drive capability on PB6
pub type I2C_PB7_FMP_R = crate::BitReader<bool>;
///Field `I2C_PB7_FMP` writer - FM+ drive capability on PB6
pub type I2C_PB7_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
///Field `I2C_PB8_FMP` reader - FM+ drive capability on PB6
pub type I2C_PB8_FMP_R = crate::BitReader<bool>;
///Field `I2C_PB8_FMP` writer - FM+ drive capability on PB6
pub type I2C_PB8_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
///Field `I2C_PB9_FMP` reader - FM+ drive capability on PB6
pub type I2C_PB9_FMP_R = crate::BitReader<bool>;
///Field `I2C_PB9_FMP` writer - FM+ drive capability on PB6
pub type I2C_PB9_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
///Field `I2C1_FMP` reader - I2C1 FM+ drive capability enable
pub type I2C1_FMP_R = crate::BitReader<bool>;
///Field `I2C1_FMP` writer - I2C1 FM+ drive capability enable
pub type I2C1_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
///Field `I2C2_FMP` reader - I2C1 FM+ drive capability enable
pub type I2C2_FMP_R = crate::BitReader<bool>;
///Field `I2C2_FMP` writer - I2C1 FM+ drive capability enable
pub type I2C2_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
///Field `I2C3_FMP` reader - I2C1 FM+ drive capability enable
pub type I2C3_FMP_R = crate::BitReader<bool>;
///Field `I2C3_FMP` writer - I2C1 FM+ drive capability enable
pub type I2C3_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
///Field `I2C4_FMP` reader - I2C1 FM+ drive capability enable
pub type I2C4_FMP_R = crate::BitReader<bool>;
///Field `I2C4_FMP` writer - I2C1 FM+ drive capability enable
pub type I2C4_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
///Field `FPU_IE` reader - FPU Interrupts Enable
pub type FPU_IE_R = crate::FieldReader<u8, u8>;
///Field `FPU_IE` writer - FPU Interrupts Enable
pub type FPU_IE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 6, O>;
impl R {
    ///Bit 8 - BOOSTEN
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIO analog switch control voltage selection
    #[inline(always)]
    pub fn anaswvdd(&self) -> ANASWVDD_R {
        ANASWVDD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2C4_FMP_R {
        I2C4_FMP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 26:31 - FPU Interrupts Enable
    #[inline(always)]
    pub fn fpu_ie(&self) -> FPU_IE_R {
        FPU_IE_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    ///Bit 8 - BOOSTEN
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<8> {
        BOOSTEN_W::new(self)
    }
    ///Bit 9 - GPIO analog switch control voltage selection
    #[inline(always)]
    #[must_use]
    pub fn anaswvdd(&mut self) -> ANASWVDD_W<9> {
        ANASWVDD_W::new(self)
    }
    ///Bit 16 - FM+ drive capability on PB6
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<16> {
        I2C_PB6_FMP_W::new(self)
    }
    ///Bit 17 - FM+ drive capability on PB6
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<17> {
        I2C_PB7_FMP_W::new(self)
    }
    ///Bit 18 - FM+ drive capability on PB6
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<18> {
        I2C_PB8_FMP_W::new(self)
    }
    ///Bit 19 - FM+ drive capability on PB6
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<19> {
        I2C_PB9_FMP_W::new(self)
    }
    ///Bit 20 - I2C1 FM+ drive capability enable
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<20> {
        I2C1_FMP_W::new(self)
    }
    ///Bit 21 - I2C1 FM+ drive capability enable
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<21> {
        I2C2_FMP_W::new(self)
    }
    ///Bit 22 - I2C1 FM+ drive capability enable
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<22> {
        I2C3_FMP_W::new(self)
    }
    ///Bit 23 - I2C1 FM+ drive capability enable
    #[inline(always)]
    #[must_use]
    pub fn i2c4_fmp(&mut self) -> I2C4_FMP_W<23> {
        I2C4_FMP_W::new(self)
    }
    ///Bits 26:31 - FPU Interrupts Enable
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie(&mut self) -> FPU_IE_W<26> {
        FPU_IE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///peripheral mode configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr1](index.html) module
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr1::R](R) reader structure
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr1::W](W) writer structure
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR1 to value 0x7c00_0001
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x7c00_0001;
}
