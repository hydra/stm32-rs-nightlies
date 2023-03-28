///Register `EPIDR` reader
pub struct R(crate::R<EPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EPIDR` writer
pub struct W(crate::W<EPIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPIDR_SPEC>;
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
impl From<crate::W<EPIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPIDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MIPIID` reader - 4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\[15:12\]
///of the 48-bit provisioned ID. Note: The bits\[11:0\]
///of the provisioned ID may be 0.
pub type MIPIID_R = crate::FieldReader<u8, u8>;
///Field `MIPIID` writer - 4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\[15:12\]
///of the 48-bit provisioned ID. Note: The bits\[11:0\]
///of the provisioned ID may be 0.
pub type MIPIID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPIDR_SPEC, u8, u8, 4, O>;
///Field `IDTSEL` reader - provisioned ID type selector This field is set as 0 i.e. vendor fixed value. This field represents the bit\[32\]
///of the 48-bit provisioned ID. Note: The bits\[31:16\]
///of the provisioned ID may be 0.
pub type IDTSEL_R = crate::BitReader<bool>;
///Field `MIPIMID` reader - 15-bit MIPI manufacturer ID This read field is the 15-bit STMicroelectronics MIPI ID i.e. 0x0104. This field represents the bits\[47:33\]
///of the 48-bit provisioned ID.
pub type MIPIMID_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 12:15 - 4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\[15:12\]
    ///of the 48-bit provisioned ID. Note: The bits\[11:0\]
    ///of the provisioned ID may be 0.
    #[inline(always)]
    pub fn mipiid(&self) -> MIPIID_R {
        MIPIID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - provisioned ID type selector This field is set as 0 i.e. vendor fixed value. This field represents the bit\[32\]
    ///of the 48-bit provisioned ID. Note: The bits\[31:16\]
    ///of the provisioned ID may be 0.
    #[inline(always)]
    pub fn idtsel(&self) -> IDTSEL_R {
        IDTSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:31 - 15-bit MIPI manufacturer ID This read field is the 15-bit STMicroelectronics MIPI ID i.e. 0x0104. This field represents the bits\[47:33\]
    ///of the 48-bit provisioned ID.
    #[inline(always)]
    pub fn mipimid(&self) -> MIPIMID_R {
        MIPIMID_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    ///Bits 12:15 - 4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\[15:12\]
    ///of the 48-bit provisioned ID. Note: The bits\[11:0\]
    ///of the provisioned ID may be 0.
    #[inline(always)]
    #[must_use]
    pub fn mipiid(&mut self) -> MIPIID_W<12> {
        MIPIID_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I3C extended provisioned ID register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [epidr](index.html) module
pub struct EPIDR_SPEC;
impl crate::RegisterSpec for EPIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [epidr::R](R) reader structure
impl crate::Readable for EPIDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [epidr::W](W) writer structure
impl crate::Writable for EPIDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EPIDR to value 0x0208_0000
impl crate::Resettable for EPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0208_0000;
}
