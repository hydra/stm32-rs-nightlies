///Register `STR` reader
pub struct R(crate::R<STR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `STR` writer
pub struct W(crate::W<STR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STR_SPEC>;
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
impl From<crate::W<STR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NBLW` reader - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW.
pub type NBLW_R = crate::FieldReader<u8, u8>;
///Field `NBLW` writer - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW.
pub type NBLW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STR_SPEC, u8, u8, 5, O>;
///Field `DCAL` reader - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0.
pub type DCAL_R = crate::BitReader<bool>;
///Field `DCAL` writer - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0.
pub type DCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STR_SPEC, bool, O>;
impl R {
    ///Bits 0:4 - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW.
    #[inline(always)]
    pub fn nblw(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0.
    #[inline(always)]
    pub fn dcal(&self) -> DCAL_R {
        DCAL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW.
    #[inline(always)]
    #[must_use]
    pub fn nblw(&mut self) -> NBLW_W<0> {
        NBLW_W::new(self)
    }
    ///Bit 8 - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0.
    #[inline(always)]
    #[must_use]
    pub fn dcal(&mut self) -> DCAL_W<8> {
        DCAL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HASH start register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [str](index.html) module
pub struct STR_SPEC;
impl crate::RegisterSpec for STR_SPEC {
    type Ux = u32;
}
///`read()` method returns [str::R](R) reader structure
impl crate::Readable for STR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [str::W](W) writer structure
impl crate::Writable for STR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets STR to value 0
impl crate::Resettable for STR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
