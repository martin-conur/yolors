# YOLOrs 🚧
YOLO v8 CLI implementation using [Candle](https://github.com/huggingface/candle/tree/main).

!["people classified by yolors](https://github.com/martin-conur/yolors/blob/main/demo_images/people.classification.jpg)

## Description
YOLOrs is a CLI tool for image classification, pose estimation or (in the future) image segmentation. Uses [ultralytics](https://yolov8.com) implementation (v8) of the YOLO (You Only Look Once) algorithms family. 

YOLOrs is written in Rust, using HuggingFace's Candle library. Is Fast, Light, Secure and reliable.


## Installation 🚧
You can install it with cargo:
```console
  cargo install yolors
```

Or alternatively, you can clone this repo:
```console
 git clone https://github.com/martin-conur/yolors.git
```

and run one of the examples below eg. 
 ```console
yolors demo_images/people.jpg 
```

## Examples
You can ask for classification: 

``` console
yolors demo_images/people.jpg    
```
!["result classification"](https://github.com/martin-conur/yolors/blob/main/demo_images/running2.pp.jpg)


or Pose estimation:

 ```console
 yolors  --task "pose" demo_images/running1.jpg
 ```
 
 !["result pose"](https://github.com/martin-conur/yolors/blob/main/demo_images/running1.pose.jpg)

 ## Commands
 ```console
 Arguments:
  <IMAGES>... 
    [REQUIRED]

Options:
      --cpu
          Run on CPU rather than on GPU
      --tracing
          Enable tracing (generates a trace-timestamp.json file)
      --model <MODEL>
          Model weights, in safetensors format
      --which <WHICH>
          Which model variant to use [default: s] [possible values: n, s, m, l, x]
      --confidence-threshold <CONFIDENCE_THRESHOLD>
          Threshold for the model confidence level [default: 0.25]
      --nms-threshold <NMS_THRESHOLD>
          Threshold for non-maximum suppression [default: 0.45]
      --task <TASK>
          The task to be run [default: detect] [possible values: detect, pose]
      --legend-size <LEGEND_SIZE>
          The size for the legend, 0 means no legend [default: 14]
  -h, --help
          Print help
  -V, --version
          Print version
 ```

## features:
  * blazingly fast
  * tiny (6 MB)
  * easy to use
  * Secure
  * Reliable
  * Written in Rust 🤘

## TODO:
 * ~~Add CI github workflow~~
 * ~~Add JSON output ~~capabilities 50% done
 * ~~Add cargo tests~~
 * Cross compilation and distribution
 * Add segmentation
 * Add video capabilities
