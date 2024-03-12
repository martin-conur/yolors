# YOLOrs 🚧
YOLO v8 CLI implementation using [Candle](https://github.com/huggingface/candle/tree/main).

!["people classified by yolors](https://github.com/martin-conur/yolors/blob/main/demo_images/people.classification.jpg)

## Description


## Example
You can ask for classification: 


or Pose estimation:

 '''console
 cargo run --release --  --task "pose" demo_images/running1.jpg
 '''
 
 result:
 !["result1"](https://github.com/martin-conur/yolors/blob/main/demo_images/running1.pose.jpg)

## features:
  * blazingly fast
  * easy to use
  * Written in Rust 🤘

## TODO:
 * Add segmentation
 * Add video capabilities
