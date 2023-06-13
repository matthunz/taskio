#![no_std]
#![no_main]

use core::pin::Pin;
use cortex_m::singleton;
use cortex_m_rt::entry;
use defmt::println;
use stm32f1xx_hal::{
    device::USART3,
    dma::{dma1::C3, CircBuffer, Half, RxDma, Event},
    pac,
    prelude::*,
    serial::{Config, Rx, Serial},
};
use taskio::{io::Read, Poll, Task};
use taskio_hal_examples as _;

struct SerialRx {
    circ_buffer: CircBuffer<[u8; 1], RxDma<Rx<USART3>, C3>>,
    half: Half,
}

impl SerialRx {
    pub fn new(circ_buffer: CircBuffer<[u8; 1], RxDma<Rx<USART3>, C3>>) -> Self {
        Self {
            circ_buffer,
            half: Half::Second,
        }
    }
}

impl Read for SerialRx {
    type Error = ();

    fn poll_read(mut self: Pin<&mut Self>, buf: &mut [u8]) -> Poll<Result<usize, Self::Error>> {
        let half = self.circ_buffer.readable_half().unwrap();
        if half == self.half {
            return Poll::Pending;
        }

        self.half = half;

        let bytes = self.circ_buffer.peek(|half, _| *half).unwrap();
        let len = core::cmp::min(bytes.len(), buf.len());
        buf[..len].copy_from_slice(&bytes[..len]);

        Poll::Ready(Ok(len))
    }
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain();
    let channels = p.DMA1.split();
    // channels.3.listen(Event::TransferComplete);

    // let mut gpioa = p.GPIOA.split();
    let mut gpiob = p.GPIOB.split();

    // USART1
    // let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    // let rx = gpioa.pa10;

    // USART1
    // let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let rx = gpiob.pb7;

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // USART3
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    let serial = Serial::new(
        p.USART3,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(9_600.bps()),
        &clocks,
    );

    let rx = serial.rx.with_dma(channels.3);
    let buf = singleton!(: [[u8; 1]; 2] = [[0; 1]; 2]).unwrap();
    let circ_buffer = rx.circ_read(buf);

    let mut rx = SerialRx::new(circ_buffer);

    let mut buf = [0; 8];
    loop {
        let used = (&mut rx).read(&mut buf).block().unwrap();
        println!("{:?}", &buf[..used]);
    }
}
