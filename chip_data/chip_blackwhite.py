from PIL import Image
import numpy as np

from PIL import Image
import numpy as np

def image_to_ascii(image_path, max_size=(80, 80)):
    img = Image.open(image_path)
    img.thumbnail(max_size)  # scale it down
    img = img.convert('L')  # convert image to grayscale

    WIDTH, HEIGHT = img.size
    ascii_str = ''

    ascii_chars = '@%#*+=-:. '  # define the ASCII characters to use

    for y in range(0, HEIGHT, 2):  # we skip a line each time to correct the aspect ratio
        for x in range(WIDTH):
            brightness = np.mean(img.getpixel((x, y)))  # get the brightness of the pixel
            ascii_index = brightness // (255 // (len(ascii_chars) - 1))  # scale brightness to the index of ascii_chars
            ascii_str += ascii_chars[int(ascii_index)]
        ascii_str += '\n'
    return ascii_str


def main():
    images_path = "./frames/"
    # load images from path file are called 0001 to 0008
    for i in range(1, 9):
        ascii = image_to_ascii(images_path + "000" + str(i) + ".png")
        # print ascii with colored background
        print('\033[48;5;0m' + ascii + '\033[0m')  # ANSI escape code to print with colored background
        with open('chip_black_and_white.rs', 'a') as f:
            f.write(f"pub const ASCII_ART_{i}: &str = r##\"{ascii}\"##;\n")
main()