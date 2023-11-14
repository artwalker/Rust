# Background ## What's a microcontroller? A microcontroller is a *system* on a chip.
Whereas your computer is made up of several discrete components: a processor, RAM, storage, an Ethernet port, etc.; a microcontroller has all those types of components built into a single "chip" or package.
This makes it possible to build systems with fewer parts.
## What can you do with a microcontroller? Lots of things! Microcontrollers are the central part of what are known as "*embedded* systems".
Embedded systems are everywhere, but you don't usually notice them.
They control the machines that wash your clothes, print your documents, and cook your food.
Embedded systems keep the buildings that you live and work in at a comfortable temperature, and control the components that make the vehicles you travel in stop and go.
Most embedded systems operate without user intervention.
Even if they expose a user interface like a washing machine does; most of their operation is done on their own.
# 背景
## 什么是微控制器？
微控制器是一种芯片上的系统。
而你的计算机由几个离散的组件组成：处理器、RAM、存储器、以太网端口等等；而微控制器将所有这些类型的组件集成到一个单独的“芯片”或封装中。
这使得可以用更少的零件来构建系统。


## 你可以用微控制器做什么？
很多事情！
微控制器是所谓的“嵌入式系统”的核心部分。
嵌入式系统无处不在，但你通常不会注意到它们。
它们控制着洗衣机、打印机、烹饪设备等机器的运行。
嵌入式系统保持你居住和工作的建筑物处于舒适的温度，并控制着你乘坐的车辆的停止和启动。
大多数嵌入式系统在没有用户干预的情况下运行。
即使它们像洗衣机一样提供用户界面；它们的大部分操作都是自动完成的。

 Embedded systems are often used to *control* a physical process.
To make this possible, they have one or more devices to tell them about the state of the world ("sensors"), and one or more devices which allow them to change things ("actuators").
For example, a building climate control system might have: - Sensors which measure temperature and humidity in various locations.
- Actuators which control the speed of fans.
- Actuators which cause heat to be added or removed from the building.
## When should I use a microcontroller? Many of the embedded systems listed above could be implemented with a computer running Linux (for example a "Raspberry Pi").
Why use a microcontroller instead? Sounds like it might be harder to develop a program.
Some reasons might include: **Cost.** A microcontroller is much cheaper than a general purpose computer.
Not only is the microcontroller cheaper; it also requires many fewer external electrical components to operate.
嵌入式系统通常用于控制物理过程。
为了实现这一点，它们有一个或多个设备来告诉它们世界的状态（“传感器”），以及一个或多个设备可以改变事物（“执行器”）。
例如，建筑物的气候控制系统可能具有以下功能：-测量各个位置的温度和湿度的传感器。
-控制风扇速度的执行器。
-使建筑物加热或冷却的执行器。
##何时应该使用微控制器？上述许多嵌入式系统可以使用运行Linux的计算机（例如“树莓派”）来实现。
为什么要使用微控制器呢？听起来可能更难开发程序。
一些原因可能包括：**成本。
**微控制器比通用计算机便宜得多。
微控制器不仅更便宜，而且需要更少的外部电子元件来运行。

 This makes Printed Circuit Boards (PCB) smaller and cheaper to design and manufacture.
**Power consumption.** Most microcontrollers consume a fraction of the power of a full blown processor.
For applications which run on batteries, that makes a huge difference.
**Responsiveness.** To accomplish their purpose, some embedded systems must always react within a limited time interval (e.g.
the "anti-lock" braking system of a car).
If the system misses this type of *deadline*, a catastrophic failure might occur.
Such a deadline is called a "hard real time" requirement.
An embedded system which is bound by such a deadline is referred to as a "hard real-time system".
A general purpose computer and OS usually has many software components which share the computer's processing resources.
This makes it harder to guarantee execution of a program within tight time constraints.
**Reliability.** In systems with fewer components (both hardware and software), there is less to go wrong!
这使得印刷电路板（PCB）的设计和制造更小、更便宜。
**功耗。
**大多数微控制器的功耗只有完整处理器的一小部分。
对于使用电池的应用程序来说，这是一个巨大的差异。
**响应能力。
**为了实现其目的，一些嵌入式系统必须始终在有限的时间间隔内做出反应（例如汽车的“防抱死”制动系统）。
如果系统错过了这种类型的“截止日期”，可能会发生灾难性的故障。
这种截止日期被称为“硬实时”要求。
受此截止日期限制的嵌入式系统被称为“硬实时系统”。
通用计算机和操作系统通常有许多共享计算机处理资源的软件组件。
这使得在严格的时间限制内保证程序的执行变得更加困难。
**可靠性。
**在组件较少（包括硬件和软件）的系统中，出错的可能性较小！

 ## When should I *not* use a microcontroller? Where heavy computations are involved.
To keep their power consumption low, microcontrollers have very limited computational resources available to them.
For example, some microcontrollers don't even have hardware support for floating point operations.
On those devices, performing a simple addition of single precision numbers can take hundreds of CPU cycles.
## Why use Rust and not C? Hopefully, I don't need to convince you here as you are probably familiar with the language differences between Rust and C.
One point I do want to bring up is package management.
C lacks an official, widely accepted package management solution whereas Rust has Cargo.
This makes development *much* easier.
And, IMO, easy package management encourages code reuse because libraries can be easily integrated into an application which is also a good thing as libraries get more "battle testing".
## Why should I not use Rust? Or why should I prefer C over Rust?
何时不应使用微控制器？当涉及到大量计算时。
为了保持功耗低，微控制器的计算资源非常有限。
例如，一些微控制器甚至没有硬件支持浮点运算。
在这些设备上，执行单精度数的简单加法可能需要数百个CPU周期。


为什么使用Rust而不是C？希望我不需要在这里说服你，因为你可能已经熟悉Rust和C之间的语言差异。
我想提出的一个观点是包管理。
C缺乏官方、广泛接受的包管理解决方案，而Rust有Cargo。
这使得开发变得更加容易。
而且，在我看来，简单的包管理鼓励代码重用，因为库可以轻松地集成到应用程序中，这也是一个好事，因为库经过了更多的“实战测试”。


为什么不应该使用Rust？或者为什么应该更喜欢C而不是Rust？
 The C ecosystem is way more mature.
Off the shelf solutions for several problems already exist.
If you need to control a time sensitive process, you can grab one of the existing commercial Real Time Operating Systems (RTOS) out there and solve your problem.
There are no commercial, production-grade RTOSes in Rust yet so you would have to either create one yourself or try one of the ones that are in development.
You can find a list of those in the [Awesome Embedded Rust] repository.
[Awesome Embedded Rust]: https://github.
C生态系统更加成熟。
已经存在许多现成的解决方案来解决几个问题。
如果您需要控制一个时间敏感的过程，您可以选择现有的商业实时操作系统（RTOS）并解决您的问题。
目前还没有商业级别的Rust RTOS，所以您要么自己创建一个，要么尝试其中一个正在开发中的RTOS。
您可以在[令人惊叹的嵌入式Rust]存储库中找到这些的列表。
[令人惊叹的嵌入式Rust]: https://github.
