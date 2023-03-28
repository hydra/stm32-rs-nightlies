///Register `CCSWCR` reader
pub struct R(crate::R<CCSWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCSWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCSWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCSWCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCSWCR` writer
pub struct W(crate::W<CCSWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCSWCR_SPEC>;
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
impl From<crate::W<CCSWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCSWCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SW_ANSRC1` reader - NMOS compensation code for VDD power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR.
pub type SW_ANSRC1_R = crate::FieldReader<u8, u8>;
///Field `SW_ANSRC1` writer - NMOS compensation code for VDD power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR.
pub type SW_ANSRC1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCSWCR_SPEC, u8, u8, 4, O>;
///Field `SW_APSRC1` reader - PMOS compensation code for the VDD power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR.
pub type SW_APSRC1_R = crate::FieldReader<u8, u8>;
///Field `SW_APSRC1` writer - PMOS compensation code for the VDD power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR.
pub type SW_APSRC1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCSWCR_SPEC, u8, u8, 4, O>;
///Field `SW_ANSRC2` reader - NMOS compensation code for VDDIO power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR.
pub type SW_ANSRC2_R = crate::FieldReader<u8, u8>;
///Field `SW_ANSRC2` writer - NMOS compensation code for VDDIO power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR.
pub type SW_ANSRC2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCSWCR_SPEC, u8, u8, 4, O>;
///Field `SW_APSRC2` reader - PMOS compensation code for the V&lt;sub>DDIO&lt;/sub> power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR.
pub type SW_APSRC2_R = crate::FieldReader<u8, u8>;
///Field `SW_APSRC2` writer - PMOS compensation code for the V&lt;sub>DDIO&lt;/sub> power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR.
pub type SW_APSRC2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCSWCR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - NMOS compensation code for VDD power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR.
    #[inline(always)]
    pub fn sw_ansrc1(&self) -> SW_ANSRC1_R {
        SW_ANSRC1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PMOS compensation code for the VDD power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR.
    #[inline(always)]
    pub fn sw_apsrc1(&self) -> SW_APSRC1_R {
        SW_APSRC1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - NMOS compensation code for VDDIO power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR.
    #[inline(always)]
    pub fn sw_ansrc2(&self) -> SW_ANSRC2_R {
        SW_ANSRC2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - PMOS compensation code for the V&lt;sub>DDIO&lt;/sub> power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR.
    #[inline(always)]
    pub fn sw_apsrc2(&self) -> SW_APSRC2_R {
        SW_APSRC2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - NMOS compensation code for VDD power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR.
    #[inline(always)]
    #[must_use]
    pub fn sw_ansrc1(&mut self) -> SW_ANSRC1_W<0> {
        SW_ANSRC1_W::new(self)
    }
    ///Bits 4:7 - PMOS compensation code for the VDD power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR.
    #[inline(always)]
    #[must_use]
    pub fn sw_apsrc1(&mut self) -> SW_APSRC1_W<4> {
        SW_APSRC1_W::new(self)
    }
    ///Bits 8:11 - NMOS compensation code for VDDIO power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR.
    #[inline(always)]
    #[must_use]
    pub fn sw_ansrc2(&mut self) -> SW_ANSRC2_W<8> {
        SW_ANSRC2_W::new(self)
    }
    ///Bits 12:15 - PMOS compensation code for the V&lt;sub>DDIO&lt;/sub> power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR.
    #[inline(always)]
    #[must_use]
    pub fn sw_apsrc2(&mut self) -> SW_APSRC2_W<12> {
        SW_APSRC2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SBS compensation cell for I/Os software code register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccswcr](index.html) module
pub struct CCSWCR_SPEC;
impl crate::RegisterSpec for CCSWCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccswcr::R](R) reader structure
impl crate::Readable for CCSWCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccswcr::W](W) writer structure
impl crate::Writable for CCSWCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCSWCR to value 0x7878
impl crate::Resettable for CCSWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7878;
}
