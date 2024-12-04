<p align="center">
<img src="https://github.com/user-attachments/assets/6d132c3c-f817-4a12-9c37-4fa596baa267" width="600px">
</p>

## How to run:

Grab the .exe or the msi installer from the [releases](https://github.com/chrisheib/temocr/releases).

You will need to disregard the Microsoft Defender notice (clicking more information -> continue anyway) because I am not rich enough to afford a Microsoft Code Signing Certificate. Sad dev noises.

## Compile yourself:

clone git repo, install rust, run `cargo r -r`

## ui:
first run:
`cargo install tauri-cli --version '^2.0.0' --locked`

For Desktop development, run:
`cargo tauri dev`

## Kudos:

rten networks from https://github.com/robertknight/ocrs/blob/main/ocrs/examples/download-models.sh

```
DETECTION_MODEL="https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten"
RECOGNITION_MODEL="https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten"
```

temtem jsons from https://github.com/maael/temtem-api/blob/master/data

