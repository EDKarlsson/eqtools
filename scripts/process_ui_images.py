from PIL import Image
import sys
import os

project_root = os.path.dirname(os.path.dirname(__file__))
print(f"Proj directory: {project_root}")

img_dir = f"{project_root}/img"
grouped_dir = f"{img_dir}/grouped"
individual_dir = f"{img_dir}/individual"

PIXELS = 40


def split_image(im: Image.Image):
    regions: list[Image.Image] = []
    for y in range(0, 6):
        for x in range(0, 6):
            box = (x * PIXELS, y * PIXELS, (x + 1) * PIXELS, (y + 1) * PIXELS)
            regions.append(im.crop(box))
    return regions


def process_all_files():
    # Gem dimensions are different so need to add functionality to handle that
    image_types = ["spells", "items"]
    for img_type in image_types:
        _img_dir = f"{grouped_dir}/{img_type}"
        for i, img in enumerate(os.listdir(_img_dir)):
            outdir = f"{individual_dir}/{img_type}/group{i}"
            if not os.path.isdir(outdir):
                os.mkdir(outdir)
            process_one(f"{_img_dir}/{img}", img_type[:-1], outdir)


def process_one(image_file_name: str, file_prefix: str, outdir: str):
    im = Image.open(image_file_name)
    images = split_image(im)
    for i, image in enumerate(images):
        image.save(f"{outdir}/{file_prefix}_{i}.png")


def example():
    file_group = f"{individual_dir}/spells/group01"
    if not os.path.isdir(file_group):
        os.mkdir(file_group)
    process_one(f"{grouped_dir}/spells/Spells01.png", "spell", file_group)


if __name__ == "__main__":
    process_all_files()
