
# Daiskii

**Daiskii is a simple ASCII art Rust app. It's not the fastest, doesn't have the cleanest code, yet it has the weirdest name**.


## Features

- Turns any image to ASCII ART using the command line.
- Works with both local paths and URLs.
- You can adjust the quality of the generated ASCII art.
- Simple.


## Download

Download the app from [**here**](https://github.com/ITs-AIKO/Daiskii/raw/main/assets/daiskii.exe).


## Usage (Normal usage)

1 - Start a cmd in the same folder as the executable you just downloaded.

2 - Run the command:

```bash
daiskii "https://i.pinimg.com/originals/7c/f8/2c/7cf82cd4ab4b62161d822f779edccc98.jpg" med
```

3 - You may need to zoom out using `ctrl`+`-` (minus sign) or `ctrl`+`scroll wheel`.

## Usage (Envirement variable)

1 - Move the executable file to a specific directory.

2 - Open the command line in any directory and run the command:

```bash
daiskii "https://i.pinimg.com/originals/7c/f8/2c/7cf82cd4ab4b62161d822f779edccc98.jpg" med
```

3 - You may need to zoom out using `ctrl`+`-` (minus sign) or `ctrl`+`scroll wheel`.

## Result

![Preview.](https://i.imgur.com/gFtmbO0.png)

## Reference

```bash
daiskii Image Quality
```

- **Image**: A URL or a local Path.
- **Quality**: The quality of the generated ASCII art, the default value is `100 characters by 100 characters`, it can be:
    - `very_low` 50 characters by 50 characters.
    - `low` 200 characters by 200 characters.
    - `medium` 300 characters by 300 characters.
    - `high` 500 characters by 500 characters.
    - `very_high` 700 characters by 700 characters.


## Note

This README.md file took longer to write than the app itself.


## License

[MIT](https://choosealicense.com/licenses/mit/) License.

