///Register `CCIPR5` reader
pub struct R(crate::R<CCIPR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR5_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCIPR5` writer
pub struct W(crate::W<CCIPR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR5_SPEC>;
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
impl From<crate::W<CCIPR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR5_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADCDACSEL` reader - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled
pub type ADCDACSEL_R = crate::FieldReader<u8, u8>;
///Field `ADCDACSEL` writer - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled
pub type ADCDACSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR5_SPEC, u8, u8, 3, O>;
///Field `DACSEL` reader - DAC hold clock
pub type DACSEL_R = crate::BitReader<bool>;
///Field `DACSEL` writer - DAC hold clock
pub type DACSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR5_SPEC, bool, O>;
///Field `RNGSEL` reader - RNG kernel clock source selection
pub type RNGSEL_R = crate::FieldReader<u8, u8>;
///Field `RNGSEL` writer - RNG kernel clock source selection
pub type RNGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR5_SPEC, u8, u8, 2, O>;
///Field `CECSEL` reader - HSMI-CEC kernel clock source selection
pub type CECSEL_R = crate::FieldReader<u8, u8>;
///Field `CECSEL` writer - HSMI-CEC kernel clock source selection
pub type CECSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR5_SPEC, u8, u8, 2, O>;
///Field `FDCAN12SEL` reader - FDCAN1 and FDCAN2 kernel clock source selection
pub type FDCAN12SEL_R = crate::FieldReader<u8, u8>;
///Field `FDCAN12SEL` writer - FDCAN1 and FDCAN2 kernel clock source selection
pub type FDCAN12SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR5_SPEC, u8, u8, 2, O>;
///Field `SAI1SEL` reader - SAI1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type SAI1SEL_R = crate::FieldReader<u8, u8>;
///Field `SAI1SEL` writer - SAI1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type SAI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR5_SPEC, u8, u8, 3, O>;
///Field `SAI2SEL` reader - SAI2 kernel clock source selection others: reserved, the kernel clock is disabled
pub type SAI2SEL_R = crate::FieldReader<u8, u8>;
///Field `SAI2SEL` writer - SAI2 kernel clock source selection others: reserved, the kernel clock is disabled
pub type SAI2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR5_SPEC, u8, u8, 3, O>;
///Field `CKPERSEL` reader - per_ck clock source selection
pub type CKPERSEL_R = crate::FieldReader<u8, u8>;
///Field `CKPERSEL` writer - per_ck clock source selection
pub type CKPERSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR5_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:2 - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn adcdacsel(&self) -> ADCDACSEL_R {
        ADCDACSEL_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - DAC hold clock
    #[inline(always)]
    pub fn dacsel(&self) -> DACSEL_R {
        DACSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - RNG kernel clock source selection
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - HSMI-CEC kernel clock source selection
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - FDCAN1 and FDCAN2 kernel clock source selection
    #[inline(always)]
    pub fn fdcan12sel(&self) -> FDCAN12SEL_R {
        FDCAN12SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:18 - SAI1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - SAI2 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bits 30:31 - per_ck clock source selection
    #[inline(always)]
    pub fn ckpersel(&self) -> CKPERSEL_R {
        CKPERSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:2 - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn adcdacsel(&mut self) -> ADCDACSEL_W<0> {
        ADCDACSEL_W::new(self)
    }
    ///Bit 3 - DAC hold clock
    #[inline(always)]
    #[must_use]
    pub fn dacsel(&mut self) -> DACSEL_W<3> {
        DACSEL_W::new(self)
    }
    ///Bits 4:5 - RNG kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<4> {
        RNGSEL_W::new(self)
    }
    ///Bits 6:7 - HSMI-CEC kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn cecsel(&mut self) -> CECSEL_W<6> {
        CECSEL_W::new(self)
    }
    ///Bits 8:9 - FDCAN1 and FDCAN2 kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn fdcan12sel(&mut self) -> FDCAN12SEL_W<8> {
        FDCAN12SEL_W::new(self)
    }
    ///Bits 16:18 - SAI1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<16> {
        SAI1SEL_W::new(self)
    }
    ///Bits 19:21 - SAI2 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<19> {
        SAI2SEL_W::new(self)
    }
    ///Bits 30:31 - per_ck clock source selection
    #[inline(always)]
    #[must_use]
    pub fn ckpersel(&mut self) -> CKPERSEL_W<30> {
        CKPERSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC kernel clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccipr5](index.html) module
pub struct CCIPR5_SPEC;
impl crate::RegisterSpec for CCIPR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccipr5::R](R) reader structure
impl crate::Readable for CCIPR5_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccipr5::W](W) writer structure
impl crate::Writable for CCIPR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCIPR5 to value 0
impl crate::Resettable for CCIPR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
