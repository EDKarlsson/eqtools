from PIL import Image
import os

project_root = os.path.dirname(os.path.dirname(__file__))
print(f"Proj directory: {project_root}")

img_dir = f"{project_root}/img"
gem_images = f"{img_dir}/gems"
spells_images = f"{img_dir}/spells"
