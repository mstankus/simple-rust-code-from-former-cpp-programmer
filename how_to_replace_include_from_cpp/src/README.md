The idea here is to replace the following C++ code with rust code. The example is simple, but is meant to be used as a stepping stone.

```C++
// File: main.cpp
extern void ace();
int main() { ace();}

// File: a.cpp
extern void bar();
void ace() { bar();}

// File: b.cpp
#include <iostream>
void bar() { std::cout << "bar\n";}
```
or
```C++
// Not adding include guards to make the code more clear.

// File: main.cpp
#include "a.hpp"
int main() { ace();}

// File: a.hpp
extern void ace();

// File: a.cpp
#include "b.hpp"
void ace() { bar();}

// File: b.hpp
extern void bar();

// File: b.cpp
#include <iostream>
void bar() { std::cout << "bar\n";}
```
