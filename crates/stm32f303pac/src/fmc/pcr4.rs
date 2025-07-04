#[doc = "Register `PCR4` reader"]
pub type R = crate::R<Pcr4Spec>;
#[doc = "Register `PCR4` writer"]
pub type W = crate::W<Pcr4Spec>;
#[doc = "Field `PWAITEN` reader - PWAITEN"]
pub type PwaitenR = crate::BitReader;
#[doc = "Field `PWAITEN` writer - PWAITEN"]
pub type PwaitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBKEN` reader - PBKEN"]
pub type PbkenR = crate::BitReader;
#[doc = "Field `PBKEN` writer - PBKEN"]
pub type PbkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTYP` reader - PTYP"]
pub type PtypR = crate::BitReader;
#[doc = "Field `PTYP` writer - PTYP"]
pub type PtypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWID` reader - PWID"]
pub type PwidR = crate::FieldReader;
#[doc = "Field `PWID` writer - PWID"]
pub type PwidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECCEN` reader - ECCEN"]
pub type EccenR = crate::BitReader;
#[doc = "Field `ECCEN` writer - ECCEN"]
pub type EccenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCLR` reader - TCLR"]
pub type TclrR = crate::FieldReader;
#[doc = "Field `TCLR` writer - TCLR"]
pub type TclrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TAR` reader - TAR"]
pub type TarR = crate::FieldReader;
#[doc = "Field `TAR` writer - TAR"]
pub type TarW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ECCPS` reader - ECCPS"]
pub type EccpsR = crate::FieldReader;
#[doc = "Field `ECCPS` writer - ECCPS"]
pub type EccpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PwaitenR {
        PwaitenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&self) -> PbkenR {
        PbkenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    pub fn ptyp(&self) -> PtypR {
        PtypR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&self) -> PwidR {
        PwidR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&self) -> EccenR {
        EccenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&self) -> TclrR {
        TclrR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&self) -> TarR {
        TarR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    pub fn eccps(&self) -> EccpsR {
        EccpsR::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PwaitenW<'_, Pcr4Spec> {
        PwaitenW::new(self, 1)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&mut self) -> PbkenW<'_, Pcr4Spec> {
        PbkenW::new(self, 2)
    }
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    pub fn ptyp(&mut self) -> PtypW<'_, Pcr4Spec> {
        PtypW::new(self, 3)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&mut self) -> PwidW<'_, Pcr4Spec> {
        PwidW::new(self, 4)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&mut self) -> EccenW<'_, Pcr4Spec> {
        EccenW::new(self, 6)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&mut self) -> TclrW<'_, Pcr4Spec> {
        TclrW::new(self, 9)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&mut self) -> TarW<'_, Pcr4Spec> {
        TarW::new(self, 13)
    }
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    pub fn eccps(&mut self) -> EccpsW<'_, Pcr4Spec> {
        EccpsW::new(self, 17)
    }
}
#[doc = "PC Card/NAND Flash control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcr4Spec;
impl crate::RegisterSpec for Pcr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr4::R`](R) reader structure"]
impl crate::Readable for Pcr4Spec {}
#[doc = "`write(|w| ..)` method takes [`pcr4::W`](W) writer structure"]
impl crate::Writable for Pcr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCR4 to value 0x18"]
impl crate::Resettable for Pcr4Spec {
    const RESET_VALUE: u32 = 0x18;
}
