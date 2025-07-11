#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `LSION` reader - Internal low speed oscillator enable"]
pub type LsionR = crate::BitReader;
#[doc = "Field `LSION` writer - Internal low speed oscillator enable"]
pub type LsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - Internal low speed oscillator ready"]
pub type LsirdyR = crate::BitReader;
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RmvfR = crate::BitReader;
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RmvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub type OblrstfR = crate::BitReader;
#[doc = "Field `OBLRSTF` writer - Option byte loader reset flag"]
pub type OblrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINRSTF` reader - PIN reset flag"]
pub type PinrstfR = crate::BitReader;
#[doc = "Field `PINRSTF` writer - PIN reset flag"]
pub type PinrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub type PorrstfR = crate::BitReader;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub type PorrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SftrstfR = crate::BitReader;
#[doc = "Field `SFTRSTF` writer - Software reset flag"]
pub type SftrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDGRSTF` reader - Independent watchdog reset flag"]
pub type IwdgrstfR = crate::BitReader;
#[doc = "Field `IWDGRSTF` writer - Independent watchdog reset flag"]
pub type IwdgrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WwdgrstfR = crate::BitReader;
#[doc = "Field `WWDGRSTF` writer - Window watchdog reset flag"]
pub type WwdgrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub type LpwrrstfR = crate::BitReader;
#[doc = "Field `LPWRRSTF` writer - Low-power reset flag"]
pub type LpwrrstfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal low speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LsionR {
        LsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal low speed oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LsirdyR {
        LsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RmvfR {
        RmvfR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OblrstfR {
        OblrstfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PinrstfR {
        PinrstfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PorrstfR {
        PorrstfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SftrstfR {
        SftrstfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IwdgrstfR {
        IwdgrstfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WwdgrstfR {
        WwdgrstfR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LpwrrstfR {
        LpwrrstfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal low speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&mut self) -> LsionW<'_, CsrSpec> {
        LsionW::new(self, 0)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RmvfW<'_, CsrSpec> {
        RmvfW::new(self, 24)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&mut self) -> OblrstfW<'_, CsrSpec> {
        OblrstfW::new(self, 25)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PinrstfW<'_, CsrSpec> {
        PinrstfW::new(self, 26)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PorrstfW<'_, CsrSpec> {
        PorrstfW::new(self, 27)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SftrstfW<'_, CsrSpec> {
        SftrstfW::new(self, 28)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&mut self) -> IwdgrstfW<'_, CsrSpec> {
        IwdgrstfW::new(self, 29)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WwdgrstfW<'_, CsrSpec> {
        WwdgrstfW::new(self, 30)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LpwrrstfW<'_, CsrSpec> {
        LpwrrstfW::new(self, 31)
    }
}
#[doc = "Control/status register (RCC_CSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0x0c00_0000"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
