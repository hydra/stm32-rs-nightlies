///Register `RCC_ICSCR3` reader
pub struct R(crate::R<RCC_ICSCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_ICSCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_ICSCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_ICSCR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_ICSCR3` writer
pub struct W(crate::W<RCC_ICSCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_ICSCR3_SPEC>;
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
impl From<crate::W<RCC_ICSCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_ICSCR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HSICAL` reader - HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value.
pub type HSICAL_R = crate::FieldReader<u16, u16>;
///Field `HSITRIM` reader - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\[11:0\]
///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI.
pub type HSITRIM_R = crate::FieldReader<u8, u8>;
///Field `HSITRIM` writer - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\[11:0\]
///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI.
pub type HSITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_ICSCR3_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:11 - HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value.
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:20 - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\[11:0\]
    ///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI.
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 16:20 - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\[11:0\]
    ///bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI.
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<16> {
        HSITRIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC internal clock sources calibration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_icscr3](index.html) module
pub struct RCC_ICSCR3_SPEC;
impl crate::RegisterSpec for RCC_ICSCR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_icscr3::R](R) reader structure
impl crate::Readable for RCC_ICSCR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_icscr3::W](W) writer structure
impl crate::Writable for RCC_ICSCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_ICSCR3 to value 0x0010_0000
impl crate::Resettable for RCC_ICSCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0000;
}
