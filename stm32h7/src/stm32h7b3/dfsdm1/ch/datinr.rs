///Register `DATINR` reader
pub struct R(crate::R<DATINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATINR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DATINR` writer
pub struct W(crate::W<DATINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATINR_SPEC>;
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
impl From<crate::W<DATINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATINR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INDAT0` reader - Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX\[1:0\]=1 or DATMPX\[1:0\]=2. Data can be written by CPU/DMA (if DATMPX\[1:0\]=2) or directly by internal ADC (if DATMPX\[1:0\]=1). If DATPACK\[1:0\]=0 (standard mode) Channel y data sample is stored into INDAT0\[15:0\]. If DATPACK\[1:0\]=1 (interleaved mode) First channel y data sample is stored into INDAT0\[15:0\]. Second channel y data sample is stored into INDAT1\[15:0\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\[1:0\]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0\[15:0\]. For odd y channels: INDAT0\[15:0\]
///is write protected. See for more details. INDAT0\[15:0\]
///is in the16-bit signed format.
pub type INDAT0_R = crate::FieldReader<u16, u16>;
///Field `INDAT0` writer - Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX\[1:0\]=1 or DATMPX\[1:0\]=2. Data can be written by CPU/DMA (if DATMPX\[1:0\]=2) or directly by internal ADC (if DATMPX\[1:0\]=1). If DATPACK\[1:0\]=0 (standard mode) Channel y data sample is stored into INDAT0\[15:0\]. If DATPACK\[1:0\]=1 (interleaved mode) First channel y data sample is stored into INDAT0\[15:0\]. Second channel y data sample is stored into INDAT1\[15:0\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\[1:0\]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0\[15:0\]. For odd y channels: INDAT0\[15:0\]
///is write protected. See for more details. INDAT0\[15:0\]
///is in the16-bit signed format.
pub type INDAT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATINR_SPEC, u16, u16, 16, O>;
///Field `INDAT1` reader - Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX\[1:0\]=1 or DATMPX\[1:0\]=2. Data can be written by CPU/DMA (if DATMPX\[1:0\]=2) or directly by internal ADC (if DATMPX\[1:0\]=1). If DATPACK\[1:0\]=0 (standard mode) INDAT0\[15:0\]
///is write protected (not used for input sample). If DATPACK\[1:0\]=1 (interleaved mode) Second channel y data sample is stored into INDAT1\[15:0\]. First channel y data sample is stored into INDAT0\[15:0\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\[1:0\]=2 (dual mode). For even y channels: sample in INDAT1\[15:0\]
///is automatically copied into INDAT0\[15:0\]
///of channel (y+1). For odd y channels: INDAT1\[15:0\]
///is write protected. See for more details. INDAT0\[15:1\]
///is in the16-bit signed format.
pub type INDAT1_R = crate::FieldReader<u16, u16>;
///Field `INDAT1` writer - Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX\[1:0\]=1 or DATMPX\[1:0\]=2. Data can be written by CPU/DMA (if DATMPX\[1:0\]=2) or directly by internal ADC (if DATMPX\[1:0\]=1). If DATPACK\[1:0\]=0 (standard mode) INDAT0\[15:0\]
///is write protected (not used for input sample). If DATPACK\[1:0\]=1 (interleaved mode) Second channel y data sample is stored into INDAT1\[15:0\]. First channel y data sample is stored into INDAT0\[15:0\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\[1:0\]=2 (dual mode). For even y channels: sample in INDAT1\[15:0\]
///is automatically copied into INDAT0\[15:0\]
///of channel (y+1). For odd y channels: INDAT1\[15:0\]
///is write protected. See for more details. INDAT0\[15:1\]
///is in the16-bit signed format.
pub type INDAT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATINR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX\[1:0\]=1 or DATMPX\[1:0\]=2. Data can be written by CPU/DMA (if DATMPX\[1:0\]=2) or directly by internal ADC (if DATMPX\[1:0\]=1). If DATPACK\[1:0\]=0 (standard mode) Channel y data sample is stored into INDAT0\[15:0\]. If DATPACK\[1:0\]=1 (interleaved mode) First channel y data sample is stored into INDAT0\[15:0\]. Second channel y data sample is stored into INDAT1\[15:0\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\[1:0\]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0\[15:0\]. For odd y channels: INDAT0\[15:0\]
    ///is write protected. See for more details. INDAT0\[15:0\]
    ///is in the16-bit signed format.
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX\[1:0\]=1 or DATMPX\[1:0\]=2. Data can be written by CPU/DMA (if DATMPX\[1:0\]=2) or directly by internal ADC (if DATMPX\[1:0\]=1). If DATPACK\[1:0\]=0 (standard mode) INDAT0\[15:0\]
    ///is write protected (not used for input sample). If DATPACK\[1:0\]=1 (interleaved mode) Second channel y data sample is stored into INDAT1\[15:0\]. First channel y data sample is stored into INDAT0\[15:0\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\[1:0\]=2 (dual mode). For even y channels: sample in INDAT1\[15:0\]
    ///is automatically copied into INDAT0\[15:0\]
    ///of channel (y+1). For odd y channels: INDAT1\[15:0\]
    ///is write protected. See for more details. INDAT0\[15:1\]
    ///is in the16-bit signed format.
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX\[1:0\]=1 or DATMPX\[1:0\]=2. Data can be written by CPU/DMA (if DATMPX\[1:0\]=2) or directly by internal ADC (if DATMPX\[1:0\]=1). If DATPACK\[1:0\]=0 (standard mode) Channel y data sample is stored into INDAT0\[15:0\]. If DATPACK\[1:0\]=1 (interleaved mode) First channel y data sample is stored into INDAT0\[15:0\]. Second channel y data sample is stored into INDAT1\[15:0\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\[1:0\]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0\[15:0\]. For odd y channels: INDAT0\[15:0\]
    ///is write protected. See for more details. INDAT0\[15:0\]
    ///is in the16-bit signed format.
    #[inline(always)]
    #[must_use]
    pub fn indat0(&mut self) -> INDAT0_W<0> {
        INDAT0_W::new(self)
    }
    ///Bits 16:31 - Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX\[1:0\]=1 or DATMPX\[1:0\]=2. Data can be written by CPU/DMA (if DATMPX\[1:0\]=2) or directly by internal ADC (if DATMPX\[1:0\]=1). If DATPACK\[1:0\]=0 (standard mode) INDAT0\[15:0\]
    ///is write protected (not used for input sample). If DATPACK\[1:0\]=1 (interleaved mode) Second channel y data sample is stored into INDAT1\[15:0\]. First channel y data sample is stored into INDAT0\[15:0\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\[1:0\]=2 (dual mode). For even y channels: sample in INDAT1\[15:0\]
    ///is automatically copied into INDAT0\[15:0\]
    ///of channel (y+1). For odd y channels: INDAT1\[15:0\]
    ///is write protected. See for more details. INDAT0\[15:1\]
    ///is in the16-bit signed format.
    #[inline(always)]
    #[must_use]
    pub fn indat1(&mut self) -> INDAT1_W<16> {
        INDAT1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DFSDM channel 0 data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [datinr](index.html) module
pub struct DATINR_SPEC;
impl crate::RegisterSpec for DATINR_SPEC {
    type Ux = u32;
}
///`read()` method returns [datinr::R](R) reader structure
impl crate::Readable for DATINR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [datinr::W](W) writer structure
impl crate::Writable for DATINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DATINR to value 0
impl crate::Resettable for DATINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
