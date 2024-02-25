# image_base_colors

```bash
curl -F image_name=@path_to_image --verbose 'http://localhost:3000/info?number_of_clusters=4&max_try_count=30' | json_pp
```

```bash
curl -F image_name=@path_to_image -o output.png --verbose 'http://localhost:3000/draw?number_of_clusters=4&max_try_count=30'
```
