#[doc = "Register `DCR` reader"]
pub type R = crate::R<DcrSpec>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DcrSpec>;
#[doc = "Field `DBA` reader - DMA base address"]
pub type DbaR = crate::FieldReader;
#[doc = "Field `DBA` writer - DMA base address"]
pub type DbaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBL` reader - DMA burst length"]
pub type DblR = crate::FieldReader;
#[doc = "Field `DBL` writer - DMA burst length"]
pub type DblW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline(always)]
    pub fn dba(&self) -> DbaR {
        DbaR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline(always)]
    pub fn dbl(&self) -> DblR {
        DblR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline(always)]
    pub fn dba(&mut self) -> DbaW<'_, DcrSpec> {
        DbaW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline(always)]
    pub fn dbl(&mut self) -> DblW<'_, DcrSpec> {
        DblW::new(self, 8)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrSpec;
impl crate::RegisterSpec for DcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DcrSpec {}
