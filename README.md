# `vk2dfd`

This is a tiny crate that converts Vulkan formats to Khronos [Data Format Descriptor]s.

Conversion of Vulkan formats to DFDs is primarily useful for software that wants
to write [KTX] or [KTX2] format textures. The format is straightforward to
write, with the sole exception of the data format descriptor, which is a complex
specification. With this crate, you can write textures in those formats with
ease.

Unlike the official Khronos [`dfdutils`] package, this crate simply uses a
hardcoded table of formats and their associated DFDs. The number of formats is
relatively small, so this is the most efficient way to implement this
functionality.

# Code of Conduct

`vk2dfd` follows the same Code of Conduct as Rust itself. Reports can be made to
the project authors.

# License

Licensed under the Apache 2.0 license, or the MIT license, at your option. See
the `LICENSE-APACHE2` and `LICENSE-MIT` files for details.

[Data Format Descriptor]: https://registry.khronos.org/DataFormat/specs/1.3/dataformat.1.3.html#_the_khronos_data_format_descriptor_overview

[KTX]: https://www.khronos.org/ktx/

[KTX2]: https://www.khronos.org/ktx/

[`dfdutils`]: https://github.com/KhronosGroup/KTX-Software
