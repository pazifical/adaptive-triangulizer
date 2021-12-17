# <u>AdapTri</u> - Adaptive Triangulizer
[![Build Status](https://app.travis-ci.com/TwoWaySix/adaptive-triangulizer.svg?branch=main)](https://app.travis-ci.com/TwoWaySix/adaptive-triangulizer)
[![codecov](https://codecov.io/gh/TwoWaySix/adaptive-triangulizer/branch/main/graph/badge.svg?token=2MZCDM1QPQ)](https://codecov.io/gh/TwoWaySix/adaptive-triangulizer)
[![GitHub release](https://img.shields.io/github/release/TwoWaySix/adaptive-triangulizer.svg)](https://GitHub.com/TwoWaySix/adaptive-triangulizer/releases/)



**AdapTri** is a command line tool to convert images to ***pretty triangularized versions of themselves***. It is written in Rust 🦀 and can be either easily compiled 🔧 or downloaded as a release binary 📩. 

Using mesh generation techniques from the scientific field of ***computational fluid dynamics*** 🌊, the underlying details of the input image can be preserved and result in a high quality triangular representation 🖼️.

<img src="https://raw.githubusercontent.com/TwoWaySix/adaptive-triangulizer/main/data/2017_China_Chongqing_Boats.jpg" 
   style="width: 400px; height: auto; margin: auto">
<img src="https://raw.githubusercontent.com/TwoWaySix/adaptive-triangulizer/main/data/out/2017_China_Chongqing_Boats.jpg" 
   style="width: 400px; height: auto; margin: auto">
   
   
### 🤔 Yet another image triangulizer?

You may have seen an image like the second one, right? There are many open and closed source solutions to convert a regular image to one that consists of single colored triangles. Some of them create pretty ones, some of them not. But you might have not seen one where the ***triangle size adapts to the underlying structure*** 🏗️ of the input image. This is achieved by an ***iterative mesh refinement strategy*** 🕷️.


### 🕸️ What is a Mesh?

A Mesh consists of **vertices**, **edges** and **faces**. In the case of a **triangular Mesh**, each face is a triangle. 

Wait... what? That sounds rather complicated... but it actually is not that technical: Just imagine connecting a bunch of dots to triangles and you have a triangular mesh 👍. The goal is that those triangles are rather equilateral.

The most widely used technique to create a Mesh is the **Delaunay Triangulation**. Given a set of points (=vertices) and optionally segments, a triangular Mesh is constructed. It generates quality Meshes by adhering to a rule that enforces a maximization of each triangle's angles. For more information on the topic, please just read the Wikipedia article 📖.


### ♻️ How does the iterative refinement work?

After the initial triangulation of the image, each triangle's **centroid** 🎯 is closer inspected: The interpolated color is compared to the color of the underlying original image. If the difference is greater than a given threshold, a new vertex with the underlying color will be inserted at that position. After all triangles are inspected, the new set of vertices is triangulated again 🕸️.


### 📝 What's left to do:

- [x] Implement command line parsing
- [ ] Improve input error handling

<br><br>

## 🔉 How to compile and/or run the program

If you have Rust (and Cargo) installed, the easiest way is to run the following code in the top directory:
```
cargo build --release
```
This will result in the compilation of the program. After it's finished, an executable will be created in the /target/release directory. 

To see the possible command line arguments, just type
```
adaptri --help
```
