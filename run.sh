file_name=img_$(date +"%Y_%m_%d_%T")
cargo run --release > "$file_name.ppm"
magick $file_name.ppm $file_name.png
rm $file_name.ppm
echo "File saved as $file_name.png"
