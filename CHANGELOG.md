<a name="0.1.0"></a>
## 0.1.0 (2016-12-21)


#### Features

*   provide TermCell constructors ([b494b2d6](https://github.com/lidavidm/voodoo/commit/b494b2d644cfdb573ec242474b40f3c2b9963a85))
*   restructure around Termion ([b47c5588](https://github.com/lidavidm/voodoo/commit/b47c5588b25d3614c7cabdc517135d40a7c0af24))
*   make poll_event more natural ([9da714a3](https://github.com/lidavidm/voodoo/commit/9da714a3e231bb3aba1a84478cdad3e1bc81bf05))
*   add basic wrapper around NCurses init routines ([548bd2cf](https://github.com/lidavidm/voodoo/commit/548bd2cfca372d82362ae13f18f14657e6692ed1))
* **Color:**
  *  implement more of the colors ([8052d3bd](https://github.com/lidavidm/voodoo/commit/8052d3bd468fcd88b4538f2ea6601e76229f7fdf))
  *  implement red ([841db1f1](https://github.com/lidavidm/voodoo/commit/841db1f1622c8daa3a86f957fbf4fc9abd6e9cfb))
  *  add easier-to-use Color enum ([7d42a963](https://github.com/lidavidm/voodoo/commit/7d42a963d0504cfd8afeb8c68e8ec636dc38e51f))
* **Compositor:**  add basic Compositor type ([fa3ff5a2](https://github.com/lidavidm/voodoo/commit/fa3ff5a2070b055746882bcf4bf8d33055e28ccd))
* **Mouse:**  add basic mouse event support ([015f3a71](https://github.com/lidavidm/voodoo/commit/015f3a7145aab25a726f0acf841589360c3987c8))
* **Terminal:**
  *  add global clear functions ([304c0bab](https://github.com/lidavidm/voodoo/commit/304c0bab7f836442371cf671cc24b1441114e69f))
  *  reset color and clear on exit ([315ebff7](https://github.com/lidavidm/voodoo/commit/315ebff754e4608a1465da273d21a988154c423c))
  *  simplify clear_color ([dcaafc7f](https://github.com/lidavidm/voodoo/commit/dcaafc7f3439ee820db7349ba2c48cb6dfc3f9e5))
  *  clear terminal with color ([f16fbd7a](https://github.com/lidavidm/voodoo/commit/f16fbd7a2655b6838118a29bd8513c18683432be))
  *  add clear function ([62f38caa](https://github.com/lidavidm/voodoo/commit/62f38caaaacfc516d5e4277b4e3f61e4542206a0))
* **Window:**
  *  use new compositor ([813f3651](https://github.com/lidavidm/voodoo/commit/813f36515065bcdce80c75d5d289480f952c186d))
  *  add clear method ([855245ae](https://github.com/lidavidm/voodoo/commit/855245ae217132e66d68e8899bbf891ba1ca3846))
  *  print strings with colors ([9ac6ba98](https://github.com/lidavidm/voodoo/commit/9ac6ba987bdc1a0877fb34dd3a0958b440bc288e))
  *  add print_at ([10d8b987](https://github.com/lidavidm/voodoo/commit/10d8b98793d51bca58d712df700ceba237918239))
  *  reset color before updating each cell ([96413994](https://github.com/lidavidm/voodoo/commit/96413994dbe6b6ddf7d0aaeab4f990d50e70cc18))
  *  make cell color public ([2ac1325e](https://github.com/lidavidm/voodoo/commit/2ac1325e8120f0406628057cbdc1f1a9da250a56))
  *  implement cell colors ([26d78750](https://github.com/lidavidm/voodoo/commit/26d7875050e9771e752152b5ef64a490618b2984))
  *  basic character attributes ([9eb27a4b](https://github.com/lidavidm/voodoo/commit/9eb27a4bb38faada84fbac19c388bceb610ffde3))
  *  use mvwaddch for put_at ([c35df262](https://github.com/lidavidm/voodoo/commit/c35df26204af6dc925919d6fc16aaded3e1ba659))
  *  add put_at ([71942088](https://github.com/lidavidm/voodoo/commit/719420886bfb41f2924190eccfe7d310a7f24452))

#### Bug Fixes

* **Window:**  don't convert out-of-bounds coordinates ([4c29281a](https://github.com/lidavidm/voodoo/commit/4c29281a8755777a3df72a866267134decafa3bf))
