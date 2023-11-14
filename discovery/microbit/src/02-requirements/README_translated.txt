# Hardware/knowledge requirements The primary knowledge requirement to read this book is to know *some* Rust.
It's hard for me to quantify *some* but at least I can tell you that you don't need to fully grok generics, but you do need to know how to *use* closures.
You also need to be familiar with the idioms of the [2018 edition], in particular with the fact that `extern crate` is not necessary in the 2018 edition.
[2018 edition]: https://rust-lang-nursery.github.io/edition-guide/ Also, to follow this material you'll need the following hardware: - A [micro:bit v2] board, alternatively a [micro:bit v1.5] board, the book will refer to the v1.5 as just v1.
[micro:bit v2]: https://tech.microbit.org/hardware/ [micro:bit v1.5]: https://tech.microbit.org/hardware/1-5-revision/ (You can purchase this board from several [electronics][0] [suppliers][1]) [0]: https://microbit.org/buy/ [1]: https://www.mouser.com/microbit/_/N-aez3t?P=1y8um0l <p align="center"> <img title="micro:bit" src="../assets/microbit-v2.
# 硬件/知识要求
阅读本书的主要知识要求是了解一些Rust。
很难量化“一些”，但至少我可以告诉你，你不需要完全理解泛型，但你需要知道如何使用闭包。
你还需要熟悉[2018版]的习惯用法，特别是要知道在2018版中不需要使用`extern crate`。
[2018版]: https://rust-lang-nursery.github.io/edition-guide/
此外，为了跟随本材料，你需要以下硬件：
- 一个[micro:bit v2]开发板，或者一个[micro:bit v1.5]开发板，本书将把v1.5简称为v1。
[micro:bit v2]: https://tech.microbit.org/hardware/
[micro:bit v1.5]: https://tech.microbit.org/hardware/1-5-revision/
（你可以从几个[电子][0] [供应商][1]购买这个开发板）[0]: https://microbit.org/buy/
[1]: https://www.mouser.com/microbit/_/N-aez3t?P=1y8um0l
<p align="center"> <img title="micro:bit" src="../assets/microbit-v2.
jpg"> </p> > **NOTE** This is an image of a micro:bit v2, the front of the v1 looks slightly different - One micro-B USB cable, required to make the micro:bit board work.
Make sure that the cable supports data transfer as some cables only support charging devices.
<p align="center"> <img title="micro-B USB cable" src="../assets/usb-cable.jpg"> </p> > **NOTE** You may already have a cable like this, as some micro:bit kits ship with such cables.
> Some USB cables used to charge mobile devices may also work, if they are micro-B and have the > capability to transmit data.
> **FAQ**: Wait, why do I need this specific hardware? It makes my life and yours much easier.
The material is much, much more approachable if we don't have to worry about hardware differences.
Trust me on this one.
> **FAQ**: Can I follow this material with a different development board? Maybe?
jpg"> </p> > **注意** 这是一张 micro:bit v2 的图片，v1 的正面看起来略有不同 - 一个 micro-B USB 数据线，用于使 micro:bit 板工作。
请确保该数据线支持数据传输，因为有些数据线只支持充电设备。
 <p align="center"> <img title="micro-B USB 数据线" src="../assets/usb-cable.jpg"> </p> > **注意** 你可能已经有一根类似的数据线，因为一些 micro:bit 套件会附带这样的数据线。
 > 一些用于充电移动设备的 USB 数据线也可能适用，如果它们是 micro-B 类型并具有传输数据的能力。
 > **常见问题**：等等，为什么我需要这个特定的硬件？这样做会让我的生活和你的生活更轻松。
如果我们不必担心硬件差异，材料会更容易理解。
相信我。
 > **常见问题**：我可以用其他开发板来跟随这个材料吗？也许？
 It depends mainly on two things: your previous experience with microcontrollers and/or whether a high level crate already exists, like the [`nrf52-hal`], for your development board somewhere.
You can look through the [Awesome Embedded Rust HAL list] for your microcontroller, if you intend to use a different one.
[`nrf52-hal`]: https://docs.rs/nrf52-hal [Awesome Embedded Rust HAL list]: https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates With a different development board, this text would lose most if not all its beginner friendliness and "easy to follow"-ness, IMO.
If you have a different development board and you don't consider yourself a total beginner, you are better off starting with the [quickstart] project template.
[quickstart]: https://rust-embedded.github.
这主要取决于两件事：您之前对微控制器的经验以及是否已经存在高级的板条箱，例如[`nrf52-hal`]，用于您的开发板。
如果您打算使用其他微控制器，可以在[嵌入式Rust HAL列表]中查找。
[`nrf52-hal`]: https://docs.rs/nrf52-hal [嵌入式Rust HAL列表]: https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates 对于不同的开发板，这段文字将失去大部分，如果不是全部的初学者友好性和“易于跟随性”，在我看来。
如果您有不同的开发板，并且不认为自己是完全的初学者，最好从[快速入门]项目模板开始。
[快速入门]: https://rust-embedded.github.
