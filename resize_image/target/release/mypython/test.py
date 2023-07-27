import my_image_resize_crate
import time

input_image = [
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0]
    ]
t1 = time.perf_counter()
processed_image = my_image_resize_crate.process_image(input_image)
t2 = time.perf_counter()
print(processed_image)
print(f"Took {(t2-t1)*1000}ms")
