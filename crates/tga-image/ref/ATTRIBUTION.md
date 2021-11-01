These C++ files are being used as a reference for Rust implementation. Unfortunately the structure isn't very conducive to Rust as it's possible to construct the Image in an invalid state which makes use of some member functions undefined behaviour in such a state. While the basic operations will be the same, the overall structure of them will be quite different.

---
Original licence
```
Tiny Renderer, https://github.com/ssloy/tinyrenderer
Copyright Dmitry V. Sokolov

This software is provided 'as-is', without any express or implied warranty.
In no event will the authors be held liable for any damages arising from the use of this software.
Permission is granted to anyone to use this software for any purpose,
including commercial applications, and to alter it and redistribute it freely,
subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
3. This notice may not be removed or altered from any source distribution.
```
