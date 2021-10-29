<!-- PROJECT LOGO -->
<br />
<p align="center">
  <h3 align="center">Learning to Rank ++</h3>

  <p align="center">
    Small Learning to Rank library based on RankLib
    <br />
    <a href="https://ltr.readthedocs.io/"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/marcosfpr/ltrpp/issues">Report Bug</a>
    ·
    <a href="https://github.com/marcosfpr/ltrpp/issues">Request Feature</a>
  </p>
</p>

<!-- ABOUT THE PROJECT -->
## About The Project

LTR++ is a Learning to Rank library written in C++ and based on the famous [RankLib](https://sourceforge.net/p/lemur/wiki/RankLib%20How%20to%20use/) library. The main goal of this project is to provide a simple, fast and memory safe Learning to Rank library which implements a wide variety of LTR models. 

### Built With

The development of ltr++ are using almost only C++17 language features. Additionally, some external libraries were used, as shown below: 

* [Boost Libraries](https://www.boost.org): Boost is a set of libraries for the C++ programming language that provides support for tasks and structures such as linear algebra and regular expressions.
* [Google Test](https://github.com/google/googletest): Google Testing and Mocking Framework.
* [Spdlog](https://github.com/gabime/spdlog): Fast C++ logging library. 
* [CMake](https://cmake.org): Tools designed to build, test and package software.

<!-- GETTING STARTED -->
## Getting Started

Initially, we will understand all prerequisites and compatibilities of ltr++ and also how to install the project. To get a local copy up and running, follow these simple example steps. 

### Prerequisites

The main prerequisite for installing ltr++ is CMake (> 3.15). Other external libraries will be downloaded automatically, if you don't have they. So that make sure you have CMake installed.

Also, make sure that your operating system is compatible with any build that is working: 


|       System       |                                                        1.0.0                                                |
|:------------------:|:-----------------------------------------------------------------------------------------------------------:|
|     Windows x86    |  [![ltrpp-w64](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/marcosfpr/ltrpp) |
|     Windows x64    |  [![ltrpp-w64](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/marcosfpr/ltrpp) |
|     MacOSX x64     |  [![ltrpp-w64](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/marcosfpr/ltrpp) |
| Linux (ubuntu) x64 |  [![ltrpp-w64](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/marcosfpr/ltrpp) |


### Integration

Using CMake and certifying that all prerequisites are ok, let's understand how to build and install ltr.

### 1. C++

##### Embed as header-only

Copy the files from the `source` directory of this project to your `include` directory.

  ```c++
  #include <ltr.hpp>
  ```

[**Warning**] Make sure you have C++17 (>) installed

##### Embed as CMake subdirectory

You can use ltr++ directly in CMake projects as a subproject.

Clone the whole project inside your own project:

```bash
git clone https://github.com/marcosfpr/ltrpp/
```

and add the subdirectory to your CMake script:

```cmake
add_subdirectory(ltrpp)
```

When creating your executable, link the library to the targets you want:

```cmake
add_executable(my_target main.cpp)
target_link_libraries(my_target PRIVATE ltr)
```

Your target will be able to see the ltr++ headers now.

##### Embed with CPM.cmake

[CPM.cmake](https://github.com/TheLartians/CPM.cmake) is a nice wrapper around the CMake FetchContent function.
Install [CPM.cmake](https://github.com/TheLartians/CPM.cmake) and then use this command to add ltr++ to your build
script:

```cmake
CPMAddPackage(
        NAME ltr
        GITHUB_REPOSITORY marcosfpr/ltrpp
        GIT_TAG origin/master # or whatever tag you want
)
# ...
target_link_libraries(my_target PUBLIC ltr)
```

Your target will be able to see the ltr++ headers now.

##### Find as CMake package

If you are using CMake and have the library installed on your system, you can then find ltr++ with the
usual `find_package` command:

```cmake
find_package(ltr REQUIRED)
# ...
target_link_libraries(my_target PUBLIC ltr)
```

Your target will be able to see the ltr++ headers now.

[**Warning**] "find_package on windows"
There is no easy default directory for find_package on windows. You have
to [set it](https://stackoverflow.com/questions/21314893/what-is-the-default-search-path-for-find-package-in-windows-using-cmake)
yourself.



<!-- USAGE EXAMPLES -->
## Usage

In this section, I'll show you a toy example of ltr++. Don't get stuck with this example: see another examples and the official documentation! 

The main goal in this case is to import the `OHSUMED` training file, fit a LTR model using AdaRank and save parameters:

* Loss: P@2 (Precision)
* Iterations: 50
* Tolerance: 0.003
* Consecutive Selections: 3

```c++
#include <ltr.hpp>

// .. other imports

using namespace ltr;

int main() {
    string base_path = std::getenv("OHSUMED");

    DataSet training_samples = ltr::load_svmlight(base_path + "/Data/All/OHSUMED.txt");

    AdaRank ranker(training_samples, std::make_unique<PrecisionScorer>(2), 50, 0.003, 3);

    ranker.fit();

    return 0;
}

```


<!-- ROADMAP -->
## Roadmap

The project is in the early stages of development. Thus, feel free to contribute and help ltr++ to grow up!

See the [open issues](https://github.com/marcosfpr/ltrpp/issues) for a list of proposed features (and known issues).

**OBS**: To propose new features or report bugs, check out the correct templates.

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<!-- CONTACT -->
## Contact

Marcos Pontes - mfprezende@gmail.com
