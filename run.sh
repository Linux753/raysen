file_name=img_$(date +"%Y_%m_%d_%T").ppm
cargo run --release > $file_name
echo "File saved as $file_name"