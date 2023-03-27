///Register `RCC_ICSCR2` reader
pub struct R(crate::R<RCC_ICSCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_ICSCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_ICSCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_ICSCR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_ICSCR2` writer
pub struct W(crate::W<RCC_ICSCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_ICSCR2_SPEC>;
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
impl From<crate::W<RCC_ICSCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_ICSCR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MSITRIM3` reader - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\[4:0\]
///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM3_R = crate::FieldReader<u8, u8>;
///Field `MSITRIM3` writer - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\[4:0\]
///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_ICSCR2_SPEC, u8, u8, 5, O>;
///Field `MSITRIM2` reader - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\[4:0\]
///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM2_R = crate::FieldReader<u8, u8>;
///Field `MSITRIM2` writer - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\[4:0\]
///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_ICSCR2_SPEC, u8, u8, 5, O>;
///Field `MSITRIM1` reader - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\[4:0\]
///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM1_R = crate::FieldReader<u8, u8>;
///Field `MSITRIM1` writer - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\[4:0\]
///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_ICSCR2_SPEC, u8, u8, 5, O>;
///Field `MSITRIM0` reader - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\[4:0\]
///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM0_R = crate::FieldReader<u8, u8>;
///Field `MSITRIM0` writer - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\[4:0\]
///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_ICSCR2_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\[4:0\]
    ///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim3(&self) -> MSITRIM3_R {
        MSITRIM3_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\[4:0\]
    ///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim2(&self) -> MSITRIM2_R {
        MSITRIM2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\[4:0\]
    ///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim1(&self) -> MSITRIM1_R {
        MSITRIM1_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\[4:0\]
    ///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim0(&self) -> MSITRIM0_R {
        MSITRIM0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\[4:0\]
    ///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    #[must_use]
    pub fn msitrim3(&mut self) -> MSITRIM3_W<0> {
        MSITRIM3_W::new(self)
    }
    ///Bits 5:9 - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\[4:0\]
    ///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    #[must_use]
    pub fn msitrim2(&mut self) -> MSITRIM2_W<5> {
        MSITRIM2_W::new(self)
    }
    ///Bits 10:14 - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\[4:0\]
    ///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    #[must_use]
    pub fn msitrim1(&mut self) -> MSITRIM1_W<10> {
        MSITRIM1_W::new(self)
    }
    ///Bits 15:19 - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\[4:0\]
    ///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    #[must_use]
    pub fn msitrim0(&mut self) -> MSITRIM0_W<15> {
        MSITRIM0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC internal clock sources calibration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_icscr2](index.html) module
pub struct RCC_ICSCR2_SPEC;
impl crate::RegisterSpec for RCC_ICSCR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_icscr2::R](R) reader structure
impl crate::Readable for RCC_ICSCR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_icscr2::W](W) writer structure
impl crate::Writable for RCC_ICSCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_ICSCR2 to value 0x0008_4210
impl crate::Resettable for RCC_ICSCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_4210;
}
