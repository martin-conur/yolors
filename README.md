# YOLOrs 🚧
YOLO v8 CLI implementation using [Candle](https://github.com/huggingface/candle/tree/main).

!["people classified by yolors](https://github.com/martin-conur/yolors/blob/main/demo_images/people.classification.jpg)

## Description
YOLOrs is a CLI tool for image classification, pose estimation or (in the future) image segmentation. Uses [ultralytics](https://yolov8.com) implementation (v8) of the YOLO (You Only Look Once) algorithms family. 

YOLOrs is written in Rust, using HuggingFace's Candle library. Is Fast, Light, Secure and reliable.


## Installation
🚧

## Example
You can ask for classification: 

``` console
cargo run --release demo_images/people.jpg    
```
!["result classification"](https://github.com/martin-conur/yolors/blob/main/demo_images/running2.pp.jpg)


or Pose estimation:

 ```console
 cargo run --release --  --task "pose" demo_images/running1.jpg
 ```
 
 !["result pose"](https://github.com/martin-conur/yolors/blob/main/demo_images/running1.pose.jpg)

## features:
  * blazingly fast
  * easy to use
  * Secure
  * Reliable
  * Written in Rust 🤘

## TODO:
 * Cross compilation and distribution
 * Add segmentation
 * Add video capabilities
