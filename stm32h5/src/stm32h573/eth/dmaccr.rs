///Register `DMACCR` reader
pub struct R(crate::R<DMACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMACCR` writer
pub struct W(crate::W<DMACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACCR_SPEC>;
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
impl From<crate::W<DMACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MSS` reader - Maximum Segment Size This field specifies the maximum segment size that should be used while segmenting the packet. This field is valid only if the TSE bit of Channel transmit control register (ETH_DMACTXCR) is set. The value programmed in this field must be more than the configured Data width in bytes. It is recommended to use a MSS value of 64 bytes or more.
pub type MSS_R = crate::FieldReader<u16, u16>;
///Field `MSS` writer - Maximum Segment Size This field specifies the maximum segment size that should be used while segmenting the packet. This field is valid only if the TSE bit of Channel transmit control register (ETH_DMACTXCR) is set. The value programmed in this field must be more than the configured Data width in bytes. It is recommended to use a MSS value of 64 bytes or more.
pub type MSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACCR_SPEC, u16, u16, 14, O>;
///Field `PBLX8` reader - 8xPBL mode When this bit is set, the PBL value programmed in Bits\[21:16\]
///in Channel transmit control register (ETH_DMACTXCR) is multiplied eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value.
pub type PBLX8_R = crate::BitReader<bool>;
///Field `PBLX8` writer - 8xPBL mode When this bit is set, the PBL value programmed in Bits\[21:16\]
///in Channel transmit control register (ETH_DMACTXCR) is multiplied eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value.
pub type PBLX8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACCR_SPEC, bool, O>;
///Field `DSL` reader - Descriptor Skip Length This bit specifies the 32-bit word number to skip between two unchained descriptors. The address skipping starts from the end of the current descriptor to the start of the next descriptor. When the DSL value is equal to zero, the DMA takes the descriptor table as contiguous.
pub type DSL_R = crate::FieldReader<u8, u8>;
///Field `DSL` writer - Descriptor Skip Length This bit specifies the 32-bit word number to skip between two unchained descriptors. The address skipping starts from the end of the current descriptor to the start of the next descriptor. When the DSL value is equal to zero, the DMA takes the descriptor table as contiguous.
pub type DSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACCR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:13 - Maximum Segment Size This field specifies the maximum segment size that should be used while segmenting the packet. This field is valid only if the TSE bit of Channel transmit control register (ETH_DMACTXCR) is set. The value programmed in this field must be more than the configured Data width in bytes. It is recommended to use a MSS value of 64 bytes or more.
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\[21:16\]
    ///in Channel transmit control register (ETH_DMACTXCR) is multiplied eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value.
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 18:20 - Descriptor Skip Length This bit specifies the 32-bit word number to skip between two unchained descriptors. The address skipping starts from the end of the current descriptor to the start of the next descriptor. When the DSL value is equal to zero, the DMA takes the descriptor table as contiguous.
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    ///Bits 0:13 - Maximum Segment Size This field specifies the maximum segment size that should be used while segmenting the packet. This field is valid only if the TSE bit of Channel transmit control register (ETH_DMACTXCR) is set. The value programmed in this field must be more than the configured Data width in bytes. It is recommended to use a MSS value of 64 bytes or more.
    #[inline(always)]
    #[must_use]
    pub fn mss(&mut self) -> MSS_W<0> {
        MSS_W::new(self)
    }
    ///Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\[21:16\]
    ///in Channel transmit control register (ETH_DMACTXCR) is multiplied eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value.
    #[inline(always)]
    #[must_use]
    pub fn pblx8(&mut self) -> PBLX8_W<16> {
        PBLX8_W::new(self)
    }
    ///Bits 18:20 - Descriptor Skip Length This bit specifies the 32-bit word number to skip between two unchained descriptors. The address skipping starts from the end of the current descriptor to the start of the next descriptor. When the DSL value is equal to zero, the DMA takes the descriptor table as contiguous.
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<18> {
        DSL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmaccr](index.html) module
pub struct DMACCR_SPEC;
impl crate::RegisterSpec for DMACCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmaccr::R](R) reader structure
impl crate::Readable for DMACCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmaccr::W](W) writer structure
impl crate::Writable for DMACCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMACCR to value 0
impl crate::Resettable for DMACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
