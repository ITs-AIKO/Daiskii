
# Daiskii

**Daiskii is a simple ASCII art Rust app. It's not the fastest, doesn't have the cleanest code, yet it has the weirdest name**.


# Features

- Turns any image to ASCII ART using the command line.
- Works with both local paths and URLs.
- You can adjust the quality of the generated ASCII art.
- Generate it based on your own palette.
- Simple.

# Usage
## Dev Usage

```bash
cargo run "https://i.pinimg.com/originals/7c/f8/2c/7cf82cd4ab4b62161d822f779edccc98.jpg"
```


## Normal Usage

1 - Download the executable from [**here**](https://github.com/ITs-AIKO/Daiskii/releases/download/0.1/daiskii.exe).

2 - Start a cmd in the same folder as the executable you just downloaded.

3 - Run the command:

```bash
daiskii "https://i.pinimg.com/originals/7c/f8/2c/7cf82cd4ab4b62161d822f779edccc98.jpg"
```

4 - You may need to zoom out using `ctrl`+`-` (minus sign) or `ctrl`+`scroll wheel`.

## Envirement variable

1 - Download the executable from [**here**](https://github.com/ITs-AIKO/Daiskii/releases/download/0.1/daiskii.exe).

2 - Move the executable file to a specific directory.

3 - Set that directory as a Path environment variable.

4 - Open the command line in any directory and run the command:

```bash
daiskii "https://i.pinimg.com/originals/7c/f8/2c/7cf82cd4ab4b62161d822f779edccc98.jpg"
```

5 - You may need to zoom out using `ctrl`+`-` (minus sign) or `ctrl`+`scroll wheel`.

## Result

[The Original Image: ](https://i.pinimg.com/originals/7c/f8/2c/7cf82cd4ab4b62161d822f779edccc98.jpg) `https://i.pinimg.com/originals/7c/f8/2c/7cf82cd4ab4b62161d822f779edccc98.jpg`

![Preview.](https://i.imgur.com/gFtmbO0.png)

![Preview.](https://i.imgur.com/TXMCxrk.png)

## Reference

```bash
cargo run [OPTIONS] <URI>
```

```bash
daiskii [OPTIONS] <URI>
```

# Arguements
- **URI**: A URL or a local Path.
# Options
- **Quality** `-q` : The quality of the generated ASCII art, the default value is `100 characters`, it can be:
    - `very_low` 50 characters.
    - `low` 200 characters.
    - `medium` 300 characters.
    - `high` 500 characters.
    - `very_high` 700 characters.
- **Force Black** `-f` : The generated ASCII will use black instead of gray (Zoom out before using this command for better results).
- **Palette** `-p` : It takes a list of characters (Default @#S%?+;:,.), and the generated ASCII art will be based on the provided characters (Zoom out before using this command for better results). You can even use emojis like '😌😊😒' BUT DO IT AT YOUR OWN RISK.

## Note

This README.md file took longer to write than the app itself.


## License

[MIT](https://choosealicense.com/licenses/mit/) License.


